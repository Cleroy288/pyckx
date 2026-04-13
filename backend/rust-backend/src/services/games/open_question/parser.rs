//! Open Question parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::OpenQuestionAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::QuestionId;

use super::domain::OpenQuestion;

/// Parse AI-generated open question response into domain objects
pub fn parse_open_question_response(
    response: &str,
) -> Result<Vec<OpenQuestion>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: OpenQuestionAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        StudyError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as open questions: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    let questions: Vec<OpenQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| OpenQuestion {
            id: QuestionId::new(),
            question: q.question,
            user_answer: String::new(), // Empty for newly generated questions
            expected_answer: Some(q.expected_answer),
            hint: Some(q.hint), // Wrap in Some for domain type
        })
        .collect();

    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_open_question_valid_returns_questions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "question": "Explain ownership",
                "expected_answer": "Ownership is...",
                "hint": "Think about memory"
            }]
        })
        .to_string();

        // act
        let qs =
            parse_open_question_response(&json).unwrap();

        // assert
        assert_eq!(qs.len(), 1);
        assert_eq!(qs[0].question, "Explain ownership");
        assert!(qs[0].user_answer.is_empty());
    }

    #[test]
    fn test_parse_open_question_invalid_json_returns_error() {
        // arrange / act
        let result = parse_open_question_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_open_question_empty_returns_empty() {
        // arrange
        let json = r#"{"questions": []}"#;

        // act
        let result =
            parse_open_question_response(json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_open_question_sets_expected_and_hint() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "question": "Q?",
                "expected_answer": "EA",
                "hint": "H"
            }]
        })
        .to_string();

        // act
        let qs = parse_open_question_response(&json).unwrap();

        // assert
        assert_eq!(qs[0].expected_answer, Some("EA".into()));
        assert_eq!(qs[0].hint, Some("H".into()));
    }

    #[test]
    fn test_parse_open_question_missing_hint_returns_error() {
        // arrange
        let json = r#"{"questions": [{
            "question": "Q?",
            "expected_answer": "A"
        }]}"#;

        // act
        let result = parse_open_question_response(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_open_question_extra_text_before_json_returns_ok()
    {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "question": "Q?",
                "expected_answer": "A",
                "hint": "H"
            }]
        })
        .to_string();
        let input = format!("Output:\n{}", json);

        // act / assert
        parse_open_question_response(&input).unwrap();
    }

    #[test]
    fn test_parse_open_question_multiple_questions_returns_all() {
        // arrange
        let json = serde_json::json!({
            "questions": [
                {"question": "Q1?",
                    "expected_answer": "A1", "hint": "H1"},
                {"question": "Q2?",
                    "expected_answer": "A2", "hint": "H2"},
                {"question": "Q3?",
                    "expected_answer": "A3", "hint": "H3"}
            ]
        })
        .to_string();

        // act
        let qs =
            parse_open_question_response(&json).unwrap();

        // assert
        assert_eq!(qs.len(), 3);
    }
}

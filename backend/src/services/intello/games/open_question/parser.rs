//! Open Question parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::OpenQuestionAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::QuestionId;

use super::domain::OpenQuestion;

// ** parse_open_question_response **
// ==> Parses AI-generated open question response into domain objects
//
// @ response : Raw AI response string containing JSON
// @ returns : Vector of parsed OpenQuestion objects
// @ errors : ValidationFailed if JSON parsing fails
pub fn parse_open_question_response(
    response: &str,
) -> Result<Vec<OpenQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: OpenQuestionAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as open questions: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    // Step 3: Transform AI schema to domain objects with unique IDs
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

    // Step 4: Return parsed open questions
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
        let result = parse_open_question_response(&json);

        // assert
        assert!(result.is_ok());
        let qs = result.unwrap();
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
        let result = parse_open_question_response(json);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
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
}

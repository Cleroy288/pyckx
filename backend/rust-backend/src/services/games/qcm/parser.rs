//! QCM parser - converts AI responses to domain types

use crate::infra::openrouter::extract_json_from_response;
use crate::services::ai_response::QcmAiResponse;
use crate::services::error_domain::StudyError;
use crate::services::QuestionId;

use super::domain::QcmQuestion;

/// Parse AI-generated QCM response into domain objects
pub fn parse_qcm_response(
    response: &str,
) -> Result<Vec<QcmQuestion>, StudyError> {
    let json_str = extract_json_from_response(response);

    let ai_response: QcmAiResponse =
        serde_json::from_str(&json_str).map_err(|err| {
            StudyError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as QCM: {}. Response was: {}",
                    err, json_str
                ),
            )
        })?;

    let questions: Vec<QcmQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| QcmQuestion {
            id: QuestionId::new(),
            question: q.question,
            wrong_answers: q.wrong_answers,
            right_answer: q.right_answer,
            explanation: q.explanation,
        })
        .collect();

    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qcm_valid_json_returns_questions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "question": "What is 1+1?",
                "wrong_answers": ["1", "3", "4"],
                "right_answer": "2",
                "explanation": "Basic math"
            }]
        })
        .to_string();

        // act
        let questions = parse_qcm_response(&json).unwrap();

        // assert
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].question, "What is 1+1?");
    }

    #[test]
    fn test_parse_qcm_invalid_json_returns_error() {
        // arrange / act
        let result = parse_qcm_response("not json");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_qcm_empty_questions_returns_empty_vec() {
        // arrange
        let json = r#"{"questions": []}"#;

        // act
        let result = parse_qcm_response(json).unwrap();

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_qcm_markdown_wrapped_returns_ok() {
        // arrange
        let json = format!(
            "```json\n{}\n```",
            serde_json::json!({
                "questions": [{
                    "question": "Q?",
                    "wrong_answers": ["A", "B", "C"],
                    "right_answer": "D",
                    "explanation": "E"
                }]
            })
        );

        // act / assert
        parse_qcm_response(&json).unwrap();
    }

    #[test]
    fn test_parse_qcm_assigns_unique_ids() {
        // arrange
        let json = serde_json::json!({
            "questions": [
                {
                    "question": "Q1?",
                    "wrong_answers": ["A", "B", "C"],
                    "right_answer": "D",
                    "explanation": "E1"
                },
                {
                    "question": "Q2?",
                    "wrong_answers": ["X", "Y", "Z"],
                    "right_answer": "W",
                    "explanation": "E2"
                }
            ]
        })
        .to_string();

        // act
        let questions = parse_qcm_response(&json).unwrap();

        // assert
        assert_ne!(questions[0].id, questions[1].id);
    }

    #[test]
    fn test_parse_qcm_missing_field_returns_error() {
        // arrange
        let json = r#"{"questions": [{
            "question": "Q?",
            "wrong_answers": ["A","B","C"],
            "right_answer": "D"
        }]}"#;

        // act
        let result = parse_qcm_response(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_qcm_extra_text_before_json_returns_ok() {
        // arrange
        let json = format!(
            "Here is the answer:\n{}",
            serde_json::json!({
                "questions": [{
                    "question": "Q?",
                    "wrong_answers": ["A", "B", "C"],
                    "right_answer": "D",
                    "explanation": "E"
                }]
            })
        );

        // act / assert
        parse_qcm_response(&json).unwrap();
    }

    #[test]
    fn test_parse_qcm_multiple_questions_returns_all() {
        // arrange
        let json = serde_json::json!({
            "questions": [
                {
                    "question": "Q1?",
                    "wrong_answers": ["A","B","C"],
                    "right_answer": "D",
                    "explanation": "E1"
                },
                {
                    "question": "Q2?",
                    "wrong_answers": ["A","B","C"],
                    "right_answer": "D",
                    "explanation": "E2"
                },
                {
                    "question": "Q3?",
                    "wrong_answers": ["A","B","C"],
                    "right_answer": "D",
                    "explanation": "E3"
                }
            ]
        })
        .to_string();

        // act
        let result = parse_qcm_response(&json).unwrap();

        // assert
        assert_eq!(result.len(), 3);
    }
}

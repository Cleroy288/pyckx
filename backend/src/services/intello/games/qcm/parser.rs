//! QCM parser - converts AI responses to domain types

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::QcmAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::QuestionId;

use super::domain::QcmQuestion;

// ** parse_qcm_response **
// ==> Parses AI-generated QCM response into domain question objects
//
// @ response : Raw AI response string containing JSON
// @ returns : Vector of parsed QcmQuestion objects
// @ errors : ValidationFailed if JSON parsing fails
pub fn parse_qcm_response(
    response: &str,
) -> Result<Vec<QcmQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response (removes markdown, etc.)
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: QcmAiResponse =
        serde_json::from_str(&json_str).map_err(|err| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as QCM: {}. Response was: {}",
                    err, json_str
                ),
            )
        })?;

    // Step 3: Transform AI schema to domain objects with unique IDs
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

    // Step 4: Return parsed questions
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
        let result = parse_qcm_response(&json);

        // assert
        assert!(result.is_ok());
        let questions = result.unwrap();
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
        let result = parse_qcm_response(json);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
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

        // act
        let result = parse_qcm_response(&json);

        // assert
        assert!(result.is_ok());
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
}

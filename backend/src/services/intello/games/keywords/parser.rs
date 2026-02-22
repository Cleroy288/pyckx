//! Keywords parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::KeywordsAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::{OptionId, QuestionId};

use super::domain::{Keyword, KeywordQuestion};

// ** parse_keywords_response **
// ==> Parses AI-generated keywords response into domain question objects
//
// @ response : Raw AI response string containing JSON
// @ returns : Vector of parsed KeywordQuestion objects
// @ errors : ValidationFailed if JSON parsing fails
pub fn parse_keywords_response(
    response: &str,
) -> Result<Vec<KeywordQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: KeywordsAiResponse = serde_json::from_str(&json_str).map_err(|err| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as keywords questions: {}. Response was: {}",
                err, json_str
            ),
        )
    })?;

    // Step 3: Transform AI schema to domain objects with unique IDs
    let questions: Vec<KeywordQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| KeywordQuestion {
            id: QuestionId::new(),
            statement: q.statement,
            keywords: q
                .keywords
                .into_iter()
                .map(|k| Keyword {
                    id: OptionId::new(),
                    word: k.word,
                    is_correct: k.is_correct,
                })
                .collect(),
            explanation: q.explanation,
        })
        .collect();

    // Step 4: Return parsed keyword questions
    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keywords_valid_returns_questions() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "statement": "Rust memory management",
                "keywords": [
                    {"word": "ownership", "is_correct": true},
                    {"word": "garbage", "is_correct": false}
                ],
                "explanation": "Rust uses ownership"
            }]
        })
        .to_string();

        // act
        let result = parse_keywords_response(&json);

        // assert
        assert!(result.is_ok());
        let questions = result.unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(
            questions[0].statement,
            "Rust memory management"
        );
    }

    #[test]
    fn test_parse_keywords_invalid_json_returns_error() {
        // arrange / act
        let result = parse_keywords_response("bad");

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_keywords_empty_returns_empty() {
        // arrange
        let json = r#"{"questions": []}"#;

        // act
        let result = parse_keywords_response(json);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_parse_keywords_assigns_unique_ids() {
        // arrange
        let json = serde_json::json!({
            "questions": [{
                "statement": "Test",
                "keywords": [
                    {"word": "A", "is_correct": true},
                    {"word": "B", "is_correct": false}
                ],
                "explanation": "E"
            }]
        })
        .to_string();

        // act
        let qs = parse_keywords_response(&json).unwrap();
        let kws = &qs[0].keywords;

        // assert
        assert_ne!(kws[0].id, kws[1].id);
    }
}

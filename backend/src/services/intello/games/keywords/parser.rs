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
pub fn parse_keywords_response(response: &str) -> Result<Vec<KeywordQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: KeywordsAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as keywords questions: {}. Response was: {}",
                e, json_str
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

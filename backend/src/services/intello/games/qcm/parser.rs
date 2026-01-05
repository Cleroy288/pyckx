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
pub fn parse_qcm_response(response: &str) -> Result<Vec<QcmQuestion>, IntelloError> {
    // Step 1: Extract JSON from AI response (removes markdown, etc.)
    let json_str = extract_json_from_response(response);

    // Step 2: Deserialize JSON into AI response schema
    let ai_response: QcmAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as QCM: {}. Response was: {}",
                e, json_str
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

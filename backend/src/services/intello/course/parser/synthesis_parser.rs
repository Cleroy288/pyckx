use crate::infra::openrouter::{extract_json_from_response, sanitize_ai_json};
use crate::services::intello::course::domain::ParsedSynthesis;
use crate::services::intello::error_domain::IntelloError;

// ** parse_generated_synthesis **
// ==> Parses AI response JSON into ParsedSynthesis struct
//
// @ ai_response : Raw AI response (may contain markdown or extra text)
// @ returns : ParsedSynthesis if parsing succeeds
// @ errors : ValidationFailed if JSON is invalid or structure is wrong
pub fn parse_generated_synthesis(ai_response: &str) -> Result<ParsedSynthesis, IntelloError> {
    // Step 1: Extract JSON from AI response
    let extracted = extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    // Step 2: Parse JSON into struct
    let synthesis: ParsedSynthesis = serde_json::from_str(&sanitized)
        .map_err(|e| IntelloError::validation("synthesis_json", format!("Failed to parse synthesis JSON: {}", e)))?;

    // Step 2: Validate synthesis structure
    if synthesis.summary_text.is_empty() {
        return Err(IntelloError::validation("summary_text", "Summary text cannot be empty"));
    }

    if synthesis.key_takeaways.is_empty() {
        return Err(IntelloError::validation("key_takeaways", "Must have at least one key takeaway"));
    }

    // Step 3: Validate final QCM
    if synthesis.final_qcm.questions.is_empty() {
        return Err(IntelloError::validation("final_qcm", "Final QCM must have at least one question"));
    }

    // Validate each question structure
    for (idx, question) in synthesis.final_qcm.questions.iter().enumerate() {
        if question.question.is_empty() {
            return Err(IntelloError::validation("question_text", format!("Question {} text cannot be empty", idx + 1)));
        }

        if question.right_answer.is_empty() {
            return Err(IntelloError::validation("right_answer", format!("Question {} right answer cannot be empty", idx + 1)));
        }

        if question.wrong_answers.len() != 3 {
            return Err(IntelloError::validation("wrong_answers", format!("Question {} must have exactly 3 wrong answers", idx + 1)));
        }
    }

    // Step 4: Return validated synthesis
    Ok(synthesis)
}

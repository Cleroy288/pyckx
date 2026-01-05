use crate::infra::openrouter::{extract_json_from_response, sanitize_ai_json};
use crate::services::intello::course::domain::ParsedSection;
use crate::services::intello::error_domain::IntelloError;

// ** parse_generated_section **
// ==> Parses AI response JSON into ParsedSection struct
//
// @ ai_response : Raw AI response (may contain markdown or extra text)
// @ returns : ParsedSection if parsing succeeds
// @ errors : ValidationFailed if JSON is invalid or structure is wrong
pub fn parse_generated_section(ai_response: &str) -> Result<ParsedSection, IntelloError> {
    // Step 1: Extract JSON from AI response
    let extracted = extract_json_from_response(ai_response);
    let sanitized = sanitize_ai_json(&extracted);

    // Step 2: Parse JSON into struct
    let section: ParsedSection = serde_json::from_str(&sanitized)
        .map_err(|e| IntelloError::validation("section_json", format!("Failed to parse section JSON: {}", e)))?;

    // Step 3: Validate section structure
    if section.title.is_empty() {
        return Err(IntelloError::validation("title", "Section title cannot be empty"));
    }

    if section.content_blocks.is_empty() {
        return Err(IntelloError::validation("content_blocks", "Section must have at least one content block"));
    }

    // Step 4: Validate QCM set (now required)
    if section.qcm_set.questions.is_empty() {
        return Err(IntelloError::validation("qcm_questions", "QCM set must have at least one question"));
    }

    // Validate each question structure
    for (idx, question) in section.qcm_set.questions.iter().enumerate() {
        if question.question.is_empty() {
            return Err(IntelloError::validation("question_text", format!("Question {} text cannot be empty", idx + 1)));
        }

        if question.right_answer.is_empty() {
            return Err(IntelloError::validation("right_answer", format!("Question {} right answer cannot be empty", idx + 1)));
        }

        if question.wrong_answers.len() != 3 {
            return Err(IntelloError::validation("wrong_answers", format!("Question {} must have exactly 3 wrong answers", idx + 1)));
        }

        // Validate extensive explanation (at least 50 characters)
        if question.explanation.len() < 30 {
            return Err(IntelloError::validation("explanation", format!("Question {} explanation must be at least 30 characters for good learning", idx + 1)));
        }
    }

    // Step 5: Return validated section
    Ok(section)
}

//! AI Response Parsing Service
//!
//! Centralizes all parsing logic for AI-generated content.
//! Converts AI JSON responses into domain types.

use super::ai_response::*;
use super::error_domain::IntelloError;
use super::*;
use crate::infra::openrouter::extract_json_from_response;

// ============================================================
// QCM PARSING
// ============================================================

/// Parse QCM response from AI
pub fn parse_qcm_response(response: &str) -> Result<Vec<QcmQuestion>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: QcmAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as QCM: {}. Response was: {}",
                e, json_str
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

// ============================================================
// FLASHCARD PARSING
// ============================================================

/// Parse flashcard response from AI
pub fn parse_flashcard_response(response: &str) -> Result<Vec<Flashcard>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: FlashcardAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as flashcards: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    let flashcards: Vec<Flashcard> = ai_response
        .cards
        .into_iter()
        .map(|c| Flashcard {
            id: QuestionId::new(),
            front: c.front,
            back: c.back,
        })
        .collect();

    Ok(flashcards)
}

// ============================================================
// KEYWORDS PARSING
// ============================================================

/// Parse keywords response from AI
pub fn parse_keywords_response(response: &str) -> Result<Vec<KeywordQuestion>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: KeywordsAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as keywords questions: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

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

    Ok(questions)
}

// ============================================================
// FILL BLANK PARSING
// ============================================================

/// Parse fill blank response from AI
pub fn parse_fill_blank_response(response: &str) -> Result<Vec<FillBlankQuestion>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: FillBlankAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as fill blank questions: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    let questions: Vec<FillBlankQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| FillBlankQuestion {
            id: QuestionId::new(),
            phrase: q.phrase,
            options: q
                .options
                .into_iter()
                .map(|o| FillBlankOption {
                    id: OptionId::new(),
                    text: o.text,
                    is_correct: o.is_correct,
                })
                .collect(),
            explanation: q.explanation,
        })
        .collect();

    Ok(questions)
}

// ============================================================
// ORDER PHRASE PARSING
// ============================================================

/// Parse order phrase response from AI
pub fn parse_order_phrase_response(
    response: &str,
) -> Result<Vec<OrderPhraseQuestion>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: OrderPhraseAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as order phrase questions: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    let questions: Vec<OrderPhraseQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| OrderPhraseQuestion {
            id: QuestionId::new(),
            original_phrase: q.original_phrase,
            words: q
                .words
                .into_iter()
                .map(|w| OrderPhraseWord {
                    id: OptionId::new(),
                    word: w.word,
                    position: w.position,
                })
                .collect(),
            hint: q.hint,
        })
        .collect();

    Ok(questions)
}

// ============================================================
// TRUE/FALSE PARSING
// ============================================================

/// Parse true/false response from AI
pub fn parse_true_false_response(
    response: &str,
) -> Result<Vec<TrueOrFalseStatement>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: TrueOrFalseAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as true/false statements: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    let statements: Vec<TrueOrFalseStatement> = ai_response
        .statements
        .into_iter()
        .map(|s| TrueOrFalseStatement {
            id: QuestionId::new(),
            statement: s.statement,
            answer: s.answer,
            explanation: s.explanation,
        })
        .collect();

    Ok(statements)
}

// ============================================================
// OPEN QUESTION PARSING
// ============================================================

/// Parse open question response from AI
pub fn parse_open_question_response(response: &str) -> Result<Vec<OpenQuestion>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: OpenQuestionAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as open questions: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    let questions: Vec<OpenQuestion> = ai_response
        .questions
        .into_iter()
        .map(|q| OpenQuestion {
            id: QuestionId::new(),
            question: q.question,
            user_answer: String::new(),  // Empty string for newly generated questions
            expected_answer: Some(q.expected_answer),
            hint: Some(q.hint),  // Wrap in Some since domain expects Option<String>
        })
        .collect();

    Ok(questions)
}

// ============================================================
// VERIFICATION/GRADING PARSING
// ============================================================

/// Grade for a user's answer
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnswerGrade {
    Right,
    Medium,
    Error,
}

/// A graded answer with feedback
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GradedAnswer {
    pub question_id: String,
    pub grade: AnswerGrade,
    pub feedback: String,
}

/// Parse verification/grading response from AI
pub fn parse_verification_response(response: &str) -> Result<Vec<GradedAnswer>, IntelloError> {
    let json_str = extract_json_from_response(response);

    let ai_response: VerificationAiResponse = serde_json::from_str(&json_str).map_err(|e| {
        IntelloError::validation(
            "ai_response",
            format!(
                "Failed to parse AI response as verification: {}. Response was: {}",
                e, json_str
            ),
        )
    })?;

    let grades: Vec<GradedAnswer> = ai_response
        .grades
        .into_iter()
        .map(|g| {
            let grade = match g.grade.to_lowercase().as_str() {
                "right" => AnswerGrade::Right,
                "medium" => AnswerGrade::Medium,
                _ => AnswerGrade::Error,
            };
            GradedAnswer {
                question_id: g.question_id,
                grade,
                feedback: g.feedback,
            }
        })
        .collect();

    Ok(grades)
}

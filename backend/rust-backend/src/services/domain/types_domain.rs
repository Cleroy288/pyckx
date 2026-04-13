//! Study domain types - pure input/output types
//!
//! No framework dependencies. No I/O. No service logic.

use crate::services::Level;

/// List of documents: (filename, content, token_count)
pub type DocumentList = Vec<(String, String, u32)>;

/// Input parameters for AI content generation
/// (QCM, Flashcards, Open Questions, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateContentInput {
    /// Name of the generated set
    pub name: String,
    /// Description of the set's purpose
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language code (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subject tags (max 3)
    pub subjects: Vec<String>,
    /// Number of questions to generate
    pub num_questions: u8,
    /// Uploaded documents
    pub documents: DocumentList,
}

/// Input for checking/grading open question answers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckAnswersInput {
    /// The question set ID
    pub set_id: String,
    /// User's answers to be graded
    pub answers: Vec<UserAnswer>,
}

/// A user's answer to a single open question
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAnswer {
    /// ID of the question being answered
    pub question_id: String,
    /// User's written answer text
    pub user_answer: String,
}

/// Result of grading a single answer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradingResult {
    /// ID of the graded question
    pub question_id: String,
    /// Grade assigned (Right, Medium, Error)
    pub grade: crate::services::games::shared::types::AnswerGrade,
    /// AI-generated feedback for the answer
    pub feedback: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::games::shared::types::AnswerGrade;

    #[test]
    fn test_generate_content_input_equality() {
        let make = || GenerateContentInput {
            name: "Test".to_string(),
            description: "Desc".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Easy,
            subjects: vec![],
            num_questions: 5,
            documents: vec![],
        };
        assert_eq!(make(), make());
    }

    #[test]
    fn test_generate_content_input_debug() {
        let input = GenerateContentInput {
            name: "T".to_string(),
            description: "D".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Easy,
            subjects: vec![],
            num_questions: 1,
            documents: vec![],
        };
        let debug_str = format!("{:?}", input);
        assert!(debug_str.contains("GenerateContentInput"));
    }

    #[test]
    fn test_user_answer_equality() {
        let a = UserAnswer {
            question_id: "q1".to_string(),
            user_answer: "yes".to_string(),
        };
        let b = UserAnswer {
            question_id: "q1".to_string(),
            user_answer: "yes".to_string(),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_answer_grade_equality_and_inequality() {
        assert_eq!(AnswerGrade::Right, AnswerGrade::Right);
        assert_eq!(AnswerGrade::Medium, AnswerGrade::Medium);
        assert_eq!(AnswerGrade::Error, AnswerGrade::Error);
        assert_ne!(AnswerGrade::Right, AnswerGrade::Error);
        assert_ne!(AnswerGrade::Right, AnswerGrade::Medium);
    }
}

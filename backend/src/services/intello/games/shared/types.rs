//! Shared types for game modules

use serde::{Deserialize, Serialize};

/// Grade for a user's answer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnswerGrade {
    Right,
    Medium,
    Error,
}

/// A graded answer with feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradedAnswer {
    pub question_id: String,
    pub grade: AnswerGrade,
    pub feedback: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_grade_right_serializes_lowercase() {
        // arrange
        let grade = AnswerGrade::Right;

        // act
        let json = serde_json::to_string(&grade).unwrap();

        // assert
        assert_eq!(json, r#""right""#);
    }

    #[test]
    fn test_answer_grade_medium_serializes_lowercase() {
        // arrange
        let grade = AnswerGrade::Medium;

        // act
        let json = serde_json::to_string(&grade).unwrap();

        // assert
        assert_eq!(json, r#""medium""#);
    }

    #[test]
    fn test_answer_grade_error_serializes_lowercase() {
        // arrange
        let grade = AnswerGrade::Error;

        // act
        let json = serde_json::to_string(&grade).unwrap();

        // assert
        assert_eq!(json, r#""error""#);
    }

    #[test]
    fn test_answer_grade_deserializes_from_lowercase() {
        // arrange
        let json = r#""right""#;

        // act
        let grade: AnswerGrade =
            serde_json::from_str(json).unwrap();

        // assert
        assert_eq!(grade, AnswerGrade::Right);
    }

    #[test]
    fn test_answer_grade_equality() {
        // arrange / act / assert
        assert_eq!(AnswerGrade::Right, AnswerGrade::Right);
        assert_ne!(AnswerGrade::Right, AnswerGrade::Error);
    }

    #[test]
    fn test_graded_answer_serialization_roundtrip() {
        // arrange
        let answer = GradedAnswer {
            question_id: "q-1".to_string(),
            grade: AnswerGrade::Medium,
            feedback: "Partially correct".to_string(),
        };

        // act
        let json = serde_json::to_string(&answer).unwrap();
        let deserialized: GradedAnswer =
            serde_json::from_str(&json).unwrap();

        // assert
        assert_eq!(deserialized.question_id, "q-1");
        assert_eq!(deserialized.grade, AnswerGrade::Medium);
        assert_eq!(deserialized.feedback, "Partially correct");
    }
}

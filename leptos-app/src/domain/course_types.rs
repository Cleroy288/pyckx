//! Course domain types — courses, resources, sessions

use serde::{Deserialize, Serialize};

/// Course data from API
#[derive(Debug, Clone, Deserialize)]
pub struct CourseData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

/// Resource data
#[derive(Debug, Clone, Deserialize)]
pub struct ResourceData {
    pub id: String,
    pub name: String,
    pub content_type: String,
    pub created_at: String,
}

/// Study session data
#[derive(Debug, Clone, Deserialize)]
pub struct SessionData {
    pub id: String,
    pub course_id: String,
    pub topic: String,
    pub status: String,
    pub created_at: String,
    pub content: Option<GeneratedCourse>,
}

/// Generated course content
#[derive(Debug, Clone, Deserialize)]
pub struct GeneratedCourse {
    pub course_metadata: CourseMetadata,
    pub modules: Vec<CourseModule>,
    pub synthesis: CourseModule,
}

/// Course metadata
#[derive(Debug, Clone, Deserialize)]
pub struct CourseMetadata {
    pub title: String,
    pub description: String,
    pub level: String,
}

/// Course module with content blocks
#[derive(Debug, Clone, Deserialize)]
pub struct CourseModule {
    pub title: String,
    pub blocks: Vec<ContentBlock>,
}

/// Content block variants
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    Title { content: String },
    Subtitle { content: String },
    Text { content: String },
    Schema { language: String, content: String },
    QcmSet { data: QcmSetPayload },
    TrueFalseSet { data: TrueFalseSetPayload },
    FlashcardSet { data: FlashcardSetPayload },
}

/// Embedded QCM set in course content
#[derive(Debug, Clone, Deserialize)]
pub struct QcmSetPayload {
    pub name: String,
    pub description: String,
    pub level: String,
    pub subjects: Vec<String>,
    pub questions: Vec<QcmQuestionPayload>,
}

/// QCM question in course content
#[derive(Debug, Clone, Deserialize)]
pub struct QcmQuestionPayload {
    pub question: String,
    pub right_answer: String,
    pub wrong_answers: Vec<String>,
    pub explanation: String,
}

/// Embedded true/false set
#[derive(Debug, Clone, Deserialize)]
pub struct TrueFalseSetPayload {
    pub name: String,
    pub description: String,
    pub level: String,
    pub subjects: Vec<String>,
    pub statements: Vec<TrueFalseStatementPayload>,
}

/// True/false statement in course content
#[derive(Debug, Clone, Deserialize)]
pub struct TrueFalseStatementPayload {
    pub statement: String,
    pub answer: bool,
    pub explanation: String,
}

/// Embedded flashcard set
#[derive(Debug, Clone, Deserialize)]
pub struct FlashcardSetPayload {
    pub name: String,
    pub description: String,
    pub level: String,
    pub subjects: Vec<String>,
    pub cards: Vec<FlashcardPayload>,
}

/// Flashcard in course content
#[derive(Debug, Clone, Deserialize)]
pub struct FlashcardPayload {
    pub front: String,
    pub back: String,
}

/// Generate course request
#[derive(Debug, Clone, Serialize)]
pub struct GenerateCourseRequest {
    pub topic: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub instructions: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Generate course response
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateCourseResponse {
    pub success: bool,
    pub course: GeneratedCourse,
}

/// Create course request
#[derive(Debug, Clone, Serialize)]
pub struct CreateCourseRequest {
    pub name: String,
    pub description: String,
}

/// Create session request
#[derive(Debug, Clone, Serialize)]
pub struct CreateSessionRequest {
    pub topic: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub instructions: String,
    pub language: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_course_data() {
        let json = r#"{
            "id": "c1", "name": "Rust 101",
            "description": "Learn Rust",
            "created_at": "2024-01-01"
        }"#;
        let c: CourseData =
            serde_json::from_str(json).unwrap();
        assert_eq!(c.name, "Rust 101");
    }

    #[test]
    fn test_deserialize_content_block_title() {
        let json =
            r#"{"type":"Title","content":"Hello"}"#;
        let b: ContentBlock =
            serde_json::from_str(json).unwrap();
        assert!(matches!(
            b,
            ContentBlock::Title { content }
            if content == "Hello"
        ));
    }

    #[test]
    fn test_deserialize_content_block_schema() {
        let json = r#"{
            "type": "Schema",
            "language": "rust",
            "content": "fn main() {}"
        }"#;
        let b: ContentBlock =
            serde_json::from_str(json).unwrap();
        assert!(matches!(
            b,
            ContentBlock::Schema { language, .. }
            if language == "rust"
        ));
    }

    #[test]
    fn test_deserialize_content_block_qcm_set() {
        let json = r#"{
            "type": "QcmSet",
            "data": {
                "name": "Q", "description": "D",
                "level": "easy",
                "subjects": [],
                "questions": []
            }
        }"#;
        let b: ContentBlock =
            serde_json::from_str(json).unwrap();
        assert!(matches!(b, ContentBlock::QcmSet { .. }));
    }

    #[test]
    fn test_deserialize_session_with_content() {
        let json = r#"{
            "id": "s1", "course_id": "c1",
            "topic": "Intro", "status": "done",
            "created_at": "",
            "content": {
                "course_metadata": {
                    "title": "T",
                    "description": "D",
                    "level": "easy"
                },
                "modules": [],
                "synthesis": {
                    "title": "Summary",
                    "blocks": []
                }
            }
        }"#;
        let s: SessionData =
            serde_json::from_str(json).unwrap();
        assert!(s.content.is_some());
        let c = s.content.unwrap();
        assert_eq!(c.course_metadata.title, "T");
    }
}

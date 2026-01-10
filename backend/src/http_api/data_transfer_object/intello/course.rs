use crate::shared::utils::deserialize_string_or_vec;
use serde::{Deserialize, Serialize};

// == KNOWLEDGE EXTRACTION DTOs // ==

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DecryptedUserDemand {
    pub core_intent: String,
    pub target_audience_profile: String,
    pub mandatory_topics: Vec<String>,
    pub key_constraints: Vec<String>,
    pub pedagogical_style: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtractedKnowledge {
    pub topic_summary: String,
    pub key_concepts: Vec<KeyConcept>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyConcept {
    pub name: String,
    pub definition: String,
    pub key_points: Vec<String>,
}

// == API REQUEST/RESPONSE DTOs // ==

#[derive(Debug, Deserialize)]
pub struct GenerateCourseRequest {
    pub topic: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub resource_ids: Vec<String>,
    #[serde(default)]
    pub resources: String,
    pub session_id: Option<String>,
    pub text_length: Option<String>,
    pub exercise_depth: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GenerateCourseResponse {
    pub success: bool,
    pub course: GeneratedCourse,
}

// == COURSE STRUCTURE DTOs // ==

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeneratedCourse {
    pub course_metadata: CourseMetadata,
    pub modules: Vec<CourseModule>,
    /// Synthesis module (reuses CourseModule structure)
    /// Contains text summary + final QCM in blocks
    pub synthesis: CourseModule,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseMetadata {
    pub title: String,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub description: String,
    #[serde(default)]
    pub level: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseModule {
    pub title: String,
    pub blocks: Vec<ContentBlock>,
}

// == QCM PAYLOAD // ==

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QcmSetPayload {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub description: String,
    pub level: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    pub questions: Vec<QcmQuestionPayload>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QcmQuestionPayload {
    pub question: String,
    pub right_answer: String,
    pub wrong_answers: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub explanation: String,
}

// == FLASHCARD PAYLOAD // ==

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlashcardPayload {
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub front: String,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub back: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlashcardSetPayload {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub description: String,
    pub level: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    pub cards: Vec<FlashcardPayload>,
}

// == TRUE/FALSE PAYLOAD // ==

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrueFalseStatementPayload {
    pub statement: String,
    pub answer: bool,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub explanation: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrueFalseSetPayload {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub description: String,
    pub level: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    pub statements: Vec<TrueFalseStatementPayload>,
}

// == CONTENT BLOCK (7 types) // ==

/// Content blocks for course modules
///
/// Uses polymorphic deserialization for `content` fields to handle
/// AI outputting arrays instead of strings.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Title {
        #[serde(default, deserialize_with = "deserialize_string_or_vec")]
        content: String,
    },
    Subtitle {
        #[serde(default, deserialize_with = "deserialize_string_or_vec")]
        content: String,
    },
    Text {
        #[serde(default, deserialize_with = "deserialize_string_or_vec")]
        content: String,
    },
    Schema {
        language: String,
        #[serde(default, deserialize_with = "deserialize_string_or_vec")]
        content: String,
    },
    QcmSet {
        data: QcmSetPayload,
    },
    TrueFalseSet {
        data: TrueFalseSetPayload,
    },
    FlashcardSet {
        data: FlashcardSetPayload,
    },
}

// == CORE EDUCATIONAL CONTENT // ==

/// Detailed educational content for a course section
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ContentSection {
    pub title: String,
    pub content: String,         // 400-600 words of detailed explanation
    pub key_points: Vec<String>, // 3-5 bullet points
    pub examples: Vec<String>,   // 2-3 real-world examples
}

/// Detailed educational content for the entire course (Stage 1.5 output)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct DetailedCourseContent {
    pub title: String,
    pub introduction: String, // 200+ words course introduction
    pub sections: Vec<ContentSection>,
    pub conclusion: String, // 100+ words wrap-up
}

// == FULL GENERATION RESULT // ==

/// Full result of the course generation process, including intermediate artifacts
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseGenerationResult {
    pub course: GeneratedCourse,
    pub extracted_knowledge: ExtractedKnowledge,
    pub educational_content: DetailedCourseContent,
    pub expanded_knowledge: String,
}

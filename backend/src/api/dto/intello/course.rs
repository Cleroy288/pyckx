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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseMetadata {
    pub title: String,
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
    pub explanation: String,
}

// == FLASHCARD PAYLOAD // ==

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlashcardPayload {
    pub front: String,
    pub back: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlashcardSetPayload {
    pub name: String,
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
    pub explanation: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrueFalseSetPayload {
    pub name: String,
    pub description: String,
    pub level: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    pub statements: Vec<TrueFalseStatementPayload>,
}

// == CONTENT BLOCK (7 types) // ==

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Title { content: String },
    Subtitle { content: String },
    Text { content: String },
    Schema { language: String, content: String },
    QcmSet { data: QcmSetPayload },
    TrueFalseSet { data: TrueFalseSetPayload },
    FlashcardSet { data: FlashcardSetPayload },
}

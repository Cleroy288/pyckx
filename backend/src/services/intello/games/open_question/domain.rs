//! Open Question domain entities
//!
//! Represents a set of open-ended questions where users write their own answers.
//! Unlike QCM (multiple choice), these questions have no predefined answers.

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::intello::{Level, QuestionId, SetId};

// ** OpenQuestion **
// ==> A single open-ended question with user's written answer
//
// @ id : Unique identifier for the question
// @ question : The question text
// @ user_answer : User's written answer (empty if not answered yet)
// @ expected_answer : Expected answer for AI-generated sets (for self-grading)
// @ hint : Optional hint for the user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenQuestion {
    pub id: QuestionId,                  // unique identifier
    pub question: String,                // question text
    pub user_answer: String,             // user's answer
    pub expected_answer: Option<String>, // expected answer
    pub hint: Option<String>,            // optional hint
}

// ** OpenQuestionSet **
// ==> A set of open-ended questions grouped by topic
//
// @ id : Unique identifier for the set
// @ user_id : User ID who owns this set
// @ name : Name of the open question set
// @ description : Description of what this set covers
// @ level : Difficulty level (Easy, Medium, Hard)
// @ language : Language of the set (e.g., "en", "fr")
// @ subjects : List of subjects this set covers
// @ questions : List of open questions in this set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenQuestionSet {
    pub id: SetId,                    // unique identifier
    pub user_id: UserId,              // owner
    pub name: String,                 // set name
    pub description: String,          // description
    pub level: Level,                 // difficulty level
    pub language: String,             // language code
    pub subjects: Vec<String>,        // subject tags
    pub questions: Vec<OpenQuestion>, // questions
}

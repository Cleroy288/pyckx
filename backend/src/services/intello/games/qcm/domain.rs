//! QCM domain entities
//!
//! Pure data structures representing QCM questions and sets.

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::intello::{Level, QuestionId, SetId};

// ** QcmQuestion **
// ==> A single QCM question with one correct answer and three wrong answers
//
// @ id : Unique identifier for the question
// @ question : The question text
// @ wrong_answers : List of 3 wrong answers
// @ right_answer : The correct answer
// @ explanation : Explanation of why the answer is correct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QcmQuestion {
    pub id: QuestionId,           // unique identifier
    pub question: String,         // question text
    pub wrong_answers: Vec<String>, // list of 3 wrong answers
    pub right_answer: String,     // correct answer
    pub explanation: String,      // explanation of correctness
}

// ** QcmSet **
// ==> A set of QCM questions grouped by topic
//
// @ id : Unique identifier for the set
// @ user_id : User ID who owns this set
// @ name : Name of the QCM set
// @ description : Description of what this set covers
// @ level : Difficulty level (Easy, Medium, Hard)
// @ language : Language of the QCM set (e.g., "en", "fr", "es")
// @ subjects : List of subjects this set covers
// @ questions : List of questions in this set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QcmSet {
    pub id: SetId,                  // unique identifier
    pub user_id: UserId,            // owner
    pub name: String,               // name of the set
    pub description: String,        // description
    pub level: Level,               // difficulty level
    pub language: String,           // language code
    pub subjects: Vec<String>,      // covered subjects
    pub questions: Vec<QcmQuestion>, // questions in set
}

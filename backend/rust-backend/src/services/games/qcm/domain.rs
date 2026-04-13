//! QCM domain entities
//!
//! Pure data structures representing QCM questions and sets.

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::{Level, QuestionId, SetId};

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
    pub id: QuestionId,             // unique identifier
    pub question: String,           // question text
    pub wrong_answers: Vec<String>, // list of 3 wrong answers
    pub right_answer: String,       // correct answer
    pub explanation: String,        // explanation of correctness
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
    pub id: SetId,                   // unique identifier
    pub user_id: UserId,             // owner
    pub name: String,                // name of the set
    pub description: String,         // description
    pub level: Level,                // difficulty level
    pub language: String,            // language code
    pub subjects: Vec<String>,       // covered subjects
    pub questions: Vec<QcmQuestion>, // questions in set
}

/// Input for creating a QCM set (no IDs required)
#[derive(Debug, Clone)]
pub struct CreateQcmInput {
    pub name: String,
    pub description: String,
    pub level: Level,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<QuestionInput>,
}

/// Input for a single QCM question (no ID)
#[derive(Debug, Clone)]
pub struct QuestionInput {
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}

impl QcmSet {
    /// Build a QcmSet from input, generating IDs
    pub fn from_input(
        user_id: &str,
        input: CreateQcmInput,
    ) -> Self {
        let questions = input
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

        Self {
            id: SetId::new(),
            user_id: user_id.into(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        }
    }
}

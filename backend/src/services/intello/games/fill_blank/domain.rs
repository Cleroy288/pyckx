//! Fill in the Blank domain entities
//!
//! A FillBlankSet is a collection of fill-in-the-blank questions where users
//! complete phrases by selecting the correct answer from multiple options.

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::intello::{Level, OptionId, QuestionId, SetId};

// ** FillBlankOption **
// ==> A single answer option that may be correct or incorrect
//
// @ id : Unique identifier
// @ text : The option text/word to fill in the blank
// @ is_correct : Whether this option is the correct answer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FillBlankOption {
    pub id: OptionId,     // unique identifier
    pub text: String,     // option text
    pub is_correct: bool, // correctness flag
}

// ** FillBlankQuestion **
// ==> A fill-in-the-blank question with phrase and answer options
//
// @ id : Unique identifier
// @ phrase : The phrase with blank to fill (e.g., "My name is ___")
// @ options : Answer options (exactly one should be correct)
// @ explanation : Explanation of why the correct answer is right
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FillBlankQuestion {
    pub id: QuestionId,                // unique identifier
    pub phrase: String,                // phrase with blank
    pub options: Vec<FillBlankOption>, // answer options
    pub explanation: String,           // explanation
}

// ** FillBlankSet **
// ==> A set of fill-in-the-blank questions for a specific topic
//
// @ id : Unique identifier
// @ user_id : Owner user ID
// @ name : Set name/title
// @ description : Set description
// @ level : Difficulty level (Easy, Medium, Hard)
// @ language : Language of the content
// @ subjects : Subject tags (max 3)
// @ questions : The fill-in-the-blank questions in this set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FillBlankSet {
    pub id: SetId,                         // unique identifier
    pub user_id: UserId,                   // owner
    pub name: String,                      // set name
    pub description: String,               // description
    pub level: Level,                      // difficulty level
    pub language: String,                  // language code
    pub subjects: Vec<String>,             // subject tags
    pub questions: Vec<FillBlankQuestion>, // questions
}

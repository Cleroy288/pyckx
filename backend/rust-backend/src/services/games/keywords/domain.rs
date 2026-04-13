//! Keywords domain entities
//!
//! A KeywordSet is a collection of keyword questions where users must identify
//! which keywords are related to a given statement.

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::{Level, OptionId, QuestionId, SetId};

// ** Keyword **
// ==> A single keyword option that may be correct or incorrect
//
// @ id : Unique identifier
// @ word : The keyword word/phrase
// @ is_correct : Whether this keyword is correctly related to the statement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Keyword {
    pub id: OptionId,     // unique identifier
    pub word: String,     // keyword word/phrase
    pub is_correct: bool, // correctness flag
}

// ** KeywordQuestion **
// ==> A keyword question with a statement and keyword options to match
//
// @ id : Unique identifier
// @ statement : The statement/fact to match keywords to
// @ keywords : Keyword options (mix of correct and incorrect)
// @ explanation : Explanation of why correct keywords relate to statement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeywordQuestion {
    pub id: QuestionId,         // unique identifier
    pub statement: String,      // statement to match
    pub keywords: Vec<Keyword>, // keyword options
    pub explanation: String,    // explanation
}

// ** KeywordSet **
// ==> A set of keyword questions for a specific topic
//
// @ id : Unique identifier
// @ user_id : Owner user ID
// @ name : Set name/title
// @ description : Set description
// @ level : Difficulty level (Easy, Medium, Hard)
// @ language : Language of the content
// @ subjects : Subject tags (max 3)
// @ questions : The keyword questions in this set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeywordSet {
    pub id: SetId,                       // unique identifier
    pub user_id: UserId,                 // owner
    pub name: String,                    // set name
    pub description: String,             // description
    pub level: Level,                    // difficulty level
    pub language: String,                // language code
    pub subjects: Vec<String>,           // subject tags
    pub questions: Vec<KeywordQuestion>, // questions
}

//! True or False domain entities
//!
//! A TrueOrFalseSet is a collection of statements where users must determine
//! if each statement is true or false.

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::intello::{Level, QuestionId, SetId};

// ** TrueOrFalseStatement **
// ==> A single statement to judge as true or false
//
// @ id : Unique identifier
// @ statement : The statement to evaluate
// @ answer : Correct answer (true if statement is true, false otherwise)
// @ explanation : Explanation of why statement is true/false
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrueOrFalseStatement {
    pub id: QuestionId,      // unique identifier
    pub statement: String,   // statement to judge
    pub answer: bool,        // correct answer
    pub explanation: String, // explanation
}

// ** TrueOrFalseSet **
// ==> A set of true/false statements for a specific topic
//
// @ id : Unique identifier
// @ user_id : Owner user ID
// @ name : Set name/title
// @ description : Set description
// @ level : Difficulty level (Easy, Medium, Hard)
// @ language : Language of the content
// @ subjects : Subject tags (max 3)
// @ statements : The statements in this set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrueOrFalseSet {
    pub id: SetId,                            // unique identifier
    pub user_id: UserId,                      // owner
    pub name: String,                         // set name
    pub description: String,                  // description
    pub level: Level,                         // difficulty level
    pub language: String,                     // language code
    pub subjects: Vec<String>,                // subject tags
    pub statements: Vec<TrueOrFalseStatement>, // statements
}

//! Order Phrase domain entities
//!
//! An OrderPhraseSet is a collection of phrase puzzles where users must
//! arrange shuffled words into the correct order to form a coherent phrase.

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::intello::{Level, OptionId, QuestionId, SetId};

// ** OrderPhraseWord **
// ==> A single word in a phrase puzzle with its correct position
//
// @ id : Unique identifier
// @ word : The word text
// @ position : Correct position in phrase (0-indexed)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderPhraseWord {
    pub id: OptionId,       // unique identifier
    pub word: String,       // word text
    pub position: u8,       // correct position
}

// ** OrderPhraseQuestion **
// ==> A phrase puzzle where words must be arranged in correct order
//
// @ id : Unique identifier
// @ original_phrase : Complete phrase for display/verification
// @ words : Words with positions (sent shuffled to frontend)
// @ hint : Optional hint to help the user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderPhraseQuestion {
    pub id: QuestionId,                // unique identifier
    pub original_phrase: String,       // original complete phrase
    pub words: Vec<OrderPhraseWord>,   // words with positions
    pub hint: String,                  // hint
}

// ** OrderPhraseSet **
// ==> A set of phrase puzzles for a specific topic
//
// @ id : Unique identifier
// @ user_id : Owner user ID
// @ name : Set name/title
// @ description : Set description
// @ level : Difficulty level (Easy, Medium, Hard)
// @ language : Language of the content
// @ subjects : Subject tags (max 3)
// @ questions : The phrase puzzles in this set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderPhraseSet {
    pub id: SetId,                          // unique identifier
    pub user_id: UserId,                    // owner
    pub name: String,                       // set name
    pub description: String,                // description
    pub level: Level,                       // difficulty level
    pub language: String,                   // language code
    pub subjects: Vec<String>,              // subject tags
    pub questions: Vec<OrderPhraseQuestion>, // questions
}

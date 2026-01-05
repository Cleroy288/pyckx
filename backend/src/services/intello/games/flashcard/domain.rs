//! Flashcard domain entities
//!
//! A FlashcardSet is a collection of flashcards for memorization exercises.
//! Each flashcard has a front (question/prompt) and back (answer/explanation).

use serde::{Deserialize, Serialize};

use crate::infra::user::UserId;
use crate::services::intello::{Level, QuestionId, SetId};

// ** Flashcard **
// ==> A single flashcard for memorization with front and back sides
//
// @ id : Unique identifier for the flashcard
// @ front : Front side (question/term/prompt)
// @ back : Back side (answer/definition/explanation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Flashcard {
    pub id: QuestionId,  // unique identifier
    pub front: String,   // front side
    pub back: String,    // back side
}

// ** FlashcardSet **
// ==> A set of flashcards for a specific topic
//
// @ id : Unique identifier for the set
// @ user_id : Owner user ID
// @ name : Set name/title
// @ description : Set description
// @ level : Difficulty level (Easy, Medium, Hard)
// @ language : Language of the content
// @ subjects : Subject tags (max 3)
// @ cards : The flashcards in this set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlashcardSet {
    pub id: SetId,              // unique identifier
    pub user_id: UserId,        // owner
    pub name: String,           // set name
    pub description: String,    // description
    pub level: Level,           // difficulty level
    pub language: String,       // language code
    pub subjects: Vec<String>,  // subject tags
    pub cards: Vec<Flashcard>,  // flashcards
}

//! Flashcards API surface — re-exported from
//! `crate::api::flashcards` to give the feature a single
//! `use crate::flashcards::api::*;` entry point.

pub use crate::api::flashcards::{
    create_flashcards, get_flashcard_sets,
};

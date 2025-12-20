//! Intello Use Cases
//!
//! Use cases for the Intello learning app: QCM generation, flashcards,
//! open questions, true/false, keywords, order phrases, and fill blanks.
//!
//! ## Shared Types
//! All generation use cases share a common input type (`GenerateGameInput`)
//! and validation logic (`validate_generation_input`) from the `shared` module.

#![allow(unused_imports)]

mod check_answers;
mod delete_set;
mod generate_fill_blank;
mod generate_flashcards;
mod generate_keywords;
mod generate_open_questions;
mod generate_order_phrase;
mod generate_qcm;
mod generate_true_false;
mod get_user_sets;
pub mod shared;

// Shared types (re-exported for ergonomic use)
pub use shared::{GenerateGameInput, GenerateGameOutput, validate_generation_input};

// Check Answers
pub use check_answers::{
    CheckAnswersUseCaseInput, CheckOpenQuestionAnswersUseCase, UserAnswerInput,
};

// Delete Set (not yet used by handlers)
pub use delete_set::DeleteSetUseCase;

// Generate Fill Blank
pub use generate_fill_blank::{GenerateFillBlankInput, GenerateFillBlankUseCase};

// Generate Flashcards
pub use generate_flashcards::{GenerateFlashcardsInput, GenerateFlashcardsUseCase};

// Generate Keywords
pub use generate_keywords::{GenerateKeywordsInput, GenerateKeywordsUseCase};

// Generate Open Questions
pub use generate_open_questions::{GenerateOpenQuestionsInput, GenerateOpenQuestionsUseCase};

// Generate Order Phrase
pub use generate_order_phrase::{GenerateOrderPhraseInput, GenerateOrderPhraseUseCase};

// Generate QCM
pub use generate_qcm::{GenerateQcmInput, GenerateQcmUseCase};

// Generate True/False
pub use generate_true_false::{GenerateTrueFalseInput, GenerateTrueFalseUseCase};

// Get User Sets (not yet used by handlers)
pub use get_user_sets::GetUserSetsUseCase;


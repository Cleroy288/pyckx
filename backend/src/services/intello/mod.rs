//! Intello Service module
//!
//! Business logic for all Intello app operations (QCM, Open Questions, Flashcards, True/False, Keywords, Fill Blank, Course/Sessions).

mod course_ops;     // Course and Session operations
pub mod crud_ops;   // Generic CRUD helpers
mod fill_blank_ops;
mod flashcard_ops;
mod games;
mod keywords_ops;
mod open_question_ops;
mod order_phrase_ops;
mod qcm_ops;
mod true_false_ops;
mod types;
mod validation;

pub use types::{
    AnswerGrade, CheckAnswersInput, GenerateContentInput, GradingResult, IntelloRepositories,
    IntelloService, UserAnswer,
};

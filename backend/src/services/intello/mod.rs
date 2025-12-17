//! Intello Service module
//!
//! Business logic for all Intello app operations (QCM, Open Questions, Flashcards, True/False, Keywords).

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
    AnswerGrade, CheckAnswersInput, GenerateContentInput, IntelloService, UserAnswer,
};



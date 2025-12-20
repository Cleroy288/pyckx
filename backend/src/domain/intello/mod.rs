//! Intello domain entities
//!
//! Contains pure data structures for the Intello quiz/learning app.
//! These types have no I/O or business logic - they are pure domain entities.
//!
//! # Structure
//! - `enums.rs` - Level and DocumentType enumerations
//! - `qcm_question.rs` - QcmQuestion entity
//! - `qcm_set.rs` - QcmSet entity
//! - `custom_question.rs` - CustomQuestion entity for AI-generated content
//! - `open_question.rs` - OpenQuestion and OpenQuestionSet entities
//! - `flashcard.rs` - Flashcard and FlashcardSet entities
//! - `true_false.rs` - TrueOrFalseStatement and TrueOrFalseSet entities
//! - `keywords.rs` - Keyword, KeywordQuestion, KeywordSet entities
//! - `course.rs` - Course, Lesson, Exercise entities for structured learning

pub mod course;
pub mod custom_question;
pub mod enums;
pub mod fill_blank;
pub mod flashcard;
pub mod keywords;
pub mod open_question;
pub mod order_phrase;
pub mod qcm_question;
pub mod qcm_set;
pub mod true_false;

// Course types (for future use)
// Course types (reserved for future use, not currently exported)
// pub use course::{Course, Exercise, ExerciseContent, ExerciseType, Lesson};
pub use custom_question::{CustomQuestion, CustomQuestionDocument, DocumentType};
pub use enums::Level;
pub use fill_blank::{FillBlankOption, FillBlankQuestion, FillBlankSet};
pub use flashcard::{Flashcard, FlashcardSet};
pub use keywords::{Keyword, KeywordQuestion, KeywordSet};
pub use open_question::{OpenQuestion, OpenQuestionSet};
pub use order_phrase::{OrderPhraseQuestion, OrderPhraseSet, OrderPhraseWord};
pub use qcm_question::QcmQuestion;
pub use qcm_set::QcmSet;
pub use true_false::{TrueOrFalseSet, TrueOrFalseStatement};



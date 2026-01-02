//! AI Response schemas for all game types
//!
//! These types represent the expected JSON structure from AI responses.

mod fill_blank;
mod flashcard;
mod keywords;
mod open_question;
mod order_phrase;
mod qcm;
mod true_false;
mod verification;

// Re-export all types
pub use fill_blank::{FillBlankAiOption, FillBlankAiQuestion, FillBlankAiResponse};
pub use flashcard::{FlashcardAiCard, FlashcardAiResponse};
pub use keywords::{KeywordsAiKeyword, KeywordsAiQuestion, KeywordsAiResponse};
pub use open_question::{OpenQuestionAiQuestion, OpenQuestionAiResponse};
pub use order_phrase::{OrderPhraseAiQuestion, OrderPhraseAiResponse, OrderPhraseAiWord};
pub use qcm::{QcmAiQuestion, QcmAiResponse};
pub use true_false::{TrueOrFalseAiResponse, TrueOrFalseAiStatement};
pub use verification::{GradedAnswerAi, VerificationAiResponse};

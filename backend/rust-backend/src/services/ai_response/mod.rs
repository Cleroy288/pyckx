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
pub use fill_blank::FillBlankAiResponse;
pub use flashcard::FlashcardAiResponse;
pub use keywords::KeywordsAiResponse;
pub use open_question::OpenQuestionAiResponse;
pub use order_phrase::OrderPhraseAiResponse;
pub use qcm::QcmAiResponse;
pub use true_false::TrueOrFalseAiResponse;
pub use verification::VerificationAiResponse;

//! Shared utilities and constants used across the application.

pub mod constants;
pub mod document_extractor;
pub mod open_question_cache;
pub mod prompt_builder;
pub mod session;

pub use document_extractor::{extract_pdf, extract_pptx, extract_txt, extract_word};
pub use open_question_cache::OpenQuestionCache;
pub use prompt_builder::{
    build_flashcard_prompt, build_keywords_prompt, build_open_question_prompt, build_order_phrase_prompt, build_prompt,
    build_true_false_prompt, build_verification_prompt, AnswerToGrade, FlashcardPromptInput,
    KeywordsPromptInput, OpenQuestionPromptInput, OrderPhrasePromptInput, TrueOrFalsePromptInput, VerificationPromptInput,
};



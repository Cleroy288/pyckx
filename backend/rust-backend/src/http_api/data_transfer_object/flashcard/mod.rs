//! Flashcard DTOs

pub mod conversion;
pub mod request;
pub mod response;

pub use request::CreateFlashcardRequest;
pub use response::{
    CreateFlashcardResponse, FlashcardResponse, FlashcardSetListResponse,
};

//! AI Use Cases
//!
//! Shared use cases for AI-powered content generation and answer verification.
//! Note: These are utility use cases, exported for future use.

#![allow(unused_imports)]

mod generate_content;
mod verify_answers;

pub use generate_content::GenerateContentUseCase;
pub use verify_answers::VerifyAnswersUseCase;

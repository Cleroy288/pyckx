//! Intello DTOs - Request/Response types for Intello game endpoints
//!
//! # Structure
//! - `games.rs` - Game listing DTOs
//! - `qcm/` - QCM (Multiple Choice) DTOs
//! - `open_question/` - Open Question DTOs
//! - `flashcard/` - Flashcard DTOs
//! - `true_false/` - True or False DTOs
//! - `keywords/` - Keywords DTOs
//! - `fill_blank/` - Fill in the Blank DTOs
//! - `custom_question.rs` - Custom Question generation DTOs
//! - `validation.rs` - Shared validation logic

pub mod course;

pub mod admin;
pub mod custom_question;
pub mod fill_blank;
pub mod flashcard;
pub mod games;
pub mod keywords;
pub mod open_question;
pub mod order_phrase;
pub mod qcm;
pub mod true_false;
pub mod validation;

// Re-export games
pub use games::{AvailableGamesResponse, GameResponse};

// Re-export QCM types
pub use qcm::{
    CreateQcmSetRequest, QcmQuestionResponse, QcmSetListResponse, QcmSetResponse,
    QcmSuccessResponse, UpdateQcmSetRequest,
};

// Re-export Open Question types
pub use open_question::{
    CheckAnswersRequest, CheckAnswersResponse, CreateOpenQuestionRequest,
    CreateOpenQuestionResponse, GradedAnswerResponse, OpenQuestionResponse,
    OpenQuestionSetListResponse,
};

// Re-export Flashcard types
pub use flashcard::{
    CreateFlashcardRequest, CreateFlashcardResponse, FlashcardResponse, FlashcardSetListResponse,
};

// Re-export True or False types
pub use true_false::{
    CreateTrueOrFalseRequest, CreateTrueOrFalseResponse, TrueOrFalseSetListResponse,
    TrueOrFalseSetWithStatementsResponse, TrueOrFalseStatementResponse,
};

// Re-export Keywords types
pub use keywords::{
    CreateKeywordsRequest, CreateKeywordsResponse, KeywordQuestionResponse, KeywordResponse,
    KeywordSetListResponse, KeywordSetWithQuestionsResponse,
};

// Re-export Order Phrase types
pub use order_phrase::{
    CreateOrderPhraseRequest, CreateOrderPhraseResponse, OrderPhraseQuestionResponse,
    OrderPhraseSetListResponse, OrderPhraseSetWithQuestionsResponse, OrderPhraseWordResponse,
};

// Re-export Fill Blank types
pub use fill_blank::{
    CreateFillBlankRequest, CreateFillBlankResponse, FillBlankOptionResponse,
    FillBlankQuestionResponse, FillBlankSetListResponse, FillBlankSetWithQuestionsResponse,
};

// Re-export Custom Question types
pub use custom_question::{CreateCustomQuestionRequest, CustomQuestionResponse};

//! Data Transfer Objects - Request/Response types

pub mod apps;
pub mod auth;
pub mod collection;
pub mod user;

pub mod admin;
pub mod course;
pub mod custom_question;
pub mod fill_blank;
pub mod flashcard;
pub mod games;
pub mod keywords;
pub mod open_question;
pub mod order_phrase;
pub mod qcm;
pub mod quick_qcm;
pub mod true_false;
pub mod validation;

// Re-export apps types
pub use apps::{
    AddUserAppRequest, AppListResponse, AppResponse,
    AppSuccessResponse, CreateAppRequest, SuccessResponse,
    UpdateAppRequest, UserAppSuccessResponse,
};

// Re-export auth types
pub use auth::{
    AuthResponse, LoginRequest, RegisterRequest,
};

// Re-export collection types
pub use collection::{
    AddDvdRequest, CollectionItemsResponse,
    CollectionListResponse, CollectionSuccessResponse,
    CreateCollectionRequest, DeleteResponse,
    DvdListResponse, DvdResponse, DvdSuccessResponse,
    UpdateDvdRequest,
};

// Re-export user types
pub use user::UserResponse;

// Re-export game DTOs
pub use games::{
    AvailableGamesResponse, CheckCodingRequest,
    CheckCodingResponse, GameResponse,
    GenerateCodingGameRequest, GenerateCodingGameResponse,
};

pub use qcm::{
    CreateQcmSetRequest, QcmQuestionResponse,
    QcmSetListResponse, QcmSetResponse,
    QcmSuccessResponse, UpdateQcmSetRequest,
};

pub use open_question::{
    CheckAnswersRequest, CheckAnswersResponse,
    CreateOpenQuestionRequest,
    CreateOpenQuestionResponse,
    GradedAnswerResponse, OpenQuestionResponse,
    OpenQuestionSetListResponse,
};

pub use flashcard::{
    CreateFlashcardRequest, CreateFlashcardResponse,
    FlashcardResponse, FlashcardSetListResponse,
};

pub use true_false::{
    CreateTrueOrFalseRequest,
    CreateTrueOrFalseResponse,
    TrueOrFalseSetListResponse,
    TrueOrFalseSetWithStatementsResponse,
    TrueOrFalseStatementResponse,
};

pub use keywords::{
    CreateKeywordsRequest, CreateKeywordsResponse,
    KeywordQuestionResponse, KeywordResponse,
    KeywordSetListResponse,
    KeywordSetWithQuestionsResponse,
};

pub use order_phrase::{
    CreateOrderPhraseRequest,
    CreateOrderPhraseResponse,
    OrderPhraseQuestionResponse,
    OrderPhraseSetListResponse,
    OrderPhraseSetWithQuestionsResponse,
    OrderPhraseWordResponse,
};

pub use fill_blank::{
    CreateFillBlankRequest, CreateFillBlankResponse,
    FillBlankOptionResponse,
    FillBlankQuestionResponse,
    FillBlankSetListResponse,
    FillBlankSetWithQuestionsResponse,
};

pub use custom_question::{
    CreateCustomQuestionRequest,
    CustomQuestionResponse,
};

pub use quick_qcm::{QuickQcmRequest, QuickQcmResponse};

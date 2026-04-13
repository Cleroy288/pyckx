//! QCM types — re-exports from the domain layer.
//!
//! Centralizes every type the QCM feature needs, so
//! callers never reach into `crate::domain::qcm_types`.

pub use crate::domain::qcm_types::{
    CreateQcmQuestionInput, CreateQcmSetRequest,
    GenerateQcmResponse, QcmQuestion, QcmSet,
    QcmSetListResponse, QcmSuccessResponse,
    QuickQcmApiResponse,
};

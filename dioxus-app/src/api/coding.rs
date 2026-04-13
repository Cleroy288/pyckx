//! Compatibility shim — real implementation moved to
//! `crate::coding::api`. Kept so legacy import paths resolve.

pub use crate::coding::api::{
    check as check_coding_game, generate as generate_coding_game,
    CheckRequest as CheckCodingRequest,
    CheckResponse as CheckCodingResponse,
    ExerciseResponse as CodingExerciseResponse,
    GenerateRequest as GenerateCodingRequest,
};

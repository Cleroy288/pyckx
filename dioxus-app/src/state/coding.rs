//! Compatibility shim — real implementation moved to
//! `crate::coding::state`. Kept so that `state/mod.rs`
//! and `main.rs` still resolve `CodingExerciseProvider`.

pub use crate::coding::state::{
    CodingExercise, CodingExerciseProvider,
};

//! Traits and shared types for the game engine.
//!
//! This module re-exports the domain trait and core
//! game types so game-engine consumers have a single
//! import surface: `use crate::game_engine::*;`.

pub use crate::domain::game_set_trait::GameSetInfo;
pub use crate::domain::game_types::{
    Level, NumQuestions, StudyApiError,
};

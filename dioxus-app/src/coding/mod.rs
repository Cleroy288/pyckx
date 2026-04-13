//! Coding feature — code editor exercises.
//!
//! Layers:
//! - `api`        : HTTP calls (generate / check)
//! - `state`      : domain exercise + reactive provider
//! - `generate`   : page that requests a new exercise
//! - `play`       : page that hosts the editor and submits
//! - `*_form`/`*_view` : presentational helpers

pub mod api;
pub mod state;

mod generate;
mod generate_form;
mod play;
mod play_view;

pub use generate::CodingGeneratePage;
pub use play::CodingPlayPage;
pub use state::{CodingExercise, CodingExerciseProvider};

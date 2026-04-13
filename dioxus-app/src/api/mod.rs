//! API module — HTTP client for backend communication

pub mod auth;
pub mod coding;
pub mod collection;
pub mod courses;
pub mod endpoints;
pub mod fill_blanks;
pub mod form_data;
pub mod flashcards;
pub mod helpers;
pub mod study;
pub mod log;
pub mod keywords;
pub mod open_questions;
pub mod resources;
pub mod true_false;
pub mod types;
pub mod user_apps;

pub use auth::*;
pub use types::*;

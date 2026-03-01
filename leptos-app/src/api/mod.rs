//! API module — HTTP client for backend communication

pub mod admin;
pub mod auth;
pub mod collection;
pub mod courses;
pub mod endpoints;
pub mod fill_blanks;
pub mod flashcards;
pub mod helpers;
pub mod intello;
pub mod keywords;
pub mod open_questions;
pub mod order_phrases;
pub mod resources;
pub mod true_false;
pub mod types;
pub mod user_apps;

pub use auth::*;
pub use types::*;

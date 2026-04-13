//! Domain layer — pure business types, zero framework deps

pub mod app_registry;
pub mod app_types;
pub mod auth_types;
pub mod collection_types;
pub mod course_types;
pub mod fill_blank_types;
pub mod game_set_trait;
pub mod flashcard_types;
pub mod game_types;
pub mod keywords_types;
pub mod open_question_types;
pub mod order_phrase_types;
pub mod palette_data;
pub mod palette_types;
pub mod qcm_types;
pub mod true_false_types;
pub mod validation;

pub use palette_data::{default_palette, palette_by_id};
pub use palette_types::*;

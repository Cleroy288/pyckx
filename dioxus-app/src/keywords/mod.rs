//! Keywords feature — domain-aware module that wires the
//! API layer, the player and the three pages (list,
//! generate, play) on top of `crate::game_engine`.
//!
//! Single import surface: `use crate::keywords::*;`.

pub mod api;
pub mod generate;
pub mod list;
pub mod play;
pub mod player;
mod player_chips;
mod player_handlers;
pub mod types;

pub use generate::KeywordsGeneratePage;
pub use list::KeywordsListPage;
pub use play::KeywordsPlayPage;
pub use player::KeywordsPlayer;

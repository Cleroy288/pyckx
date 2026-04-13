//! Order Phrases — full feature module.
//!
//! Single-import surface: `use crate::order_phrases::*;`.
//! Layered top-down: types -> api -> pages -> player.

pub mod api;
pub mod generate;
pub mod list;
pub mod play;
pub mod player;
pub mod types;

pub use generate::OrderPhraseGeneratePage;
pub use list::OrderPhraseListPage;
pub use play::OrderPhrasePlayPage;
pub use player::OrderPhrasePlayer;

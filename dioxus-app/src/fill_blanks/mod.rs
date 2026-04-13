//! Fill-blank feature — list, generate, and play the
//! "fill-the-blank" game.
//!
//! Flat facade: every page and the player are re-exported
//! so consumers import them via `crate::fill_blanks::*`.

pub mod api;
pub mod generate;
pub mod list;
pub mod play;
pub mod player;
pub mod types;

pub use generate::FillBlankGeneratePage;
pub use list::FillBlankListPage;
pub use play::FillBlankPlayPage;
pub use player::FillBlankPlayer;
pub use types::{
    CreateFillBlankResponse, FillBlankOption,
    FillBlankQuestion, FillBlankSet,
    FillBlankSetListResponse,
};

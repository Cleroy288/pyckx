//! True/False feature — list, generate, play.
//!
//! Flat facade: every page and the player are re-exported
//! so consumers import them via `crate::true_false::*`.

pub mod api;
pub mod generate;
pub mod list;
pub mod play;
pub mod player;
pub mod types;

pub use generate::TrueFalseGeneratePage;
pub use list::TrueFalseListPage;
pub use play::TrueFalsePlayPage;
pub use player::TrueFalsePlayer;
pub use types::{TrueFalseSet, TrueFalseStatement};

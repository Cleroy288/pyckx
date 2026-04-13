//! QCM feature — list, create, generate, quick, play.
//!
//! Flat facade: every page and the player are re-exported
//! so consumers import them via `crate::qcm::*`.

pub mod api;
pub mod create;
mod create_fields;
mod create_questions;
pub mod generate;
pub mod list;
pub mod play;
pub mod player;
mod player_option;
pub mod quick;
pub mod types;

pub use create::QcmCreatePage;
pub use generate::QcmGeneratePage;
pub use list::QcmListPage;
pub use play::QcmPlayPage;
pub use player::QcmPlayer;
pub use quick::QcmQuickPage;
pub use types::{QcmQuestion, QcmSet};

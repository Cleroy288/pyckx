//! Open Questions feature — list, AI generate, play, grade.
//!
//! Layered like every other game module:
//! * `types`    — domain re-exports + pure helpers
//! * `api`      — HTTP client (list / create / grade)
//! * `list`     — list page wired to `GameListPage`
//! * `generate` — generate page wired to `GameGeneratePage`
//! * `play`     — play page wired to `GamePlayPage`
//! * `player`   — interactive answer + grading view
//!
//! Public surface: import the three page components and the
//! player from this module's root.

pub mod api;
pub mod generate;
pub mod list;
pub mod play;
pub mod player;
pub mod types;

pub use generate::OpenQuestionGeneratePage;
pub use list::OpenQuestionListPage;
pub use play::OpenQuestionPlayPage;
pub use player::OpenQuestionPlayer;

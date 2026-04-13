//! Flashcards feature — domain types, API, player and the
//! three route-bound pages (list / generate / play).
//!
//! Reads top-to-bottom like a journal: `types` define the
//! data, `api` talks to the backend, `player` plays a set,
//! and `list` / `generate` / `play` wire it all into the
//! generic `game_engine` pages.

pub mod api;
pub mod generate;
pub mod list;
pub mod play;
pub mod player;
pub mod types;

pub use generate::FlashcardGeneratePage;
pub use list::FlashcardListPage;
pub use play::FlashcardPlayPage;
pub use player::FlashcardPlayer;

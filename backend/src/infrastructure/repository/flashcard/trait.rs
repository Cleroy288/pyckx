//! Flashcard Repository Trait - Abstraction for flashcard set persistence
//!
//! This trait extends the generic `GameSetRepository` for flashcard-specific operations.

use crate::domain::intello::FlashcardSet;
use super::super::game_set::GameSetRepository;

/// Repository trait for flashcard set persistence operations.
/// 
/// This is a type alias to `GameSetRepository<FlashcardSet>` for backwards compatibility.
/// New code should use `GameSetRepository<FlashcardSet>` directly.
pub trait FlashcardRepository: GameSetRepository<FlashcardSet> {}

/// Blanket implementation: any type implementing `GameSetRepository<FlashcardSet>`
/// automatically implements FlashcardRepository.
impl<T: GameSetRepository<FlashcardSet>> FlashcardRepository for T {}

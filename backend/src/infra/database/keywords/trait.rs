//! Keywords Repository Trait - Abstraction for keyword set persistence
//!
//! This trait extends the generic `GameSetRepository` for keywords-specific operations.

use super::super::game_set::GameSetRepository;
use crate::services::intello::KeywordSet;

/// Repository trait for keyword set persistence operations.
///
/// This is a type alias to `GameSetRepository<KeywordSet>` for backwards compatibility.
/// New code should use `GameSetRepository<KeywordSet>` directly.
pub trait KeywordsRepository: GameSetRepository<KeywordSet> {}

/// Blanket implementation: any type implementing `GameSetRepository<KeywordSet>`
/// automatically implements KeywordsRepository.
impl<T: GameSetRepository<KeywordSet>> KeywordsRepository for T {}

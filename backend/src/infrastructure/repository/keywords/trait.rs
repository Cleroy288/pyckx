//! Keywords Repository Trait - Abstraction for keyword set persistence
//!
//! This trait extends the generic `GameSetRepository` for keywords-specific operations.

use crate::domain::intello::KeywordSet;
use super::super::game_set::GameSetRepository;

/// Repository trait for keyword set persistence operations.
/// 
/// This is a type alias to `GameSetRepository<KeywordSet>` for backwards compatibility.
/// New code should use `GameSetRepository<KeywordSet>` directly.
pub trait KeywordsRepository: GameSetRepository<KeywordSet> {}

/// Blanket implementation: any type implementing `GameSetRepository<KeywordSet>`
/// automatically implements KeywordsRepository.
impl<T: GameSetRepository<KeywordSet>> KeywordsRepository for T {}

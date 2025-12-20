//! True or False Repository Trait - Abstraction for true/false set persistence
//!
//! This trait extends the generic `GameSetRepository` for true/false-specific operations.

use crate::domain::intello::TrueOrFalseSet;
use super::super::game_set::GameSetRepository;

/// Repository trait for true/false set persistence operations.
/// 
/// This is a type alias to `GameSetRepository<TrueOrFalseSet>` for backwards compatibility.
/// New code should use `GameSetRepository<TrueOrFalseSet>` directly.
pub trait TrueOrFalseRepository: GameSetRepository<TrueOrFalseSet> {}

/// Blanket implementation: any type implementing GameSetRepository<TrueOrFalseSet>
/// automatically implements TrueOrFalseRepository.
impl<T: GameSetRepository<TrueOrFalseSet>> TrueOrFalseRepository for T {}

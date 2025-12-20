//! Fill Blank Repository Trait - Abstraction for fill blank set persistence
//!
//! This trait extends the generic `GameSetRepository` for fill blank-specific operations.

use crate::domain::intello::FillBlankSet;
use super::super::game_set::GameSetRepository;

/// Repository trait for fill blank set persistence operations.
/// 
/// This is a type alias to `GameSetRepository<FillBlankSet>` for backwards compatibility.
/// New code should use `GameSetRepository<FillBlankSet>` directly.
pub trait FillBlankRepository: GameSetRepository<FillBlankSet> {}

/// Blanket implementation: any type implementing GameSetRepository<FillBlankSet>
/// automatically implements FillBlankRepository.
impl<T: GameSetRepository<FillBlankSet>> FillBlankRepository for T {}

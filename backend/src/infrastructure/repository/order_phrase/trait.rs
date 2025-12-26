//! Order Phrase Repository Trait - Abstraction for order phrase set persistence
//!
//! This trait extends the generic `GameSetRepository` for order phrase-specific operations.

use crate::domain::intello::OrderPhraseSet;
use super::super::game_set::GameSetRepository;

/// Repository trait for order phrase set persistence operations.
/// 
/// This is a type alias to `GameSetRepository<OrderPhraseSet>` for backwards compatibility.
/// New code should use `GameSetRepository<OrderPhraseSet>` directly.
pub trait OrderPhraseRepository: GameSetRepository<OrderPhraseSet> {}

/// Blanket implementation: any type implementing `GameSetRepository<OrderPhraseSet>`
/// automatically implements OrderPhraseRepository.
impl<T: GameSetRepository<OrderPhraseSet>> OrderPhraseRepository for T {}

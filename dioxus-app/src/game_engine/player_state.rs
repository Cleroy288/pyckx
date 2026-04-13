//! Reactive state shared across game players.

use crate::game_engine::results::ResultItem;
use dioxus::prelude::*;

/// Reactive state shared by all game players.
#[derive(Clone, Copy)]
pub struct GamePlayerState {
    /// Total item count.
    pub total: usize,
    /// Current item index (0-based).
    pub current: Signal<usize>,
    /// Correct answer count.
    pub score: Signal<usize>,
    /// Whether the run is finished.
    pub finished: Signal<bool>,
    /// Per-question results.
    pub results: Signal<Vec<ResultItem>>,
}

impl GamePlayerState {
    /// Create fresh state for a run of `total` items.
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: use_signal(|| 0),
            score: use_signal(|| 0),
            finished: use_signal(|| false),
            results: use_signal(Vec::new),
        }
    }

    /// Record a result without advancing.
    pub fn record(
        &self,
        correct: bool,
        question: String,
        explanation: String,
    ) {
        let mut score = self.score;
        let mut results = self.results;
        if correct {
            *score.write() += 1;
        }
        results.write().push(ResultItem {
            question,
            correct,
            explanation,
        });
    }

    /// Advance to next item or mark the run finished.
    pub fn advance(&self) {
        let mut current = self.current;
        let mut finished = self.finished;
        let next = current() + 1;
        if next >= self.total {
            finished.set(true);
        } else {
            current.set(next);
        }
    }

    /// Reset shared signals back to the initial run.
    pub fn replay(&self) {
        let mut current = self.current;
        let mut score = self.score;
        let mut finished = self.finished;
        let mut results = self.results;
        current.set(0);
        score.set(0);
        finished.set(false);
        results.set(Vec::new());
    }
}

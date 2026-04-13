//! Shared state for all game players

use crate::components::game_shared::{
    game_results::ResultItem,
};
use dioxus::prelude::*;

/// Common player state shared across game types
#[derive(Clone, Copy)]
pub struct GamePlayerState {
    /// Total number of items
    pub total: usize,
    /// Current item index
    pub current: Signal<usize>,
    /// Correct answer count
    pub score: Signal<usize>,
    /// Whether the game is finished
    pub finished: Signal<bool>,
    /// Per-question results
    pub results: Signal<Vec<ResultItem>>,
}

impl GamePlayerState {
    /// Create a new player state
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: use_signal(|| 0),
            score: use_signal(|| 0),
            finished: use_signal(|| false),
            results: use_signal(Vec::new),
        }
    }

    /// Record a result (no advance)
    pub fn record_result(
        &self,
        correct: bool,
        question: String,
        explanation: String,
    ) {
        if correct {
            let mut score = self.score;
            *score.write() += 1;
        }
        let mut results = self.results;
        results.write().push(ResultItem {
            question,
            correct,
            explanation,
        });
    }

    /// Advance to next item or set finished
    pub fn advance_or_finish(&self) {
        let next = (self.current)() + 1;
        if next >= self.total {
            let mut finished = self.finished;
            finished.set(true);
        } else {
            let mut current = self.current;
            current.set(next);
        }
    }

    /// Record + advance in one step (flashcards)
    pub fn record_and_advance(
        &self,
        correct: bool,
        question: String,
        explanation: String,
    ) {
        self.record_result(
            correct, question, explanation,
        );
        self.advance_or_finish();
    }

    /// Reset common signals (caller resets
    /// game-specific signals separately)
    pub fn replay_base(&self) {
        let mut current = self.current;
        let mut score = self.score;
        let mut finished = self.finished;
        let mut results = self.results;
        current.set(0);
        score.set(0);
        finished.set(false);
        results.set(Vec::new());
    }

    /// Build a "next" closure that advances and
    /// resets game-specific signals via `reset`.
    pub fn make_next(
        &self,
        reset: impl Fn() + Copy + 'static,
    ) -> impl Fn() + Copy + 'static {
        let me = *self;
        move || {
            me.advance_or_finish();
            reset();
        }
    }

    /// Build a "replay" closure that replays and
    /// resets game-specific signals via `reset`.
    pub fn make_replay(
        &self,
        reset: impl Fn() + Copy + 'static,
    ) -> impl Fn() + Copy + 'static {
        let me = *self;
        move || {
            me.replay_base();
            reset();
        }
    }
}

// ** game_player_state.rs **
// ==> Shared state for all game players

use crate::components::intello::shared::game_results::ResultItem;
use leptos::prelude::*;

/// Common player state shared across game types
#[derive(Clone, Copy)]
pub struct GamePlayerState {
    /// Total number of items
    pub total: usize,
    /// Current item index
    pub current: RwSignal<usize>,
    /// Correct answer count
    pub score: RwSignal<usize>,
    /// Whether the game is finished
    pub finished: RwSignal<bool>,
    /// Per-question results
    pub results: RwSignal<Vec<ResultItem>>,
}

impl GamePlayerState {
    /// Create a new player state
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: RwSignal::new(0),
            score: RwSignal::new(0),
            finished: RwSignal::new(false),
            results: RwSignal::new(Vec::new()),
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
            self.score.update(|s| *s += 1);
        }
        self.results.update(|r| {
            r.push(ResultItem {
                question,
                correct,
                explanation,
            });
        });
    }

    /// Advance to next item or set finished
    pub fn advance_or_finish(&self) {
        let next = self.current.get() + 1;
        if next >= self.total {
            self.finished.set(true);
        } else {
            self.current.set(next);
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
        self.current.set(0);
        self.score.set(0);
        self.finished.set(false);
        self.results.set(Vec::new());
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

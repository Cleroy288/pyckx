//! Event handlers used by the keywords player.
//!
//! Each builder returns a small closure that captures the
//! reactive signals it needs. Keeping them in their own
//! file lets the player file stay focused on layout.

use crate::game_engine::GamePlayerState;
use crate::keywords::types::{
    is_answer_correct, KeywordQuestion,
};
use dioxus::prelude::*;

/// Toggle a keyword id in/out of `selected`. No-op once
/// the round is checked (answers are locked then).
pub fn make_toggle(
    checked: Signal<bool>,
    selected: Signal<Vec<String>>,
) -> impl Fn(String) + Copy + 'static {
    move |id: String| {
        if (checked)() {
            return;
        }
        let mut sel = selected;
        let mut writer = sel.write();
        if writer.contains(&id) {
            writer.retain(|v| v != &id);
        } else {
            writer.push(id);
        }
    }
}

/// Build the confirm handler: score the current question
/// against the selection and lock the round.
pub fn make_confirm(
    questions: Vec<KeywordQuestion>,
    state: GamePlayerState,
    selected: Signal<Vec<String>>,
    checked: Signal<bool>,
) -> impl FnMut() + 'static {
    let mut checked = checked;
    move || {
        let q = &questions[(state.current)()];
        let correct = is_answer_correct(q, &(selected)());
        state.record(
            correct,
            q.statement.clone(),
            q.explanation.clone(),
        );
        checked.set(true);
    }
}

/// Build the next handler: advance the engine and reset
/// the per-round local signals.
pub fn make_next(
    state: GamePlayerState,
    selected: Signal<Vec<String>>,
    checked: Signal<bool>,
) -> impl FnMut() + 'static {
    let mut sel = selected;
    let mut chk = checked;
    move || {
        state.advance();
        sel.set(Vec::new());
        chk.set(false);
    }
}

/// Pick the action button label for the current phase.
pub fn action_label(
    checked: Signal<bool>,
) -> &'static str {
    if (checked)() {
        "Next"
    } else {
        "Confirm"
    }
}

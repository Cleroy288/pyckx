//! Keywords player — pick the correct keywords in a
//! statement, confirm, then move to the next question.

use crate::game_engine::{
    derive_field, GamePlayerState, GameProgress,
    GameResults,
};
use crate::keywords::player_chips::render_chips;
use crate::keywords::player_handlers::{
    action_label, make_confirm, make_next, make_toggle,
};
use crate::keywords::types::{KeywordQuestion, KeywordSet};
use dioxus::prelude::*;

const PLAYER: &str =
    "flex flex-col gap-6 max-w-[600px] mx-auto items-center";

const STATEMENT: &str = "text-lg font-semibold \
     text-[var(--color-text-primary)] p-5 \
     bg-[var(--glass-bg)] backdrop-blur-[20px] \
     border border-[var(--color-border)] \
     text-center w-full";

const ACTION_BTN: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all hover:opacity-90";

/// Run the keywords game over a full set.
#[component]
pub fn KeywordsPlayer(
    set: KeywordSet,
    on_back: EventHandler<()>,
) -> Element {
    let questions = use_hook(|| set.questions.clone());
    let total = questions.len();
    let state = GamePlayerState::new(total);
    let selected: Signal<Vec<String>> = use_signal(Vec::new);
    let checked = use_signal(|| false);

    if (state.finished)() {
        return render_results(state, total, on_back);
    }
    render_active_round(questions, state, selected, checked)
}

/// Render the in-progress round: progress, statement,
/// chips, and the confirm/next button.
fn render_active_round(
    questions: Vec<KeywordQuestion>,
    state: GamePlayerState,
    selected: Signal<Vec<String>>,
    checked: Signal<bool>,
) -> Element {
    let statement = derive_field(
        questions.clone(),
        state.current,
        |q| q.statement.clone(),
    );
    let toggle = make_toggle(checked, selected);
    let mut on_confirm = make_confirm(
        questions.clone(),
        state,
        selected,
        checked,
    );
    let mut on_next = make_next(state, selected, checked);
    rsx! {
        div { class: PLAYER,
            GameProgress { current: state.current, total: state.total }
            p { class: STATEMENT, "{statement}" }
            div { class: "flex flex-wrap gap-2 justify-center",
                {render_chips(&questions, state.current, selected, toggle)}
            }
            button {
                class: ACTION_BTN,
                onclick: move |_| {
                    if (checked)() { on_next(); } else { on_confirm(); }
                },
                "{action_label(checked)}"
            }
        }
    }
}

/// Render the score screen at the end of a run.
fn render_results(
    state: GamePlayerState,
    total: usize,
    on_back: EventHandler<()>,
) -> Element {
    rsx! {
        GameResults {
            score: (state.score)(),
            total: total,
            items: (state.results)(),
            on_replay: move |_| state.replay(),
            on_back: move |_| on_back.call(()),
        }
    }
}

// ** fill_blank_player.rs **
// ==> Fill-the-blank game — phrase with blank, pick option

use crate::components::intello::shared::{
    game_helpers::derive_field,
    game_player_state::GamePlayerState,
    game_progress::GameProgress,
    game_results::GameResults,
};
use crate::domain::fill_blank_types::{
    FillBlankQuestion, FillBlankSet,
};
use dioxus::prelude::*;

// -- Tailwind class constants --

const PLAYER: &str =
    "flex flex-col gap-6 max-w-[600px] \
     mx-auto items-center";

const GLASS_PANEL: &str =
    "text-lg font-semibold \
     text-[var(--color-text-primary)] \
     p-5 bg-[var(--glass-bg)] \
     backdrop-blur-[20px] \
     border border-[var(--color-border)] \
     text-center w-full";

const SELECTABLE: &str =
    "px-5 py-2.5 text-sm font-medium \
     bg-[var(--primary-tint-subtle)] \
     border border-[var(--color-border)] \
     text-[var(--color-text-primary)] \
     cursor-pointer transition-all \
     hover:border-[var(--color-primary)] \
     hover:bg-[var(--primary-tint-light)] \
     disabled:cursor-default disabled:opacity-70";

const ACTION_BTN: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all \
     hover:opacity-90";

/// Fill blank game player
#[component]
pub fn FillBlankPlayer(
    set: FillBlankSet,
    on_back: EventHandler<()>,
) -> Element {
    let total = set.questions.len();
    let questions =
        use_hook(|| set.questions.clone());
    let state = GamePlayerState::new(total);
    let answered = use_signal(|| false);

    let phrase_text = derive_field(
        questions.clone(),
        state.current,
        |q| q.phrase.clone(),
    );

    let qs_c = questions.clone();
    let check =
        move |_oid: String, is_correct: bool| {
            if (answered)() {
                return;
            }
            let mut a = answered;
            a.set(true);
            let q = &qs_c[(state.current)()];
            state.record_result(
                is_correct,
                q.phrase.clone(),
                q.explanation.clone(),
            );
        };

    let reset = move || {
        let mut a = answered;
        a.set(false);
    };
    let next = state.make_next(reset);
    let replay = state.make_replay(reset);

    if (state.finished)() {
        return rsx! {
            GameResults {
                score: (state.score)(),
                total: total,
                items: (state.results)(),
                on_replay: move |_| replay(),
                on_back: move |_| on_back.call(()),
            }
        };
    }

    rsx! {
        div { class: PLAYER,
            GameProgress {
                current: state.current,
                total: total,
            }
            div { class: GLASS_PANEL,
                "{phrase_text}"
            }
            div {
                class: "flex flex-wrap gap-2 \
                    justify-center",
                {render_options(
                    &questions,
                    state.current,
                    answered,
                    check,
                )}
            }
            if (answered)() {
                button {
                    class: ACTION_BTN,
                    onclick: move |_| next(),
                    "Next"
                }
            }
        }
    }
}

/// Render fill blank option buttons
fn render_options(
    questions: &[FillBlankQuestion],
    current: Signal<usize>,
    answered: Signal<bool>,
    check: impl Fn(String, bool) + Clone + 'static,
) -> Element {
    let Some(q) = questions.get((current)()) else {
        return rsx! {};
    };

    rsx! {
        for opt in q.options.iter() {
            {render_option_btn(
                opt.id.clone(),
                opt.text.clone(),
                opt.is_correct,
                answered,
                check.clone(),
            )}
        }
    }
}

/// Render a single option button
fn render_option_btn(
    oid: String,
    text: String,
    correct: bool,
    answered: Signal<bool>,
    check: impl Fn(String, bool) + Clone + 'static,
) -> Element {
    rsx! {
        button {
            class: SELECTABLE,
            disabled: (answered)(),
            onclick: move |_| {
                check(oid.clone(), correct)
            },
            "{text}"
        }
    }
}

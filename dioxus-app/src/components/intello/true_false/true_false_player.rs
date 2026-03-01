// ** true_false_player.rs **
// ==> True/False game — statement + True/False buttons

use crate::components::intello::shared::{
    game_helpers::derive_field,
    game_player_state::GamePlayerState,
    game_progress::GameProgress,
    game_results::GameResults,
};
use crate::domain::true_false_types::TrueOrFalseSet;
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

const SUCCESS_BTN: &str =
    "px-8 py-3.5 text-base font-semibold \
     cursor-pointer transition-all \
     bg-[var(--color-success-bg)] \
     text-[var(--color-success-text)] \
     border border-[var(--color-success-border)] \
     hover:bg-[var(--color-success-border)] \
     disabled:opacity-60 disabled:cursor-default";

const ERROR_BTN: &str =
    "px-8 py-3.5 text-base font-semibold \
     cursor-pointer transition-all \
     bg-[var(--color-error-bg)] \
     text-[var(--color-error-text)] \
     border border-[var(--color-error-border)] \
     hover:bg-[var(--color-error-border)] \
     disabled:opacity-60 disabled:cursor-default";

const ACTION_BTN: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all \
     hover:opacity-90";

/// True/False game player
#[component]
pub fn TrueFalsePlayer(
    set: TrueOrFalseSet,
    on_back: EventHandler<()>,
) -> Element {
    let total = set.statements.len();
    let stmts =
        use_hook(|| set.statements.clone());
    let state = GamePlayerState::new(total);
    let answered = use_signal(|| false);

    let statement_text = derive_field(
        stmts.clone(),
        state.current,
        |s| s.statement.clone(),
    );

    let stmts_c = stmts.clone();
    let check = move |answer: bool| {
        if (answered)() {
            return;
        }
        let mut a = answered;
        a.set(true);
        let s = &stmts_c[(state.current)()];
        state.record_result(
            answer == s.answer,
            s.statement.clone(),
            s.explanation.clone(),
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
                "{statement_text}"
            }
            div { class: "flex gap-4",
                button {
                    class: SUCCESS_BTN,
                    disabled: (answered)(),
                    onclick: {
                        let check = check.clone();
                        move |_| check(true)
                    },
                    "True"
                }
                button {
                    class: ERROR_BTN,
                    disabled: (answered)(),
                    onclick: move |_| check(false),
                    "False"
                }
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

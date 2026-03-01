// ** qcm_player.rs **
// ==> QCM game player — one question at a time

use crate::components::intello::shared::{
    game_helpers::{derive_field, shuffle_deterministic},
    game_player_state::GamePlayerState,
    game_progress::GameProgress,
    game_results::GameResults,
};
use crate::domain::qcm_types::{QcmQuestion, QcmSet};
use dioxus::prelude::*;

// -- Tailwind class constants for shared styles --

/// Player layout container
const PLAYER: &str =
    "flex flex-col gap-6 max-w-[600px] \
     mx-auto items-center";

/// Glass panel for question text
const GLASS_PANEL: &str =
    "text-lg font-semibold \
     text-[var(--color-text-primary)] \
     p-5 bg-[var(--glass-bg)] \
     backdrop-blur-[20px] \
     border border-[var(--color-border)] \
     text-center w-full";

/// Selectable option button (base)
const SELECTABLE: &str =
    "px-5 py-2.5 text-sm font-medium \
     bg-[var(--primary-tint-subtle)] \
     border border-[var(--color-border)] \
     text-[var(--color-text-primary)] \
     cursor-pointer transition-all \
     hover:border-[var(--color-primary)] \
     hover:bg-[var(--primary-tint-light)] \
     disabled:cursor-default disabled:opacity-70";

/// Correct answer highlight
const CORRECT: &str =
    "!border-[var(--color-success)] \
     !bg-[var(--color-success-bg)] \
     !text-[var(--color-success-text)]";

/// Wrong answer highlight
const WRONG: &str =
    "!border-[var(--color-error)] \
     !bg-[var(--color-error-bg)] \
     !text-[var(--color-error-text)]";

/// Feedback explanation block
const FEEDBACK: &str =
    "p-4 text-sm leading-relaxed \
     text-[var(--color-text-secondary)] \
     bg-[var(--glass-bg)] \
     border-l-[3px] border-[var(--color-primary)]";

/// Action button (next/confirm)
const ACTION_BTN: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all \
     hover:opacity-90";

/// QCM game player component
#[component]
pub fn QcmPlayer(
    /// The QCM set to play
    set: QcmSet,
    /// Called when user finishes and goes back
    on_back: EventHandler<()>,
) -> Element {
    let total = set.questions.len();
    let questions =
        use_hook(|| set.questions.clone());
    let state = GamePlayerState::new(total);
    let selected: Signal<Option<String>> =
        use_signal(|| None);
    let answered = use_signal(|| false);

    let right = derive_field(
        questions.clone(),
        state.current,
        |q| q.right_answer.clone(),
    );
    let question_text = derive_field(
        questions.clone(),
        state.current,
        |q| q.question.clone(),
    );
    let explanation = derive_field(
        questions.clone(),
        state.current,
        |q| q.explanation.clone(),
    );
    let answers = derive_answers(
        questions.clone(),
        state.current,
    );

    let reset = move || {
        let mut s = selected;
        let mut a = answered;
        s.set(None);
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
                "{question_text}"
            }
            div { class: "flex flex-col gap-2",
                for a in (answers)().iter() {
                    {render_option(
                        a.clone(),
                        selected,
                        answered,
                        right,
                        state,
                        questions.clone(),
                    )}
                }
            }
            if (answered)() {
                div { class: FEEDBACK,
                    "{explanation}"
                }
                button {
                    class: ACTION_BTN,
                    onclick: move |_| next(),
                    "Next"
                }
            }
        }
    }
}

/// Derive shuffled answer options
fn derive_answers(
    questions: Vec<QcmQuestion>,
    current: Signal<usize>,
) -> Memo<Vec<String>> {
    let qs = use_hook(|| questions);
    use_memo(move || {
        let Some(q) = qs.get((current)()) else {
            return Vec::new();
        };
        let mut opts = q.wrong_answers.clone();
        opts.push(q.right_answer.clone());
        shuffle_deterministic(&mut opts);
        opts
    })
}

/// Render a single answer option button
fn render_option(
    answer: String,
    selected: Signal<Option<String>>,
    answered: Signal<bool>,
    right: Memo<String>,
    state: GamePlayerState,
    questions: Vec<QcmQuestion>,
) -> Element {
    let a = answer.clone();
    let a2 = answer.clone();

    let class = use_memo(move || {
        option_class(
            &a,
            &(selected)(),
            (answered)(),
            &(right)(),
        )
    });

    rsx! {
        button {
            class: "{class}",
            disabled: (answered)(),
            onclick: move |_| {
                if (answered)() {
                    return;
                }
                let ans = a2.clone();
                let mut s = selected;
                let mut a = answered;
                s.set(Some(ans.clone()));
                a.set(true);
                let q = &questions
                    [(state.current)()];
                state.record_result(
                    ans == q.right_answer,
                    q.question.clone(),
                    q.explanation.clone(),
                );
            },
            "{answer}"
        }
    }
}

/// Compute CSS class for an answer option
fn option_class(
    answer: &str,
    selected: &Option<String>,
    answered: bool,
    right: &str,
) -> String {
    if !answered {
        return SELECTABLE.to_string();
    }
    let is_right = answer == right;
    let is_sel =
        selected.as_deref() == Some(answer);
    if is_right {
        format!("{} {}", SELECTABLE, CORRECT)
    } else if is_sel {
        format!("{} {}", SELECTABLE, WRONG)
    } else {
        SELECTABLE.to_string()
    }
}

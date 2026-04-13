//! QCM game player — one question at a time

use crate::components::game_shared::{
    game_helpers::{derive_field, shuffle_deterministic},
    game_player_state::GamePlayerState,
    game_progress::GameProgress,
    game_results::GameResults,
};
use crate::domain::qcm_types::{QcmQuestion, QcmSet};
use dioxus::prelude::*;

/// Player layout container
const PLAYER: &str =
    "flex flex-col gap-6 max-w-[700px] \
     mx-auto w-full";

/// Card panel for question text
const QUESTION_CARD: &str =
    "text-lg font-semibold \
     text-[var(--color-text-primary)] \
     p-6 bg-[var(--card)] \
     border-2 border-[var(--foreground)] \
     rounded-[12px] text-center w-full";

/// Selectable option button (base)
const SELECTABLE: &str =
    "w-full text-left px-5 py-3 \
     text-sm font-medium rounded-[12px] \
     bg-[var(--card)] \
     border-[1.5px] border-[var(--color-border)] \
     text-[var(--color-text-primary)] \
     cursor-pointer \
     transition-[transform,border-color] \
     duration-150 \
     hover:border-[var(--color-primary)] \
     hover:-translate-y-0.5 \
     disabled:cursor-default \
     disabled:opacity-70 \
     disabled:hover:translate-y-0";

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
    "p-5 text-sm leading-relaxed \
     text-[var(--color-text-secondary)] \
     bg-[var(--card)] rounded-[12px] \
     border-[1.5px] border-[var(--color-border)] \
     border-l-[3px] \
     border-l-[var(--color-primary)]";

/// Action button (next/confirm)
const ACTION_BTN: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold \
     text-[var(--color-background)] \
     bg-[var(--color-primary)] rounded-[12px] \
     border-[1.5px] \
     border-[var(--color-primary)] \
     cursor-pointer \
     transition-[transform] duration-150 \
     hover:-translate-y-0.5 \
     active:translate-y-0";

/// Reactive state for the QCM player
struct QcmState {
    questions: Vec<QcmQuestion>,
    state: GamePlayerState,
    selected: Signal<Option<String>>,
    answered: Signal<bool>,
}

/// Derived memos for the current question
struct QuestionMemos {
    right: Memo<String>,
    text: Memo<String>,
    explanation: Memo<String>,
    answers: Memo<Vec<String>>,
}

/// Initialize player reactive state
fn use_qcm_state(set: &QcmSet) -> QcmState {
    let total = set.questions.len();
    QcmState {
        questions: use_hook(|| {
            set.questions.clone()
        }),
        state: GamePlayerState::new(total),
        selected: use_signal(|| None),
        answered: use_signal(|| false),
    }
}

/// Derive memos from current question index
fn use_question_memos(
    qs: &[QcmQuestion],
    current: Signal<usize>,
) -> QuestionMemos {
    let q = qs.to_vec();
    QuestionMemos {
        right: derive_field(
            q.clone(), current,
            |q| q.right_answer.clone(),
        ),
        text: derive_field(
            q.clone(), current,
            |q| q.question.clone(),
        ),
        explanation: derive_field(
            q.clone(), current,
            |q| q.explanation.clone(),
        ),
        answers: derive_answers(q, current),
    }
}

/// QCM game player component
#[component]
pub fn QcmPlayer(
    /// The QCM set to play
    set: QcmSet,
    /// Called when user goes back
    on_back: EventHandler<()>,
) -> Element {
    let s = use_qcm_state(&set);
    let m = use_question_memos(
        &s.questions, s.state.current,
    );
    let sel = s.selected;
    let ans = s.answered;

    let reset = move || {
        let mut sl = sel;
        let mut an = ans;
        sl.set(None);
        an.set(false);
    };
    let next = s.state.make_next(reset);
    let replay = s.state.make_replay(reset);

    if (s.state.finished)() {
        return render_results(
            &s.state, set.questions.len(),
            replay, on_back,
        );
    }

    render_play(&s, &m, next)
}

/// Render results screen
fn render_results(
    state: &GamePlayerState,
    total: usize,
    replay: impl Fn() + 'static,
    on_back: EventHandler<()>,
) -> Element {
    rsx! {
        GameResults {
            score: (state.score)(),
            total: total,
            items: (state.results)(),
            on_replay: move |_| replay(),
            on_back: move |_| on_back.call(()),
        }
    }
}

/// Render active play view
fn render_play(
    s: &QcmState,
    m: &QuestionMemos,
    next: impl Fn() + 'static,
) -> Element {
    let total = s.questions.len();

    rsx! {
        div { class: PLAYER,
            GameProgress {
                current: s.state.current,
                total: total,
            }
            div { class: QUESTION_CARD,
                "{m.text}"
            }
            div {
                class: "flex flex-col gap-3 w-full",
                for (idx, a) in
                    (m.answers)().iter().enumerate()
                {
                    div { key: "{idx}",
                        {render_option(
                            a.clone(), s, &m.right,
                        )}
                    }
                }
            }
            if (s.answered)() {
                div { class: FEEDBACK,
                    "{m.explanation}"
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
    ctx: &QcmState,
    right: &Memo<String>,
) -> Element {
    let a = answer.clone();
    let selected = ctx.selected;
    let answered = ctx.answered;
    let right = *right;
    let state = ctx.state;
    let questions = ctx.questions.clone();

    let class = use_memo(move || {
        option_class(
            &a, &(selected)(),
            (answered)(), &(right)(),
        )
    });

    rsx! {
        button {
            class: "{class}",
            disabled: (answered)(),
            onclick: move |_| {
                handle_answer(
                    &answer, selected, answered,
                    state, &questions,
                )
            },
            "{answer}"
        }
    }
}

/// Record user's answer selection
fn handle_answer(
    answer: &str,
    mut selected: Signal<Option<String>>,
    mut answered: Signal<bool>,
    state: GamePlayerState,
    questions: &[QcmQuestion],
) {
    if (answered)() {
        return;
    }
    selected.set(Some(answer.to_string()));
    answered.set(true);
    let q = &questions[(state.current)()];
    state.record_result(
        answer == q.right_answer,
        q.question.clone(),
        q.explanation.clone(),
    );
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

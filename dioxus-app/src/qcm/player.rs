//! QCM player — one question at a time, with feedback.

use dioxus::prelude::*;

use crate::game_engine::{
    derive_field, shuffle, GamePlayerState,
    GameProgress, GameResults,
};
use crate::qcm::player_option::{render_option, OptionCtx};
use crate::qcm::types::{QcmQuestion, QcmSet};

/// Outer player layout (column, centered, max width).
const PLAYER_CLS: &str =
    "flex flex-col gap-6 max-w-[700px] \
     mx-auto w-full";

/// Card containing the current question text.
const QUESTION_CLS: &str =
    "text-lg font-semibold \
     text-[var(--color-text-primary)] \
     p-6 bg-[var(--card)] \
     border-2 border-[var(--foreground)] \
     rounded-[12px] text-center w-full";

/// Explanation banner shown after answering.
const FEEDBACK_CLS: &str =
    "p-5 text-sm leading-relaxed \
     text-[var(--color-text-secondary)] \
     bg-[var(--card)] rounded-[12px] \
     border-[1.5px] border-[var(--color-border)] \
     border-l-[3px] \
     border-l-[var(--color-primary)]";

/// "Next" button to advance to the next question.
const NEXT_BTN_CLS: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold \
     text-[var(--color-background)] \
     bg-[var(--color-primary)] rounded-[12px] \
     border-[1.5px] border-[var(--color-primary)] \
     cursor-pointer transition-[transform] \
     duration-150 hover:-translate-y-0.5 \
     active:translate-y-0";

/// QCM player component — drives a full QCM run.
#[component]
pub fn QcmPlayer(
    /// Set of questions to play.
    set: QcmSet,
    /// Called when the user wants to leave the player.
    on_back: EventHandler<()>,
) -> Element {
    let total = set.questions.len();
    let questions = use_hook(|| set.questions.clone());
    let state = GamePlayerState::new(total);
    let selected: Signal<Option<String>> =
        use_signal(|| None);
    let answered = use_signal(|| false);
    let memos = derive_memos(
        questions.clone(), state.current,
    );

    if (state.finished)() {
        return render_results(state, total, on_back);
    }
    render_play(PlayCtx {
        state,
        questions,
        selected,
        answered,
        memos,
    })
}

/// Bundle of reactive context for the play view.
#[derive(Clone)]
struct PlayCtx {
    state: GamePlayerState,
    questions: Vec<QcmQuestion>,
    selected: Signal<Option<String>>,
    answered: Signal<bool>,
    memos: QuestionMemos,
}

/// Derived memos for the current question.
#[derive(Clone, Copy)]
struct QuestionMemos {
    text: Memo<String>,
    explanation: Memo<String>,
    right: Memo<String>,
    answers: Memo<Vec<String>>,
}

/// Build every per-question memo from the set + index.
fn derive_memos(
    questions: Vec<QcmQuestion>,
    current: Signal<usize>,
) -> QuestionMemos {
    let q = questions.clone();
    QuestionMemos {
        text: derive_field(q.clone(), current, |q| {
            q.question.clone()
        }),
        explanation: derive_field(
            q.clone(), current,
            |q| q.explanation.clone(),
        ),
        right: derive_field(q.clone(), current, |q| {
            q.right_answer.clone()
        }),
        answers: derive_answers(q, current),
    }
}

/// Memoize the shuffled answer list for the current Q.
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
        shuffle(&mut opts);
        opts
    })
}

/// Render the end-of-run results screen.
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

/// Render the active play view (question + options).
fn render_play(ctx: PlayCtx) -> Element {
    let total = ctx.questions.len();
    let state = ctx.state;
    let answered = ctx.answered;
    let selected = ctx.selected;
    let memos = ctx.memos;
    let options = render_option_list(&ctx);
    let next = move |_| advance(state, selected, answered);
    rsx! {
        div { class: PLAYER_CLS,
            GameProgress {
                current: state.current,
                total: total,
            }
            div { class: QUESTION_CLS, "{memos.text}" }
            div { class: "flex flex-col gap-3 w-full",
                {options}
            }
            if (answered)() {
                div { class: FEEDBACK_CLS,
                    "{memos.explanation}"
                }
                button {
                    class: NEXT_BTN_CLS,
                    onclick: next,
                    "Next"
                }
            }
        }
    }
}

/// Advance to the next question, resetting per-Q signals.
fn advance(
    state: GamePlayerState,
    mut selected: Signal<Option<String>>,
    mut answered: Signal<bool>,
) {
    selected.set(None);
    answered.set(false);
    state.advance();
}

/// Render every shuffled option as a clickable button.
fn render_option_list(ctx: &PlayCtx) -> Element {
    let answers = (ctx.memos.answers)();
    let option_ctx = OptionCtx {
        sel: ctx.selected,
        ans: ctx.answered,
        right: ctx.memos.right,
        state: ctx.state,
        questions: ctx.questions.clone(),
    };
    rsx! {
        for (idx, answer) in answers.iter().enumerate() {
            div { key: "{idx}",
                {render_option(answer.clone(), option_ctx.clone())}
            }
        }
    }
}

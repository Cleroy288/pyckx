//! Fill-the-blank player — show a phrase, pick the right
//! word among options, advance to the next question.

use crate::fill_blanks::types::{
    FillBlankQuestion, FillBlankSet,
};
use crate::game_engine::{
    derive_field, GamePlayerState, GameProgress,
    GameResults,
};
use dioxus::prelude::*;

/// Outer container layout.
const PLAYER_CLASS: &str =
    "flex flex-col gap-6 max-w-[600px] \
     mx-auto items-center";

/// Glass card showing the phrase with its blank.
const PHRASE_CLASS: &str =
    "text-lg font-semibold \
     text-[var(--color-text-primary)] \
     p-5 bg-[var(--glass-bg)] \
     backdrop-blur-[20px] \
     border border-[var(--color-border)] \
     text-center w-full";

/// Pickable option button.
const OPTION_CLASS: &str =
    "px-5 py-2.5 text-sm font-medium \
     bg-[var(--primary-tint-subtle)] \
     border border-[var(--color-border)] \
     text-[var(--color-text-primary)] \
     cursor-pointer transition-all \
     hover:border-[var(--color-primary)] \
     hover:bg-[var(--primary-tint-light)] \
     disabled:cursor-default disabled:opacity-70";

/// Options row layout.
const OPTIONS_ROW_CLASS: &str =
    "flex flex-wrap gap-2 justify-center";

/// "Next" call-to-action shown after answering.
const NEXT_CLASS: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all \
     hover:opacity-90";

/// Fill-blank player for a single set.
#[component]
pub fn FillBlankPlayer(
    /// Set being played.
    set: FillBlankSet,
    /// Called when the player wants to leave.
    on_back: EventHandler<()>,
) -> Element {
    let total = set.questions.len();
    let questions: Signal<Vec<FillBlankQuestion>> =
        use_signal(|| set.questions.clone());
    let state = GamePlayerState::new(total);
    let answered = use_signal(|| false);
    let phrase = derive_field(
        set.questions.clone(),
        state.current,
        |q| q.phrase.clone(),
    );

    if (state.finished)() {
        return final_screen(state, total, on_back, answered);
    }
    in_progress_view(questions, phrase, state, answered)
}

/// Render the post-run results screen.
fn final_screen(
    state: GamePlayerState,
    total: usize,
    on_back: EventHandler<()>,
    mut answered: Signal<bool>,
) -> Element {
    let on_replay = move |_| {
        state.replay();
        answered.set(false);
    };
    rsx! {
        GameResults {
            score: (state.score)(),
            total: total,
            items: (state.results)(),
            on_replay: on_replay,
            on_back: move |_| on_back.call(()),
        }
    }
}

/// Render the in-progress question view.
fn in_progress_view(
    questions: Signal<Vec<FillBlankQuestion>>,
    phrase: Memo<String>,
    state: GamePlayerState,
    mut answered: Signal<bool>,
) -> Element {
    let total = questions.read().len();
    let on_next = move |_| {
        state.advance();
        answered.set(false);
    };
    rsx! {
        div { class: PLAYER_CLASS,
            GameProgress { current: state.current, total: total }
            div { class: PHRASE_CLASS, "{phrase}" }
            { options_row(questions, state, answered) }
            if (answered)() {
                button {
                    class: NEXT_CLASS,
                    onclick: on_next,
                    "Next"
                }
            }
        }
    }
}

/// Render the row of selectable options.
fn options_row(
    questions: Signal<Vec<FillBlankQuestion>>,
    state: GamePlayerState,
    answered: Signal<bool>,
) -> Element {
    let qs = questions.read();
    let Some(q) = qs.get((state.current)()) else {
        return rsx! {};
    };
    let options = q.options.clone();
    rsx! {
        div { class: OPTIONS_ROW_CLASS,
            for opt in options.into_iter() {
                {render_option(opt, questions, state, answered)}
            }
        }
    }
}

/// Render a single option button wired to `record_answer`.
fn render_option(
    opt: crate::fill_blanks::types::FillBlankOption,
    questions: Signal<Vec<FillBlankQuestion>>,
    state: GamePlayerState,
    mut answered: Signal<bool>,
) -> Element {
    let is_correct = opt.is_correct;
    let on_pick = move |_| {
        if (answered)() {
            return;
        }
        answered.set(true);
        let qs = questions.read();
        let q = &qs[(state.current)()];
        state.record(
            is_correct,
            q.phrase.clone(),
            q.explanation.clone(),
        );
    };
    rsx! {
        div { key: "{opt.id}",
            button {
                class: OPTION_CLASS,
                disabled: (answered)(),
                onclick: on_pick,
                "{opt.text}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fill_blanks::types::FillBlankOption;

    fn opt(id: &str, text: &str, ok: bool) -> FillBlankOption {
        FillBlankOption {
            id: id.into(),
            text: text.into(),
            is_correct: ok,
        }
    }

    fn question(id: &str) -> FillBlankQuestion {
        FillBlankQuestion {
            id: id.into(),
            phrase: format!("phrase-{id}"),
            options: vec![
                opt("a", "first", true),
                opt("b", "second", false),
            ],
            explanation: format!("why-{id}"),
        }
    }

    #[test]
    fn test_question_factory_keeps_two_options() {
        let q = question("q1");
        assert_eq!(q.options.len(), 2);
    }

    #[test]
    fn test_question_factory_first_option_is_correct() {
        let q = question("q1");
        assert!(q.options[0].is_correct);
    }

    #[test]
    fn test_question_factory_second_option_is_wrong() {
        let q = question("q1");
        assert!(!q.options[1].is_correct);
    }

    #[test]
    fn test_question_factory_phrase_includes_id() {
        let q = question("xyz");
        assert_eq!(q.phrase, "phrase-xyz");
    }

    #[test]
    fn test_question_factory_explanation_includes_id() {
        let q = question("xyz");
        assert_eq!(q.explanation, "why-xyz");
    }
}

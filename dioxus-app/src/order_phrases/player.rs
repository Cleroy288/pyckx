//! Order Phrases — interactive player. The user must
//! reorder the shuffled words back into the original
//! sentence; we record correctness, then advance.

use dioxus::prelude::*;

use crate::game_engine::{
    derive_field, shuffle, GamePlayerState, GameProgress,
    GameResults,
};

use super::types::{
    is_phrase_correct, join_words, toggle_placed,
    OrderPhraseQuestion, OrderPhraseSet,
};

/// Player layout container.
const PLAYER_CLS: &str =
    "flex flex-col gap-6 max-w-[600px] \
     mx-auto items-center";

/// Hint paragraph shown above the words.
const HINT_CLS: &str =
    "text-[0.9375rem] \
     text-[var(--color-text-secondary)] \
     text-center italic m-0";

/// Current "user phrase" preview area.
const PREVIEW_CLS: &str =
    "min-h-[3rem] w-full p-4 \
     bg-[var(--glass-bg)] backdrop-blur-[20px] \
     border border-[var(--color-border)] \
     text-base \
     text-[var(--color-text-primary)] text-center";

/// Word list container.
const WORDS_CLS: &str =
    "flex flex-wrap gap-2 justify-center";

/// Single selectable word button.
const WORD_BTN_CLS: &str =
    "px-5 py-2.5 text-sm font-medium \
     bg-[var(--primary-tint-subtle)] \
     border border-[var(--color-border)] \
     text-[var(--color-text-primary)] \
     cursor-pointer transition-all \
     hover:border-[var(--color-primary)] \
     hover:bg-[var(--primary-tint-light)] \
     disabled:cursor-default disabled:opacity-70";

/// Class added when a word is already placed.
const WORD_USED_CLS: &str = "opacity-40 line-through";

/// Primary "Check / Next" action button.
const ACTION_BTN_CLS: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all hover:opacity-90";

/// Order-phrase player: drives a full run of a set.
#[component]
pub fn OrderPhrasePlayer(
    /// Set of questions to play.
    set: OrderPhraseSet,
    /// Called when the user wants to leave the player.
    on_back: EventHandler<()>,
) -> Element {
    let total = set.questions.len();
    let questions = use_hook(|| set.questions.clone());
    let state = GamePlayerState::new(total);
    let placed: Signal<Vec<String>> = use_signal(Vec::new);
    let checked = use_signal(|| false);

    if (state.finished)() {
        return render_results(state, total, on_back);
    }
    render_play(PlayCtx {
        state,
        questions,
        placed,
        checked,
    })
}

/// Bundle of reactive context for the play view.
struct PlayCtx {
    state: GamePlayerState,
    questions: Vec<OrderPhraseQuestion>,
    placed: Signal<Vec<String>>,
    checked: Signal<bool>,
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

/// Render the active play view (hint + words + action).
fn render_play(ctx: PlayCtx) -> Element {
    let total = ctx.questions.len();
    let hint = derive_field(
        ctx.questions.clone(),
        ctx.state.current,
        |q| q.hint.clone(),
    );
    let placed = ctx.placed;

    rsx! {
        div { class: PLAYER_CLS,
            GameProgress {
                current: ctx.state.current,
                total: total,
            }
            p { class: HINT_CLS, "{hint}" }
            div { class: PREVIEW_CLS,
                "{join_words(&(placed)())}"
            }
            div { class: WORDS_CLS, {render_words(&ctx)} }
            {render_action(&ctx)}
        }
    }
}

/// Render the shuffled word buttons for the current Q.
fn render_words(ctx: &PlayCtx) -> Element {
    let words = shuffled_words(ctx);
    let placed = ctx.placed;
    let checked = ctx.checked;
    rsx! {
        for (idx, word) in words.iter().enumerate() {
            div { key: "{idx}",
                {render_word_btn(word.clone(), placed, checked)}
            }
        }
    }
}

/// Compute the shuffled word list for the current Q.
fn shuffled_words(ctx: &PlayCtx) -> Vec<String> {
    let Some(q) = ctx.questions.get((ctx.state.current)())
    else {
        return Vec::new();
    };
    let mut words: Vec<String> =
        q.words.iter().map(|w| w.word.clone()).collect();
    shuffle(&mut words);
    words
}

/// Render a single selectable word button.
fn render_word_btn(
    word: String,
    placed: Signal<Vec<String>>,
    checked: Signal<bool>,
) -> Element {
    let class_value =
        word_btn_class(word.clone(), placed);
    let on_click = {
        let word = word.clone();
        move |_| toggle_word(&word, placed, checked)
    };
    rsx! {
        button {
            class: "{class_value}",
            disabled: (checked)(),
            onclick: on_click,
            "{word}"
        }
    }
}

/// Reactive class for a word button (placed vs available).
fn word_btn_class(
    word: String,
    placed: Signal<Vec<String>>,
) -> Memo<String> {
    use_memo(move || {
        if (placed)().iter().any(|w| w == &word) {
            format!("{WORD_BTN_CLS} {WORD_USED_CLS}")
        } else {
            WORD_BTN_CLS.to_string()
        }
    })
}

/// Toggle a word in/out of the placed list.
fn toggle_word(
    word: &str,
    mut placed: Signal<Vec<String>>,
    checked: Signal<bool>,
) {
    if (checked)() {
        return;
    }
    let next = toggle_placed(&placed(), word);
    placed.set(next);
}

/// Render the "Check" or "Next" action button.
fn render_action(ctx: &PlayCtx) -> Element {
    let state = ctx.state;
    let placed = ctx.placed;
    let checked = ctx.checked;
    let questions = ctx.questions.clone();

    if (checked)() {
        rsx! {
            button {
                class: ACTION_BTN_CLS,
                onclick: move |_| advance(
                    state, placed, checked,
                ),
                "Next"
            }
        }
    } else {
        let qs = questions.clone();
        rsx! {
            button {
                class: ACTION_BTN_CLS,
                onclick: move |_| confirm(
                    state, placed, checked, &qs,
                ),
                "Check"
            }
        }
    }
}

/// Lock in the user's phrase and record the result.
fn confirm(
    state: GamePlayerState,
    placed: Signal<Vec<String>>,
    mut checked: Signal<bool>,
    questions: &[OrderPhraseQuestion],
) {
    checked.set(true);
    let user_phrase = join_words(&(placed)());
    let q = &questions[(state.current)()];
    state.record(
        is_phrase_correct(&user_phrase, &q.original_phrase),
        q.hint.clone(),
        q.original_phrase.clone(),
    );
}

/// Advance to the next question, resetting per-Q signals.
fn advance(
    state: GamePlayerState,
    mut placed: Signal<Vec<String>>,
    mut checked: Signal<bool>,
) {
    placed.set(Vec::new());
    checked.set(false);
    state.advance();
}

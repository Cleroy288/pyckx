// ** order_phrase_player.rs **
// ==> Reorder shuffled words into correct phrase

use crate::components::intello::shared::{
    game_helpers::{derive_field, shuffle_deterministic},
    game_player_state::GamePlayerState,
    game_progress::GameProgress,
    game_results::GameResults,
};
use crate::domain::order_phrase_types::{
    OrderPhraseQuestion, OrderPhraseSet,
};
use dioxus::prelude::*;

// -- Tailwind class constants --

const PLAYER: &str =
    "flex flex-col gap-6 max-w-[600px] \
     mx-auto items-center";

const SELECTABLE: &str =
    "px-5 py-2.5 text-sm font-medium \
     bg-[var(--primary-tint-subtle)] \
     border border-[var(--color-border)] \
     text-[var(--color-text-primary)] \
     cursor-pointer transition-all \
     hover:border-[var(--color-primary)] \
     hover:bg-[var(--primary-tint-light)] \
     disabled:cursor-default disabled:opacity-70";

const WORD_USED: &str =
    "opacity-40 line-through";

const ACTION_BTN: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all \
     hover:opacity-90";

/// Order phrase game player
#[component]
pub fn OrderPhrasePlayer(
    set: OrderPhraseSet,
    on_back: EventHandler<()>,
) -> Element {
    let total = set.questions.len();
    let questions =
        use_hook(|| set.questions.clone());
    let state = GamePlayerState::new(total);
    let placed: Signal<Vec<String>> =
        use_signal(Vec::new);
    let checked = use_signal(|| false);

    let hint_text = derive_field(
        questions.clone(),
        state.current,
        |q| q.hint.clone(),
    );

    let toggle_word = move |word: String| {
        if (checked)() {
            return;
        }
        let mut p = placed;
        let mut w = p.write();
        if w.contains(&word) {
            w.retain(|v| v != &word);
        } else {
            w.push(word);
        }
    };

    let qs_c = questions.clone();
    let confirm = move || {
        let mut c = checked;
        c.set(true);
        let user_phrase = (placed)().join(" ");
        let q = &qs_c[(state.current)()];
        let is_correct =
            user_phrase == q.original_phrase;
        state.record_result(
            is_correct,
            q.hint.clone(),
            q.original_phrase.clone(),
        );
    };

    let reset = move || {
        let mut p = placed;
        let mut c = checked;
        p.set(Vec::new());
        c.set(false);
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
            p {
                class: "text-[0.9375rem] \
                    text-[var(--color-text-secondary)] \
                    text-center italic m-0",
                "{hint_text}"
            }
            div {
                class: "min-h-[3rem] w-full p-4 \
                    bg-[var(--glass-bg)] \
                    backdrop-blur-[20px] \
                    border border-[var(--color-border)] \
                    text-base \
                    text-[var(--color-text-primary)] \
                    text-center",
                "{(placed)().join(\" \")}"
            }
            div {
                class: "flex flex-wrap gap-2 \
                    justify-center",
                {render_words(
                    &questions,
                    state.current,
                    placed,
                    toggle_word,
                )}
            }
            if !(checked)() {
                button {
                    class: ACTION_BTN,
                    onclick: move |_| confirm(),
                    "Check"
                }
            } else {
                button {
                    class: ACTION_BTN,
                    onclick: move |_| next(),
                    "Next"
                }
            }
        }
    }
}

/// Render shuffled word buttons
fn render_words(
    questions: &[OrderPhraseQuestion],
    current: Signal<usize>,
    placed: Signal<Vec<String>>,
    toggle: impl Fn(String) + Copy + 'static,
) -> Element {
    let Some(q) = questions.get((current)()) else {
        return rsx! {};
    };
    let mut words: Vec<String> = q
        .words
        .iter()
        .map(|w| w.word.clone())
        .collect();
    shuffle_deterministic(&mut words);

    rsx! {
        for w in words.iter() {
            {render_word_btn(
                w.clone(), placed, toggle,
            )}
        }
    }
}

/// Render a single word button
fn render_word_btn(
    word: String,
    placed: Signal<Vec<String>>,
    toggle: impl Fn(String) + Copy + 'static,
) -> Element {
    let w = word.clone();
    let class = use_memo(move || {
        if (placed)().contains(&w) {
            format!("{} {}", SELECTABLE, WORD_USED)
        } else {
            SELECTABLE.to_string()
        }
    });

    rsx! {
        button {
            class: "{class}",
            onclick: {
                let word = word.clone();
                move |_| toggle(word.clone())
            },
            "{word}"
        }
    }
}

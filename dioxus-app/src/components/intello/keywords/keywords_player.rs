// ** keywords_player.rs **
// ==> Keywords game — click correct keywords in statement

use crate::components::intello::shared::{
    game_helpers::derive_field,
    game_player_state::GamePlayerState,
    game_progress::GameProgress,
    game_results::GameResults,
};
use crate::domain::keywords_types::{
    KeywordQuestion, KeywordSet,
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

const SELECTED_TOGGLE: &str =
    "!bg-[var(--primary-tint-medium)] \
     !border-[var(--color-primary)] \
     !text-[var(--color-primary)]";

const ACTION_BTN: &str =
    "self-center px-8 py-3 text-[0.9375rem] \
     font-semibold text-[var(--color-background)] \
     bg-[var(--color-primary)] border-none \
     cursor-pointer transition-all \
     hover:opacity-90";

/// Keywords game player
#[component]
pub fn KeywordsPlayer(
    set: KeywordSet,
    on_back: EventHandler<()>,
) -> Element {
    let total = set.questions.len();
    let questions =
        use_hook(|| set.questions.clone());
    let state = GamePlayerState::new(total);
    let selected: Signal<Vec<String>> =
        use_signal(Vec::new);
    let checked = use_signal(|| false);

    let statement_text = derive_field(
        questions.clone(),
        state.current,
        |q| q.statement.clone(),
    );

    let toggle = move |word_id: String| {
        if (checked)() {
            return;
        }
        let mut sel = selected;
        let mut w = sel.write();
        if w.contains(&word_id) {
            w.retain(|v| v != &word_id);
        } else {
            w.push(word_id);
        }
    };

    let qs_c = questions.clone();
    let confirm = move || {
        let mut c = checked;
        c.set(true);
        let sel = (selected)();
        let q = &qs_c[(state.current)()];
        let correct_ids: Vec<&String> = q
            .keywords
            .iter()
            .filter(|k| k.is_correct)
            .map(|k| &k.id)
            .collect();
        let all_correct =
            sel.len() == correct_ids.len()
                && correct_ids
                    .iter()
                    .all(|id| sel.contains(id));
        state.record_result(
            all_correct,
            q.statement.clone(),
            q.explanation.clone(),
        );
    };

    let reset = move || {
        let mut s = selected;
        let mut c = checked;
        s.set(Vec::new());
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
            p { class: GLASS_PANEL,
                "{statement_text}"
            }
            div {
                class: "flex flex-wrap gap-2 \
                    justify-center",
                {render_chips(
                    &questions,
                    state.current,
                    selected,
                    toggle,
                )}
            }
            if !(checked)() {
                button {
                    class: ACTION_BTN,
                    onclick: move |_| confirm(),
                    "Confirm"
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

/// Render keyword chips for current question
fn render_chips(
    questions: &[KeywordQuestion],
    current: Signal<usize>,
    selected: Signal<Vec<String>>,
    toggle: impl Fn(String) + Copy + 'static,
) -> Element {
    let Some(q) = questions.get((current)()) else {
        return rsx! {};
    };

    rsx! {
        for k in q.keywords.iter() {
            {render_single_chip(
                k.id.clone(),
                k.word.clone(),
                selected,
                toggle,
            )}
        }
    }
}

/// Render a single keyword chip button
fn render_single_chip(
    kid: String,
    word: String,
    selected: Signal<Vec<String>>,
    toggle: impl Fn(String) + Copy + 'static,
) -> Element {
    let kid_c = kid.clone();
    let class = use_memo(move || {
        if (selected)().contains(&kid_c) {
            format!(
                "{} {}", SELECTABLE, SELECTED_TOGGLE,
            )
        } else {
            SELECTABLE.to_string()
        }
    });

    rsx! {
        button {
            class: "{class}",
            onclick: move |_| toggle(kid.clone()),
            "{word}"
        }
    }
}

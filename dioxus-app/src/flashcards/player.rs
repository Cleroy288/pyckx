//! Flashcard player — flip card, record know / don't know,
//! advance, then show shared `GameResults`.

use crate::flashcards::types::{Flashcard, FlashcardSet};
use crate::game_engine::{
    GamePlayerState, GameProgress, GameResults,
};
use dioxus::prelude::*;

/// Outer player layout container.
const LAYOUT: &str = "flex flex-col gap-6 max-w-[600px] \
    mx-auto items-center";

/// Common base classes for both action buttons.
const BTN_BASE: &str = "px-8 py-3.5 text-base \
    font-semibold cursor-pointer transition-all \
    border disabled:opacity-60 disabled:cursor-default";

/// Card wrapper when a face is shown (no flip applied).
const CARD_BASE: &str = "w-full min-h-[200px] \
    [perspective:1000px] cursor-pointer relative";

/// Extra classes that perform the flip animation.
const CARD_FLIP: &str = " [&>.front]:rotate-y-180 \
    [&>.back]:rotate-y-0";

/// "Don't know" button skin (error palette).
const BTN_FAIL: &str = "bg-[var(--color-error-bg)] \
    text-[var(--color-error-text)] \
    border-[var(--color-error-border)] \
    hover:bg-[var(--color-error-border)]";

/// "Know" button skin (success palette).
const BTN_OK: &str = "bg-[var(--color-success-bg)] \
    text-[var(--color-success-text)] \
    border-[var(--color-success-border)] \
    hover:bg-[var(--color-success-border)]";

/// Shared face classes (front + back).
const FACE: &str = "flex items-center justify-center \
    p-8 text-lg font-semibold text-center \
    [backface-visibility:hidden] \
    transition-transform duration-500";

/// Front face skin.
const FRONT_SKIN: &str = "bg-[var(--glass-bg)] \
    backdrop-blur-[20px] border \
    border-[var(--color-border)] \
    text-[var(--color-text-primary)]";

/// Back face skin.
const BACK_SKIN: &str = "absolute inset-0 \
    bg-[var(--primary-tint-light)] border \
    border-[var(--color-primary)] \
    text-[var(--color-primary)] rotate-y-180";

/// Flashcard player component.
#[component]
pub fn FlashcardPlayer(
    set: FlashcardSet,
    on_back: EventHandler<()>,
) -> Element {
    let total = set.cards.len();
    let cards = use_hook(|| set.cards.clone());
    let state = GamePlayerState::new(total);
    let flipped = use_signal(|| false);

    if (state.finished)() {
        return results_view(state, total, flipped, on_back);
    }
    rsx! {
        div { class: LAYOUT,
            GameProgress {
                current: state.current,
                total: total,
            }
            {flip_card(cards.clone(), state.current, flipped)}
            {action_row(cards, state, flipped)}
        }
    }
}

/// Render the end-of-run `GameResults` panel with replay
/// and back wired to the local signals.
fn results_view(
    state: GamePlayerState,
    total: usize,
    mut flipped: Signal<bool>,
    on_back: EventHandler<()>,
) -> Element {
    let on_replay = EventHandler::new(move |_| {
        state.replay();
        flipped.set(false);
    });
    let on_back_evt =
        EventHandler::new(move |_| on_back.call(()));
    rsx! {
        GameResults {
            score: (state.score)(),
            total: total,
            items: (state.results)(),
            on_replay: on_replay,
            on_back: on_back_evt,
        }
    }
}

/// Render the flippable card (front + back faces).
fn flip_card(
    cards: Vec<Flashcard>,
    current: Signal<usize>,
    mut flipped: Signal<bool>,
) -> Element {
    let idx = current();
    let front = cards[idx].front.clone();
    let back = cards[idx].back.clone();
    let flip_cls = if flipped() { CARD_FLIP } else { "" };
    rsx! {
        div {
            class: "{CARD_BASE}{flip_cls}",
            onclick: move |_| flipped.set(!flipped()),
            div { class: "front {FACE} {FRONT_SKIN}",
                "{front}"
            }
            div { class: "back {FACE} {BACK_SKIN}",
                "{back}"
            }
        }
    }
}

/// Render the "Don't Know" / "Know" action buttons.
fn action_row(
    cards: Vec<Flashcard>,
    state: GamePlayerState,
    flipped: Signal<bool>,
) -> Element {
    let pick_fail = pick_handler(
        cards.clone(), state, flipped, false,
    );
    let pick_ok = pick_handler(cards, state, flipped, true);
    rsx! {
        div { class: "flex gap-4",
            button {
                class: "{BTN_BASE} {BTN_FAIL}",
                onclick: pick_fail,
                "Don't Know"
            }
            button {
                class: "{BTN_BASE} {BTN_OK}",
                onclick: pick_ok,
                "Know"
            }
        }
    }
}

/// Build a click handler that records the answer and
/// advances to the next card.
fn pick_handler(
    cards: Vec<Flashcard>,
    state: GamePlayerState,
    mut flipped: Signal<bool>,
    correct: bool,
) -> impl FnMut(MouseEvent) + 'static {
    move |_| {
        let idx = (state.current)();
        let card = &cards[idx];
        state.record(
            correct,
            card.front.clone(),
            card.back.clone(),
        );
        state.advance();
        flipped.set(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_flip_class_contains_rotate() {
        assert!(CARD_FLIP.contains("rotate-y-180"));
    }

    #[test]
    fn test_card_base_has_perspective() {
        assert!(CARD_BASE.contains("[perspective:1000px]"));
    }
}

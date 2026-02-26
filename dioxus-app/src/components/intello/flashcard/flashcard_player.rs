// ** flashcard_player.rs **
// ==> Flashcard player — flip animation, know/don't know

use crate::components::intello::shared::{
    game_helpers::derive_field,
    game_player_state::GamePlayerState,
    game_progress::GameProgress,
    game_results::GameResults,
};
use crate::domain::flashcard_types::FlashcardSet;
use dioxus::prelude::*;

// -- Tailwind class constants --

const PLAYER: &str =
    "flex flex-col gap-6 max-w-[600px] \
     mx-auto items-center";

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

/// Flashcard player component
#[component]
pub fn FlashcardPlayer(
    set: FlashcardSet,
    on_back: EventHandler<()>,
) -> Element {
    let total = set.cards.len();
    let cards = use_hook(|| set.cards.clone());
    let state = GamePlayerState::new(total);
    let flipped = use_signal(|| false);

    let front_text = derive_field(
        cards.clone(),
        state.current,
        |c| c.front.clone(),
    );
    let back_text = derive_field(
        cards.clone(),
        state.current,
        |c| c.back.clone(),
    );

    let cards_a = cards.clone();
    let cards_b = cards.clone();

    let replay = state.make_replay(move || {
        let mut f = flipped;
        f.set(false);
    });

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

    let card_class = if (flipped)() {
        "w-full min-h-[200px] \
         [perspective:1000px] cursor-pointer \
         relative [&>.front]:rotate-y-180 \
         [&>.back]:rotate-y-0"
    } else {
        "w-full min-h-[200px] \
         [perspective:1000px] cursor-pointer \
         relative"
    };

    rsx! {
        div { class: PLAYER,
            GameProgress {
                current: state.current,
                total: total,
            }
            div {
                class: card_class,
                onclick: move |_| {
                    let val = (flipped)();
                    let mut f = flipped;
                    f.set(!val);
                },
                // Front face
                div {
                    class: "front flex items-center \
                        justify-center p-8 text-lg \
                        font-semibold text-center \
                        [backface-visibility:hidden] \
                        transition-transform duration-500 \
                        bg-[var(--glass-bg)] \
                        backdrop-blur-[20px] \
                        border border-[var(--color-border)] \
                        text-[var(--color-text-primary)]",
                    "{front_text}"
                }
                // Back face
                div {
                    class: "back absolute inset-0 \
                        flex items-center \
                        justify-center p-8 text-lg \
                        font-semibold text-center \
                        [backface-visibility:hidden] \
                        transition-transform duration-500 \
                        bg-[var(--primary-tint-light)] \
                        border border-[var(--color-primary)] \
                        text-[var(--color-primary)] \
                        rotate-y-180",
                    "{back_text}"
                }
            }
            div { class: "flex gap-4",
                button {
                    class: ERROR_BTN,
                    onclick: {
                        let cards = cards_a.clone();
                        move |_| {
                            let idx =
                                (state.current)();
                            state.record_and_advance(
                                false,
                                cards[idx].front.clone(),
                                cards[idx].back.clone(),
                            );
                            let mut f = flipped;
                            f.set(false);
                        }
                    },
                    "Don't Know"
                }
                button {
                    class: SUCCESS_BTN,
                    onclick: {
                        let cards = cards_b.clone();
                        move |_| {
                            let idx =
                                (state.current)();
                            state.record_and_advance(
                                true,
                                cards[idx].front.clone(),
                                cards[idx].back.clone(),
                            );
                            let mut f = flipped;
                            f.set(false);
                        }
                    },
                    "Know"
                }
            }
        }
    }
}

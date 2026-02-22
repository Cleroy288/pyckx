// ** flashcard_player.rs **
// ==> Flashcard player — flip animation, know/don't know

use crate::components::intello::shared::game_helpers::derive_field;
use crate::components::intello::shared::game_player_state::GamePlayerState;
use crate::components::intello::shared::game_progress::GameProgress;
use crate::components::intello::shared::game_results::GameResults;
use crate::domain::flashcard_types::FlashcardSet;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/flashcard/flashcard_player.module.css"
);
#[allow(dead_code)]
mod shared {
    stylance::import_crate_style!(
        css,
        "src/components/intello/shared/game_player.module.css"
    );
    pub use css::*;
}

/// Flashcard player component
#[component]
pub fn FlashcardPlayer(
    set: FlashcardSet,
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let total = set.cards.len();
    let cards = StoredValue::new(set.cards);
    let state = GamePlayerState::new(total);
    let flipped = RwSignal::new(false);

    let front_text = derive_field(
        cards, state.current,
        |c| c.front.clone(),
    );
    let back_text = derive_field(
        cards, state.current,
        |c| c.back.clone(),
    );

    let advance = move |knew: bool| {
        cards.with_value(|cs| {
            let idx = state.current.get();
            state.record_and_advance(
                knew,
                cs[idx].front.clone(),
                cs[idx].back.clone(),
            );
        });
        flipped.set(false);
    };

    let replay = state.make_replay(
        move || flipped.set(false),
    );

    view! {
        <Show
            when=move || !state.finished.get()
            fallback=move || view! {
                <GameResults
                    score=state.score.get()
                    total=total
                    items=state.results.get()
                    on_replay=Callback::new(
                        move |_| replay()
                    )
                    on_back=on_back
                />
            }
        >
            <div class=shared::player>
                <GameProgress
                    current=state.current
                    total=total
                />
                <div
                    class=move || format!(
                        "{} {}",
                        style::card,
                        if flipped.get() {
                            style::flipped
                        } else {
                            ""
                        }
                    )
                    on:click=move |_| {
                        flipped.update(|f| *f = !*f)
                    }
                >
                    <div class=style::front>
                        {front_text}
                    </div>
                    <div class=style::back>
                        {back_text}
                    </div>
                </div>
                <div class=style::actions>
                    <button
                        class=shared::error_btn
                        on:click=move |_| {
                            advance(false)
                        }
                    >
                        "Don't Know"
                    </button>
                    <button
                        class=shared::success_btn
                        on:click=move |_| {
                            advance(true)
                        }
                    >
                        "Know"
                    </button>
                </div>
            </div>
        </Show>
    }
}

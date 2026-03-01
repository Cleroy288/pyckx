// ** order_phrase_player.rs **
// ==> Reorder shuffled words into correct phrase

use crate::components::intello::shared::game_helpers::{
    derive_field, shuffle_deterministic,
};
use crate::components::intello::shared::game_player_state::GamePlayerState;
use crate::components::intello::shared::game_progress::GameProgress;
use crate::components::intello::shared::game_results::GameResults;
use crate::domain::order_phrase_types::OrderPhraseSet;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/order_phrase/order_phrase_player.module.css"
);
#[allow(dead_code)]
mod shared {
    stylance::import_crate_style!(
        css,
        "src/components/intello/shared/game_player.module.css"
    );
    pub use css::*;
}

/// Order phrase game player
#[component]
pub fn OrderPhrasePlayer(
    set: OrderPhraseSet,
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let total = set.questions.len();
    let questions = StoredValue::new(set.questions);
    let state = GamePlayerState::new(total);
    let placed: RwSignal<Vec<String>> =
        RwSignal::new(Vec::new());
    let checked = RwSignal::new(false);

    let hint_text = derive_field(
        questions, state.current,
        |q| q.hint.clone(),
    );

    let toggle_word = move |word: String| {
        if checked.get() {
            return;
        }
        placed.update(|v| {
            if v.contains(&word) {
                v.retain(|w| w != &word);
            } else {
                v.push(word);
            }
        });
    };

    let confirm = move || {
        checked.set(true);
        let user_phrase = placed.get().join(" ");
        questions.with_value(|qs| {
            let q = &qs[state.current.get()];
            let is_correct =
                user_phrase == q.original_phrase;
            state.record_result(
                is_correct,
                q.hint.clone(),
                q.original_phrase.clone(),
            );
        });
    };

    let reset = move || {
        placed.set(Vec::new());
        checked.set(false);
    };
    let next = state.make_next(reset);
    let replay = state.make_replay(reset);

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
                <p class=style::hint>
                    {hint_text}
                </p>
                <div class=style::placed>
                    {move || placed.get().join(" ")}
                </div>
                <div class=style::words>
                    {move || render_words(
                        questions,
                        state.current,
                        placed,
                        &toggle_word,
                    )}
                </div>
                <Show when=move || !checked.get()>
                    <button
                        class=shared::action_btn
                        on:click=move |_| confirm()
                    >
                        "Check"
                    </button>
                </Show>
                <Show when=move || checked.get()>
                    <button
                        class=shared::action_btn
                        on:click=move |_| next()
                    >
                        "Next"
                    </button>
                </Show>
            </div>
        </Show>
    }
}

/// Render shuffled word buttons
fn render_words(
    questions: StoredValue<
        Vec<
            crate::domain::order_phrase_types::OrderPhraseQuestion,
        >,
    >,
    current: RwSignal<usize>,
    placed: RwSignal<Vec<String>>,
    toggle: &(impl Fn(String) + Clone + 'static),
) -> impl IntoView {
    questions.with_value(|qs| {
        let Some(q) = qs.get(current.get()) else {
            return Vec::new().into_view();
        };
        let mut words: Vec<String> = q
            .words
            .iter()
            .map(|w| w.word.clone())
            .collect();
        shuffle_deterministic(&mut words);
        words
            .into_iter()
            .map(|w| {
                let w2 = w.clone();
                let w3 = w.clone();
                let toggle = toggle.clone();
                view! {
                    <button
                        class=move || {
                            let used = placed
                                .get()
                                .contains(&w2);
                            if used {
                                format!(
                                    "{} {}",
                                    shared::selectable,
                                    style::word_used,
                                )
                            } else {
                                shared::selectable
                                    .to_string()
                            }
                        }
                        on:click=move |_| {
                            toggle(w3.clone())
                        }
                    >
                        {w.clone()}
                    </button>
                }
            })
            .collect_view()
            .into_view()
    })
}

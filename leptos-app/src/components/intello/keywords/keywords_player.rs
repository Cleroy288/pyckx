// ** keywords_player.rs **
// ==> Keywords game — click correct keywords in statement

use crate::components::intello::shared::game_helpers::derive_field;
use crate::components::intello::shared::game_player_state::GamePlayerState;
use crate::components::intello::shared::game_progress::GameProgress;
use crate::components::intello::shared::game_results::GameResults;
use crate::domain::keywords_types::KeywordSet;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/keywords/keywords_player.module.css"
);
#[allow(dead_code)]
mod shared {
    stylance::import_crate_style!(
        css,
        "src/components/intello/shared/game_player.module.css"
    );
    pub use css::*;
}

/// Keywords game player
#[component]
pub fn KeywordsPlayer(
    set: KeywordSet,
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let total = set.questions.len();
    let questions = StoredValue::new(set.questions);
    let state = GamePlayerState::new(total);
    let selected: RwSignal<Vec<String>> =
        RwSignal::new(Vec::new());
    let checked = RwSignal::new(false);

    let statement_text = derive_field(
        questions, state.current,
        |q| q.statement.clone(),
    );

    let toggle = move |word_id: String| {
        if checked.get() {
            return;
        }
        selected.update(|v| {
            if v.contains(&word_id) {
                v.retain(|w| w != &word_id);
            } else {
                v.push(word_id);
            }
        });
    };

    let confirm = move || {
        checked.set(true);
        let sel = selected.get();
        questions.with_value(|qs| {
            let q = &qs[state.current.get()];
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
        });
    };

    let reset = move || {
        selected.set(Vec::new());
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
                <p class=shared::glass_panel>
                    {statement_text}
                </p>
                <div class=style::chips>
                    {move || render_chips(
                        questions,
                        state.current,
                        selected,
                        &toggle,
                    )}
                </div>
                <Show when=move || !checked.get()>
                    <button
                        class=shared::action_btn
                        on:click=move |_| confirm()
                    >
                        "Confirm"
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

/// Render keyword chips for current question
fn render_chips(
    questions: StoredValue<
        Vec<crate::domain::keywords_types::KeywordQuestion>,
    >,
    current: RwSignal<usize>,
    selected: RwSignal<Vec<String>>,
    toggle: &(impl Fn(String) + Clone + 'static),
) -> impl IntoView {
    questions.with_value(|qs| {
        let Some(q) = qs.get(current.get()) else {
            return Vec::new().into_view();
        };
        q.keywords
            .iter()
            .map(|k| {
                let kid = k.id.clone();
                let kid2 = kid.clone();
                let word = k.word.clone();
                let toggle = toggle.clone();
                view! {
                    <button
                        class=move || {
                            let sel = selected
                                .get()
                                .contains(&kid);
                            if sel {
                                format!(
                                    "{} {}",
                                    shared::selectable,
                                    shared::selected_toggle,
                                )
                            } else {
                                shared::selectable
                                    .to_string()
                            }
                        }
                        on:click=move |_| {
                            toggle(kid2.clone())
                        }
                    >
                        {word.clone()}
                    </button>
                }
            })
            .collect_view()
            .into_view()
    })
}

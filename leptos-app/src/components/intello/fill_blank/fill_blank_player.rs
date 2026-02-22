// ** fill_blank_player.rs **
// ==> Fill-the-blank game — phrase with blank, pick option

use crate::components::intello::shared::game_helpers::derive_field;
use crate::components::intello::shared::game_player_state::GamePlayerState;
use crate::components::intello::shared::game_progress::GameProgress;
use crate::components::intello::shared::game_results::GameResults;
use crate::domain::fill_blank_types::FillBlankSet;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/fill_blank/fill_blank_player.module.css"
);
#[allow(dead_code)]
mod shared {
    stylance::import_crate_style!(
        css,
        "src/components/intello/shared/game_player.module.css"
    );
    pub use css::*;
}

/// Fill blank game player
#[component]
pub fn FillBlankPlayer(
    set: FillBlankSet,
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let total = set.questions.len();
    let questions = StoredValue::new(set.questions);
    let state = GamePlayerState::new(total);
    let answered = RwSignal::new(false);

    let phrase_text = derive_field(
        questions, state.current,
        |q| q.phrase.clone(),
    );

    let check =
        move |_oid: String, is_correct: bool| {
            if answered.get() {
                return;
            }
            answered.set(true);
            questions.with_value(|qs| {
                let q = &qs[state.current.get()];
                state.record_result(
                    is_correct,
                    q.phrase.clone(),
                    q.explanation.clone(),
                );
            });
        };

    let reset = move || answered.set(false);
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
                <div class=shared::glass_panel>
                    {phrase_text}
                </div>
                <div class=style::options>
                    {move || render_options(
                        questions,
                        state.current,
                        answered,
                        &check,
                    )}
                </div>
                <Show when=move || answered.get()>
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

/// Render fill blank option buttons
fn render_options(
    questions: StoredValue<
        Vec<
            crate::domain::fill_blank_types::FillBlankQuestion,
        >,
    >,
    current: RwSignal<usize>,
    answered: RwSignal<bool>,
    check: &(impl Fn(String, bool) + Clone + 'static),
) -> impl IntoView {
    questions.with_value(|qs| {
        let Some(q) = qs.get(current.get()) else {
            return Vec::new().into_view();
        };
        q.options
            .iter()
            .map(|opt| {
                let oid = opt.id.clone();
                let correct = opt.is_correct;
                let text = opt.text.clone();
                let check = check.clone();
                view! {
                    <button
                        class=shared::selectable
                        disabled=answered
                        on:click=move |_| {
                            check(
                                oid.clone(),
                                correct,
                            )
                        }
                    >
                        {text.clone()}
                    </button>
                }
            })
            .collect_view()
            .into_view()
    })
}

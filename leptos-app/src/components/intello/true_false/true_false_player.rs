// ** true_false_player.rs **
// ==> True/False game — statement + True/False buttons

use crate::components::intello::shared::game_helpers::derive_field;
use crate::components::intello::shared::game_player_state::GamePlayerState;
use crate::components::intello::shared::game_progress::GameProgress;
use crate::components::intello::shared::game_results::GameResults;
use crate::domain::true_false_types::TrueOrFalseSet;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/true_false/true_false_player.module.css"
);
#[allow(dead_code)]
mod shared {
    stylance::import_crate_style!(
        css,
        "src/components/intello/shared/game_player.module.css"
    );
    pub use css::*;
}

/// True/False game player
#[component]
pub fn TrueFalsePlayer(
    set: TrueOrFalseSet,
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let total = set.statements.len();
    let stmts = StoredValue::new(set.statements);
    let state = GamePlayerState::new(total);
    let answered = RwSignal::new(false);

    let statement_text = derive_field(
        stmts, state.current,
        |s| s.statement.clone(),
    );

    let check = move |answer: bool| {
        if answered.get() {
            return;
        }
        answered.set(true);
        stmts.with_value(|ss| {
            let s = &ss[state.current.get()];
            state.record_result(
                answer == s.answer,
                s.statement.clone(),
                s.explanation.clone(),
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
                    {statement_text}
                </div>
                <div class=style::actions>
                    <button
                        class=shared::success_btn
                        disabled=answered
                        on:click=move |_| check(true)
                    >
                        "True"
                    </button>
                    <button
                        class=shared::error_btn
                        disabled=answered
                        on:click=move |_| {
                            check(false)
                        }
                    >
                        "False"
                    </button>
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

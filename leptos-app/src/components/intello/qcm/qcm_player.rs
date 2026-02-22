// ** qcm_player.rs **
// ==> QCM game player — one question at a time

use crate::components::intello::shared::game_helpers::{
    derive_field, shuffle_deterministic,
};
use crate::components::intello::shared::game_player_state::GamePlayerState;
use crate::components::intello::shared::game_progress::GameProgress;
use crate::components::intello::shared::game_results::GameResults;
use crate::domain::qcm_types::{QcmQuestion, QcmSet};
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/qcm/qcm_player.module.css"
);
#[allow(dead_code)]
mod shared {
    stylance::import_crate_style!(
        css,
        "src/components/intello/shared/game_player.module.css"
    );
    pub use css::*;
}

/// QCM game player component
#[component]
pub fn QcmPlayer(
    /// The QCM set to play
    set: QcmSet,
    /// Called when user finishes and goes back
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let total = set.questions.len();
    let questions = StoredValue::new(set.questions);
    let state = GamePlayerState::new(total);
    let selected: RwSignal<Option<String>> =
        RwSignal::new(None);
    let answered = RwSignal::new(false);

    let right = derive_field(
        questions, state.current,
        |q| q.right_answer.clone(),
    );
    let question_text = derive_field(
        questions, state.current,
        |q| q.question.clone(),
    );
    let explanation = derive_field(
        questions, state.current,
        |q| q.explanation.clone(),
    );
    let answers = derive_answers(
        questions, state.current,
    );

    let check = StoredValue::new(
        build_check_handler(
            answered, selected, questions, state,
        ),
    );
    let reset = move || {
        selected.set(None);
        answered.set(false);
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
                <div class=shared::glass_panel>
                    {question_text}
                </div>
                <div class=style::options>
                    {move || {
                        let ctx = QcmCtx {
                            selected, answered, right,
                        };
                        check.with_value(|ck| {
                            answers.get().into_iter()
                                .map(|a| {
                                    render_option(
                                        a, ctx, ck,
                                    )
                                })
                                .collect_view()
                        })
                    }}
                </div>
                <Show when=move || answered.get()>
                    <div class=shared::feedback>
                        {move || explanation.get()}
                    </div>
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

/// Build the check handler closure
fn build_check_handler(
    answered: RwSignal<bool>,
    selected: RwSignal<Option<String>>,
    questions: StoredValue<Vec<QcmQuestion>>,
    state: GamePlayerState,
) -> impl Fn(String) + Clone + 'static {
    move |answer: String| {
        if answered.get() {
            return;
        }
        selected.set(Some(answer.clone()));
        answered.set(true);
        questions.with_value(|qs| {
            let q = &qs[state.current.get()];
            state.record_result(
                answer == q.right_answer,
                q.question.clone(),
                q.explanation.clone(),
            );
        });
    }
}

/// Derive shuffled answer options
fn derive_answers(
    questions: StoredValue<Vec<QcmQuestion>>,
    current: RwSignal<usize>,
) -> Signal<Vec<String>> {
    Signal::derive(move || {
        questions.with_value(|qs| {
            let Some(q) = qs.get(current.get())
            else {
                return Vec::new();
            };
            let mut opts = q.wrong_answers.clone();
            opts.push(q.right_answer.clone());
            shuffle_deterministic(&mut opts);
            opts
        })
    })
}

/// Reactive signals needed for answer rendering
#[derive(Clone, Copy)]
struct QcmCtx {
    selected: RwSignal<Option<String>>,
    answered: RwSignal<bool>,
    right: Signal<String>,
}

/// Render a single answer option button
fn render_option(
    answer: String,
    ctx: QcmCtx,
    check: &(impl Fn(String) + Clone + 'static),
) -> impl IntoView {
    let a = answer.clone();
    let a2 = answer.clone();
    let check = check.clone();
    view! {
        <button
            class=move || {
                option_class(
                    &a, &ctx.selected.get(),
                    ctx.answered.get(),
                    &ctx.right.get(),
                )
            }
            disabled=ctx.answered
            on:click={
                let a = a2.clone();
                move |_| check(a.clone())
            }
        >
            {answer}
        </button>
    }
}

/// Compute CSS class for an answer option
fn option_class(
    answer: &str,
    selected: &Option<String>,
    answered: bool,
    right: &str,
) -> String {
    if !answered {
        return shared::selectable.to_string();
    }
    let is_right = answer == right;
    let is_sel = selected.as_deref() == Some(answer);
    if is_right {
        format!(
            "{} {}", shared::selectable, shared::correct,
        )
    } else if is_sel {
        format!(
            "{} {}", shared::selectable, shared::wrong,
        )
    } else {
        shared::selectable.to_string()
    }
}

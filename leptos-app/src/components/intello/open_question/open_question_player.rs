// ** open_question_player.rs **
// ==> Open question player — textarea answers, AI grading

use crate::api;
use crate::components::ui::button::{btn_click, Button};
use crate::components::ui::spinner::Spinner;
use crate::components::ui::textarea::Textarea;
use crate::domain::open_question_types::*;
use leptos::prelude::*;
use leptos::task::spawn_local;

stylance::import_crate_style!(
    style,
    "src/components/intello/open_question/open_question_player.module.css"
);

/// Open question player — answer all then submit
#[component]
pub fn OpenQuestionPlayer(
    set: OpenQuestionSet,
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let set_id = StoredValue::new(set.id.clone());
    let questions =
        StoredValue::new(set.questions.clone());
    let total = set.questions.len();

    // One signal per question for user answer
    let answers: Vec<RwSignal<String>> = (0..total)
        .map(|_| RwSignal::new(String::new()))
        .collect();
    let answers_stored =
        StoredValue::new(answers.clone());

    let grading = RwSignal::new(false);
    let grades: RwSignal<Option<CheckAnswersResponse>> =
        RwSignal::new(None);
    let error: RwSignal<Option<String>> =
        RwSignal::new(None);

    let submit = move |_| {
        grading.set(true);
        error.set(None);
        let req = questions.with_value(|qs| {
            answers_stored.with_value(|ans| {
                CheckAnswersRequest {
                    set_id: set_id.get_value(),
                    answers: qs
                        .iter()
                        .zip(ans.iter())
                        .map(|(q, a)| UserAnswerInput {
                            question_id: q
                                .id
                                .clone(),
                            user_answer: a.get(),
                        })
                        .collect(),
                }
            })
        });
        spawn_local(async move {
            match api::open_questions::check_answers(
                &req,
            )
            .await
            {
                Ok(resp) => grades.set(Some(resp)),
                Err(e) => error.set(Some(e)),
            }
            grading.set(false);
        });
    };

    let display_qs = set.questions.clone();

    view! {
        <div class=style::player>
            <Show
                when=move || grades.get().is_none()
                fallback=move || {
                    let g = grades.get().unwrap();
                    view! {
                        <div class=style::grades>
                            <h2>"Results"</h2>
                            <For
                                each=move || {
                                    g.grades.clone()
                                }
                                key=|g| {
                                    g.question_id.clone()
                                }
                                let:grade
                            >
                                <div
                                    class=style::grade_item
                                >
                                    <span
                                        class=style::grade_label
                                    >
                                        {grade
                                            .grade
                                            .clone()}
                                    </span>
                                    <p>
                                        {grade
                                            .feedback
                                            .clone()}
                                    </p>
                                </div>
                            </For>
                            <Button
                                text="Back"
                                on_click=btn_click(
                                    move |_| {
                                        on_back.run(())
                                    },
                                )
                            />
                        </div>
                    }
                }
            >
                {display_qs.iter().enumerate()
                    .map(|(i, q)| {
                        let sig = answers[i];
                        let question =
                            q.question.clone();
                        let hint = q.hint.clone();
                        view! {
                            <div
                                class=
                                    style::question_block
                            >
                                <h3
                                    class=style::question
                                >
                                    {i + 1}". "
                                    {question}
                                </h3>
                                {hint.map(|h| view! {
                                    <p
                                        class=
                                            style::hint
                                    >
                                        "Hint: "{h}
                                    </p>
                                })}
                                <Textarea
                                    id=format!(
                                        "oq-{}", i
                                    )
                                    placeholder=
                                        "Your answer..."
                                            .to_string()
                                    value=sig
                                    on_input=
                                        Callback::new(
                                            move |v| {
                                                sig.set(v)
                                            },
                                        )
                                />
                            </div>
                        }
                    })
                    .collect_view()
                }
                {move || error.get().map(|e| view! {
                    <p class=style::error>{e}</p>
                })}
                <Show
                    when=move || grading.get()
                    fallback=move || view! {
                        <Button
                            text="Submit Answers"
                            on_click=btn_click(submit)
                        />
                    }
                >
                    <Spinner />
                </Show>
            </Show>
        </div>
    }
}

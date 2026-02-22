//! Manual QCM Form Component
//! Form for creating QCM sets with questions manually

use crate::api::intello::{create_qcm_set, CreateQcmQuestionInput, CreateQcmSetRequest};
use crate::components::ui::button::{btn_click, Button, ButtonSize, ButtonVariant};
use crate::components::ui::icon::Icon;
use crate::components::ui::input::Input;
use crate::components::ui::section_divider::SectionDivider;
use crate::components::ui::select::{Select, SelectOption};
use crate::components::ui::textarea::Textarea;
use leptos::prelude::*;
use leptos::task::spawn_local;

stylance::import_crate_style!(style, "src/components/intello/qcm_form.module.css");

// Language options
fn language_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("en", "English"),
        SelectOption::new("fr", "French"),
        SelectOption::new("es", "Spanish"),
        SelectOption::new("de", "German"),
        SelectOption::new("nl", "Dutch"),
    ]
}

// Level options
fn level_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ]
}

#[component]
pub fn ManualQcmForm(
    #[prop(into)] on_back: Callback<()>,
    #[prop(optional, into)] on_success: Option<Callback<String>>,
) -> impl IntoView {
    // Form state
    let name = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let language = RwSignal::new("en".to_string());
    let level = RwSignal::new("medium".to_string());
    let subject_input = RwSignal::new(String::new());
    let subjects: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    // Questions state - Vec of (question, right_answer, wrong_answers[3], explanation)
    let questions: RwSignal<Vec<QuestionState>> = RwSignal::new(vec![QuestionState::default()]);

    // UI state
    let is_submitting = RwSignal::new(false);
    let error_message: RwSignal<Option<String>> = RwSignal::new(None);
    let success_message: RwSignal<Option<String>> = RwSignal::new(None);

    // Add subject handler
    let add_subject = move |_| {
        let input = subject_input.get().trim().to_string();
        if input.is_empty() || subjects.get().len() >= 3 {
            return;
        }
        subjects.update(|s| {
            if !s.contains(&input) {
                s.push(input);
            }
        });
        subject_input.set(String::new());
    };

    // Remove subject handler
    let remove_subject = move |index: usize| {
        subjects.update(|s| {
            if index < s.len() {
                s.remove(index);
            }
        });
    };

    // Add question handler
    let add_question = move |_| {
        if questions.get().len() < 20 {
            questions.update(|q| q.push(QuestionState::default()));
        }
    };

    // Remove question handler
    let remove_question = move |index: usize| {
        if questions.get().len() <= 1 {
            return;
        }
        questions.update(|q| {
            if index < q.len() {
                q.remove(index);
            }
        });
    };

    // Validation
    let is_valid = Signal::derive(move || {
        let n = name.get();
        let d = description.get();
        let qs = questions.get();

        !n.trim().is_empty()
            && !d.trim().is_empty()
            && !qs.is_empty()
            && qs.iter().all(|q| q.is_valid())
    });

    // Submit handler
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if !is_valid.get() || is_submitting.get() {
            return;
        }

        error_message.set(None);
        success_message.set(None);
        is_submitting.set(true);

        let request = CreateQcmSetRequest {
            name: name.get().trim().to_string(),
            description: description.get().trim().to_string(),
            level: level.get(),
            language: language.get(),
            subjects: subjects.get(),
            questions: questions
                .get()
                .iter()
                .map(|q| CreateQcmQuestionInput {
                    question: q.question.get().trim().to_string(),
                    right_answer: q.right_answer.get().trim().to_string(),
                    wrong_answers: q
                        .wrong_answers
                        .iter()
                        .map(|w| w.get().trim().to_string())
                        .collect(),
                    explanation: q.explanation.get().trim().to_string(),
                })
                .collect(),
        };

        let on_success = on_success;

        spawn_local(async move {
            match create_qcm_set(request).await {
                Ok(set) => {
                    let msg = format!(
                        "Created QCM set '{}' with {} questions!",
                        set.name,
                        set.questions.len()
                    );
                    success_message.set(Some(msg));
                    is_submitting.set(false);

                    let _ = on_success
                        .map(|cb| cb.run(set.id));
                }
                Err(e) => {
                    error_message.set(Some(e));
                    is_submitting.set(false);
                }
            }
        });
    };

    view! {
        <form class=style::form on:submit=handle_submit>
            // Error message
            <Show when=move || error_message.get().is_some()>
                <div class=style::error_banner>
                    {move || error_message.get().unwrap_or_default()}
                </div>
            </Show>

            // Success message
            <Show when=move || success_message.get().is_some()>
                <div class=style::success_banner>
                    {move || success_message.get().unwrap_or_default()}
                </div>
            </Show>

            // Set Details Section
            <section class=style::section>
                <SectionDivider label="Set Details" />

                <Input
                    type_="text".to_string()
                    id="name".to_string()
                    placeholder="Set name".to_string()
                    value=name
                    on_input=Callback::new(move |val: String| name.set(val))
                    required=true
                />

                <div class=style::row>
                    <Select
                        id="language".to_string()
                        value=language
                        on_change=Callback::new(move |val: String| language.set(val))
                        options=language_options()
                        disabled=is_submitting
                    />
                    <Select
                        id="level".to_string()
                        placeholder=
                            "Difficulty Level".to_string()
                        value=level
                        on_change=Callback::new(move |val: String| level.set(val))
                        options=level_options()
                        disabled=is_submitting
                    />
                </div>

                <Textarea
                    id="description".to_string()
                    placeholder="Description".to_string()
                    value=description
                    on_input=Callback::new(move |val: String| description.set(val))
                    rows=2
                    required=true
                    disabled=is_submitting
                />

                // Subjects
                <div class=style::subjects_section>
                    <div class=style::subject_input_row>
                        <Input
                            type_="text".to_string()
                            id="subject".to_string()
                            placeholder="Add a subject...".to_string()
                            value=subject_input
                            on_input=Callback::new(move |val: String| subject_input.set(val))
                        />
                        <Button
                            text="Add".to_string()
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Small
                            on_click=btn_click(add_subject)
                            disabled=Signal::derive(move || subjects.get().len() >= 3 || subject_input.get().trim().is_empty())
                        />
                    </div>
                    <div class=style::tags>
                        <For
                            each=move || subjects.get().into_iter().enumerate()
                            key=|(i, s)| format!("{}-{}", i, s)
                            children=move |(index, subject)| {
                                view! {
                                    <span class=style::tag>
                                        {subject}
                                        <button
                                            type="button"
                                            class=style::tag_remove
                                            on:click=move |_| remove_subject(index)
                                        >
                                            <Icon icon_name="X".to_string() />
                                        </button>
                                    </span>
                                }
                            }
                        />
                    </div>
                </div>
            </section>

            // Questions Section
            <section class=style::section>
                <SectionDivider label="Questions" />

                <div class=style::questions_list>
                    <For
                        each=move || questions.get().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(index, q)| {
                            let can_remove = Signal::derive(move || questions.get().len() > 1);
                            view! {
                                <QuestionCard
                                    index=index
                                    question=q
                                    on_remove=Callback::new(move |_| remove_question(index))
                                    can_remove=can_remove
                                    disabled=is_submitting
                                />
                            }
                        }
                    />
                </div>

                <Button
                    text="Add Question".to_string()
                    variant=ButtonVariant::Outline
                    size=ButtonSize::Small
                    on_click=btn_click(add_question)
                    disabled=Signal::derive(move || questions.get().len() >= 20)
                />
            </section>

            // Actions
            <div class=style::actions>
                <Button
                    text="Cancel".to_string()
                    variant=ButtonVariant::Outline
                    on_click=btn_click(move |_| on_back.run(()))
                    disabled=is_submitting
                />
                <Button
                    text="Create QCM Set".to_string()
                    type_="submit".to_string()
                    disabled=Signal::derive(move || !is_valid.get() || is_submitting.get())
                />
            </div>
        </form>
    }
}

// Question state struct
#[derive(Clone)]
struct QuestionState {
    question: RwSignal<String>,
    right_answer: RwSignal<String>,
    wrong_answers: [RwSignal<String>; 3],
    explanation: RwSignal<String>,
}

impl Default for QuestionState {
    fn default() -> Self {
        Self {
            question: RwSignal::new(String::new()),
            right_answer: RwSignal::new(String::new()),
            wrong_answers: [
                RwSignal::new(String::new()),
                RwSignal::new(String::new()),
                RwSignal::new(String::new()),
            ],
            explanation: RwSignal::new(String::new()),
        }
    }
}

impl QuestionState {
    fn is_valid(&self) -> bool {
        let q = self.question.get();
        let r = self.right_answer.get();
        let e = self.explanation.get();

        !q.trim().is_empty()
            && !r.trim().is_empty()
            && !e.trim().is_empty()
            && self.wrong_answers.iter().all(|w| !w.get().trim().is_empty())
    }
}

// Question Card subcomponent
#[component]
fn QuestionCard(
    index: usize,
    question: QuestionState,
    on_remove: Callback<()>,
    can_remove: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class=style::question_card>
            <SectionDivider
                label=format!("Question {}", index + 1)
            />
            <Show when=move || can_remove.get()>
                <button
                    type="button"
                    class=style::remove_btn
                    on:click=move |_| on_remove.run(())
                >
                    <Icon icon_name="Trash".to_string() />
                    " Remove"
                </button>
            </Show>

            <Textarea
                id=format!("q-{}-question", index)
                placeholder="Enter your question...".to_string()
                value=question.question
                on_input=Callback::new(move |val: String| question.question.set(val))
                rows=2
                required=true
                disabled=disabled
            />

            <div class=style::answers_grid>
                <Input
                    type_="text".to_string()
                    id=format!("q-{}-right", index)
                    placeholder="Correct answer".to_string()
                    value=question.right_answer
                    on_input=Callback::new(move |val: String| question.right_answer.set(val))
                    required=true
                />

                {question.wrong_answers.iter().enumerate().map(|(wi, wrong)| {
                    let wrong = *wrong;
                    view! {
                        <Input
                            type_="text".to_string()
                            id=format!("q-{}-wrong-{}", index, wi)
                            placeholder=format!("Wrong answer {}", wi + 1)
                            value=wrong
                            on_input=Callback::new(move |val: String| wrong.set(val))
                            required=true
                        />
                    }
                }).collect_view()}
            </div>

            <Textarea
                id=format!("q-{}-explanation", index)
                placeholder="Explain the correct answer...".to_string()
                value=question.explanation
                on_input=Callback::new(move |val: String| question.explanation.set(val))
                rows=2
                required=true
                disabled=disabled
            />
        </div>
    }
}

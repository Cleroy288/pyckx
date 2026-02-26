//! Manual QCM Form Component
//! Form for creating QCM sets with questions manually

use crate::api::intello::{
    create_qcm_set, CreateQcmQuestionInput,
    CreateQcmSetRequest,
};
use crate::components::ui::button::{
    Button, ButtonSize, ButtonVariant,
};
use crate::components::ui::icon::Icon;
use crate::components::ui::input::Input;
use crate::components::ui::section_divider::{
    SectionDivider,
};
use crate::components::ui::select::{
    Select, SelectOption,
};
use crate::components::ui::textarea::Textarea;
use dioxus::prelude::*;

/// Language options
fn language_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("en", "English"),
        SelectOption::new("fr", "French"),
        SelectOption::new("es", "Spanish"),
        SelectOption::new("de", "German"),
        SelectOption::new("nl", "Dutch"),
    ]
}

/// Level options
fn level_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ]
}

/// Question state struct
#[derive(Clone, PartialEq)]
struct QuestionState {
    question: Signal<String>,
    right_answer: Signal<String>,
    wrong_answers: [Signal<String>; 3],
    explanation: Signal<String>,
}

impl Default for QuestionState {
    fn default() -> Self {
        Self {
            question: Signal::new(String::new()),
            right_answer: Signal::new(String::new()),
            wrong_answers: [
                Signal::new(String::new()),
                Signal::new(String::new()),
                Signal::new(String::new()),
            ],
            explanation: Signal::new(String::new()),
        }
    }
}

impl QuestionState {
    fn is_valid(&self) -> bool {
        let q = (self.question)();
        let r = (self.right_answer)();
        let e = (self.explanation)();

        !q.trim().is_empty()
            && !r.trim().is_empty()
            && !e.trim().is_empty()
            && self
                .wrong_answers
                .iter()
                .all(|w| !(w)().trim().is_empty())
    }
}

/// Manual QCM creation form
#[component]
pub fn ManualQcmForm(
    on_back: EventHandler<()>,
    #[props(default)]
    on_success: Option<EventHandler<String>>,
) -> Element {
    // Form state
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut language =
        use_signal(|| "en".to_string());
    let mut level =
        use_signal(|| "medium".to_string());
    let mut subject_input = use_signal(String::new);
    let mut subjects: Signal<Vec<String>> =
        use_signal(Vec::new);

    // Questions state
    let mut questions: Signal<Vec<QuestionState>> =
        use_signal(|| vec![QuestionState::default()]);

    // UI state
    let mut is_submitting = use_signal(|| false);
    let mut error_message: Signal<Option<String>> =
        use_signal(|| None);
    let mut success_message: Signal<Option<String>> =
        use_signal(|| None);

    // Validation
    let is_valid = use_memo(move || {
        let n = (name)();
        let d = (description)();
        let qs = (questions)();
        !n.trim().is_empty()
            && !d.trim().is_empty()
            && !qs.is_empty()
            && qs.iter().all(|q| q.is_valid())
    });

    rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |ev| {
                ev.prevent_default();
                if !(is_valid)() || (is_submitting)() {
                    return;
                }
                error_message.set(None);
                success_message.set(None);
                is_submitting.set(true);
                let request = build_request(
                    &name, &description, &level,
                    &language, &subjects, &questions,
                );
                let on_success = on_success;
                spawn(async move {
                    handle_submit(
                        request,
                        on_success,
                        &mut success_message,
                        &mut error_message,
                        &mut is_submitting,
                    )
                    .await;
                });
            },
            // Error message
            if let Some(e) = (error_message)() {
                div {
                    class: "p-4 bg-[color-mix(in_srgb,\
                        var(--destructive)_10%,transparent)] \
                        border border-[var(--color-error)] \
                        text-[var(--color-error)] text-sm",
                    "{e}"
                }
            }
            // Success message
            if let Some(s) = (success_message)() {
                div {
                    class: "p-4 bg-[color-mix(in_srgb,\
                        var(--color-success)_10%,transparent)] \
                        border border-[var(--color-success)] \
                        text-[var(--color-success)] text-sm",
                    "{s}"
                }
            }
            // Set Details Section
            section {
                class: "flex flex-col gap-4",
                SectionDivider { label: "Set Details" }
                Input {
                    input_type: "text",
                    id: "name",
                    placeholder: "Set name",
                    value: name,
                    on_input: move |val: String| {
                        name.set(val)
                    },
                    required: true,
                }
                div { class: "flex gap-4",
                    div { class: "flex-1",
                        Select {
                            id: "language",
                            value: language,
                            on_change: move |val: String| {
                                language.set(val)
                            },
                            options: language_options(),
                            disabled: is_submitting,
                        }
                    }
                    div { class: "flex-1",
                        Select {
                            id: "level",
                            placeholder: "Difficulty Level",
                            value: level,
                            on_change: move |val: String| {
                                level.set(val)
                            },
                            options: level_options(),
                            disabled: is_submitting,
                        }
                    }
                }
                Textarea {
                    id: "description",
                    placeholder: "Description",
                    value: description,
                    on_input: move |val: String| {
                        description.set(val)
                    },
                    rows: 2,
                    required: true,
                    disabled: is_submitting,
                }
                // Subjects
                div {
                    class: "flex flex-col gap-2",
                    div {
                        class: "flex gap-2 items-stretch",
                        div { class: "flex-1",
                            Input {
                                input_type: "text",
                                id: "subject",
                                placeholder: "Add a subject...",
                                value: subject_input,
                                on_input: move |val: String| {
                                    subject_input.set(val)
                                },
                            }
                        }
                        Button {
                            text: "Add",
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Small,
                            on_click: move |_| {
                                let input = (subject_input)()
                                    .trim()
                                    .to_string();
                                if input.is_empty()
                                    || (subjects)().len() >= 3
                                {
                                    return;
                                }
                                let mut s = subjects.write();
                                if !s.contains(&input) {
                                    s.push(input);
                                }
                                subject_input.set(String::new());
                            },
                            disabled: use_memo(move || {
                                (subjects)().len() >= 3
                                    || (subject_input)()
                                        .trim()
                                        .is_empty()
                            }),
                        }
                    }
                    div {
                        class: "flex flex-wrap gap-2",
                        for (index, subject) in
                            (subjects)().iter().enumerate()
                        {
                            span {
                                class: "inline-flex items-center \
                                    gap-1 px-2 py-1 \
                                    bg-[color-mix(in_srgb,\
                                    var(--primary)_10%,transparent)] \
                                    border border-[var(--color-primary)] \
                                    text-sm text-[var(--color-primary)]",
                                "{subject}"
                                button {
                                    r#type: "button",
                                    class: "flex items-center \
                                        justify-center bg-transparent \
                                        border-none p-0 ml-1 \
                                        cursor-pointer \
                                        text-[var(--color-primary)] \
                                        opacity-70 hover:opacity-100",
                                    onclick: move |_| {
                                        let mut s = subjects.write();
                                        if index < s.len() {
                                            s.remove(index);
                                        }
                                    },
                                    Icon { icon_name: "X" }
                                }
                            }
                        }
                    }
                }
            }
            // Questions Section
            section {
                class: "flex flex-col gap-4 pt-4",
                SectionDivider { label: "Questions" }
                div {
                    class: "flex flex-col gap-4",
                    for (index, q) in
                        (questions)().iter().enumerate()
                    {
                        QuestionCard {
                            index: index,
                            question: q.clone(),
                            on_remove: move |_| {
                                if (questions)().len() <= 1 {
                                    return;
                                }
                                let mut qs =
                                    questions.write();
                                if index < qs.len() {
                                    qs.remove(index);
                                }
                            },
                            can_remove: use_memo(move || {
                                (questions)().len() > 1
                            }),
                            disabled: is_submitting,
                        }
                    }
                }
                Button {
                    text: "Add Question",
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Small,
                    on_click: move |_| {
                        if (questions)().len() < 20 {
                            questions.write().push(
                                QuestionState::default(),
                            );
                        }
                    },
                    disabled: use_memo(move || {
                        (questions)().len() >= 20
                    }),
                }
            }
            // Actions
            div {
                class: "flex gap-4 justify-end mt-6",
                Button {
                    text: "Cancel",
                    variant: ButtonVariant::Outline,
                    on_click: move |_| on_back.call(()),
                    disabled: is_submitting,
                }
                Button {
                    text: "Create QCM Set",
                    type_: "submit",
                    disabled: use_memo(move || {
                        !(is_valid)() || (is_submitting)()
                    }),
                }
            }
        }
    }
}

/// Build the API request from form signals
fn build_request(
    name: &Signal<String>,
    description: &Signal<String>,
    level: &Signal<String>,
    language: &Signal<String>,
    subjects: &Signal<Vec<String>>,
    questions: &Signal<Vec<QuestionState>>,
) -> CreateQcmSetRequest {
    CreateQcmSetRequest {
        name: (name)().trim().to_string(),
        description: (description)()
            .trim()
            .to_string(),
        level: (level)(),
        language: (language)(),
        subjects: (subjects)(),
        questions: (questions)()
            .iter()
            .map(|q| CreateQcmQuestionInput {
                question: (q.question)()
                    .trim()
                    .to_string(),
                right_answer: (q.right_answer)()
                    .trim()
                    .to_string(),
                wrong_answers: q
                    .wrong_answers
                    .iter()
                    .map(|w| (w)().trim().to_string())
                    .collect(),
                explanation: (q.explanation)()
                    .trim()
                    .to_string(),
            })
            .collect(),
    }
}

/// Handle the form submission
async fn handle_submit(
    request: CreateQcmSetRequest,
    on_success: Option<EventHandler<String>>,
    success_msg: &mut Signal<Option<String>>,
    error_msg: &mut Signal<Option<String>>,
    is_submitting: &mut Signal<bool>,
) {
    match create_qcm_set(request).await {
        Ok(set) => {
            let msg = format!(
                "Created QCM set '{}' with {} questions!",
                set.name,
                set.questions.len()
            );
            success_msg.set(Some(msg));
            is_submitting.set(false);
            if let Some(cb) = on_success {
                cb.call(set.id);
            }
        }
        Err(e) => {
            error_msg.set(Some(e));
            is_submitting.set(false);
        }
    }
}

/// Question Card subcomponent
#[component]
fn QuestionCard(
    index: usize,
    question: QuestionState,
    on_remove: EventHandler<()>,
    can_remove: Memo<bool>,
    disabled: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4",
            SectionDivider {
                label: format!(
                    "Question {}", index + 1,
                ),
            }
            if (can_remove)() {
                button {
                    r#type: "button",
                    class: "inline-flex items-center \
                        gap-1 self-end bg-transparent \
                        border-none py-1 px-2 \
                        cursor-pointer \
                        text-[var(--color-error)] \
                        text-[0.8rem] opacity-70 \
                        hover:opacity-100",
                    onclick: move |_| {
                        on_remove.call(())
                    },
                    Icon { icon_name: "Trash" }
                    " Remove"
                }
            }
            Textarea {
                id: format!("q-{}-question", index),
                placeholder: "Enter your question...",
                value: question.question,
                on_input: move |val: String| {
                    question.question.set(val)
                },
                rows: 2,
                required: true,
                disabled: disabled,
            }
            div {
                class: "grid grid-cols-2 gap-4 \
                    max-sm:grid-cols-1",
                Input {
                    input_type: "text",
                    id: format!("q-{}-right", index),
                    placeholder: "Correct answer",
                    value: question.right_answer,
                    on_input: move |val: String| {
                        question.right_answer.set(val)
                    },
                    required: true,
                }
                for (wi, wrong) in question
                    .wrong_answers.iter().enumerate()
                {
                    Input {
                        input_type: "text",
                        id: format!(
                            "q-{}-wrong-{}", index, wi,
                        ),
                        placeholder: format!(
                            "Wrong answer {}", wi + 1,
                        ),
                        value: *wrong,
                        on_input: {
                            let mut w = *wrong;
                            move |val: String| w.set(val)
                        },
                        required: true,
                    }
                }
            }
            Textarea {
                id: format!(
                    "q-{}-explanation", index,
                ),
                placeholder: "Explain the correct \
                    answer...",
                value: question.explanation,
                on_input: move |val: String| {
                    question.explanation.set(val)
                },
                rows: 2,
                required: true,
                disabled: disabled,
            }
        }
    }
}

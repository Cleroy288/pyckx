//! QCM manual creation page and submission form.

use dioxus::prelude::*;

use crate::home::HomeTopBar;
use crate::qcm::api;
use crate::qcm::create_fields::render_meta_section;
use crate::qcm::create_questions::{
    render_questions_section, QuestionState,
};
use crate::qcm::types::CreateQcmSetRequest;
use crate::ui::{
    Button, ButtonVariant, HeroBanner, PageLayout,
};

/// Page title shown in the hero banner.
const TITLE: &str = "Create QCM Set";

/// Where the cancel and post-success actions navigate to.
const BACK_PATH: &str = "/qcm";

/// QCM manual creation route component.
pub fn QcmCreatePage() -> Element {
    let nav = navigator();
    let on_done = move |_: ()| {
        nav.push(BACK_PATH);
    };
    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner { title: TITLE, span {} }
            ManualQcmForm { on_done: on_done }
        }
    }
}

/// Signals for every input field in the set form.
#[derive(Clone, Copy)]
pub struct FormFields {
    pub name: Signal<String>,
    pub description: Signal<String>,
    pub level: Signal<String>,
    pub language: Signal<String>,
    pub subjects: Signal<Vec<String>>,
    pub subject_input: Signal<String>,
    pub questions: Signal<Vec<QuestionState>>,
}

/// Reactive feedback signals (loading + messages).
#[derive(Clone, Copy)]
pub struct FeedbackState {
    pub submitting: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub success: Signal<Option<String>>,
}

/// Manual QCM creation form.
#[component]
pub fn ManualQcmForm(
    /// Called when submission succeeds.
    on_done: EventHandler<()>,
) -> Element {
    let fields = use_form_fields();
    let feedback = use_feedback_state();
    let valid = use_form_valid(fields);
    let mut submit = make_submit(fields, feedback, on_done);
    rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |ev| {
                ev.prevent_default();
                if !(valid)() || (feedback.submitting)() {
                    return;
                }
                submit();
            },
            {render_messages(feedback)}
            {render_meta_section(fields, feedback)}
            {render_questions_section(fields, feedback)}
            {render_actions(feedback, valid)}
        }
    }
}

/// Allocate signals for every form field.
fn use_form_fields() -> FormFields {
    FormFields {
        name: use_signal(String::new),
        description: use_signal(String::new),
        level: use_signal(|| "medium".to_string()),
        language: use_signal(|| "en".to_string()),
        subjects: use_signal(Vec::new),
        subject_input: use_signal(String::new),
        questions: use_signal(|| {
            vec![QuestionState::default()]
        }),
    }
}

/// Allocate the submit / error / success signals.
fn use_feedback_state() -> FeedbackState {
    FeedbackState {
        submitting: use_signal(|| false),
        error: use_signal(|| None),
        success: use_signal(|| None),
    }
}

/// Memoize whether the whole form is valid.
fn use_form_valid(fields: FormFields) -> Memo<bool> {
    use_memo(move || {
        !(fields.name)().trim().is_empty()
            && !(fields.description)().trim().is_empty()
            && !(fields.questions)().is_empty()
            && (fields.questions)()
                .iter()
                .all(|q| q.is_complete())
    })
}

/// Build the async submit closure.
fn make_submit(
    fields: FormFields,
    feedback: FeedbackState,
    on_done: EventHandler<()>,
) -> impl FnMut() + 'static {
    let FeedbackState {
        mut submitting,
        mut error,
        mut success,
    } = feedback;
    move || {
        error.set(None);
        success.set(None);
        submitting.set(true);
        let request = build_request(fields);
        spawn(async move {
            match api::create_qcm_set(request).await {
                Ok(set) => {
                    success.set(Some(format!(
                        "Created '{}' ({} Qs)",
                        set.name,
                        set.questions.len(),
                    )));
                    on_done.call(());
                }
                Err(e) => error.set(Some(e)),
            }
            submitting.set(false);
        });
    }
}

/// Build the API request from the form fields.
fn build_request(
    fields: FormFields,
) -> CreateQcmSetRequest {
    CreateQcmSetRequest {
        name: (fields.name)().trim().to_string(),
        description: (fields.description)()
            .trim()
            .to_string(),
        level: (fields.level)(),
        language: (fields.language)(),
        subjects: (fields.subjects)(),
        questions: (fields.questions)()
            .iter()
            .map(|q| q.to_input())
            .collect(),
    }
}

/// Render error and success banners (when present).
fn render_messages(feedback: FeedbackState) -> Element {
    rsx! {
        if let Some(e) = (feedback.error)() {
            div { class: "p-4 border \
                border-[var(--color-error)] \
                text-[var(--color-error)] text-sm",
                "{e}"
            }
        }
        if let Some(s) = (feedback.success)() {
            div { class: "p-4 border \
                border-[var(--color-success)] \
                text-[var(--color-success)] text-sm",
                "{s}"
            }
        }
    }
}

/// Render the cancel + submit action row.
fn render_actions(
    feedback: FeedbackState,
    valid: Memo<bool>,
) -> Element {
    let nav = navigator();
    let submit_disabled = use_memo(move || {
        !(valid)() || (feedback.submitting)()
    });
    rsx! {
        div { class: "flex gap-4 justify-end mt-6",
            Button {
                text: "Cancel".to_string(),
                variant: ButtonVariant::Outline,
                on_click: move |_| { nav.push(BACK_PATH); },
                disabled: feedback.submitting,
            }
            Button {
                text: "Create QCM Set".to_string(),
                type_: "submit".to_string(),
                disabled: submit_disabled,
            }
        }
    }
}

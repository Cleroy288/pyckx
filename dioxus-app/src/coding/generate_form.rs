//! Generate page form — reactive state + view + dropdown
//! options. Pure presentation; submission lives in `generate`.

use dioxus::prelude::*;

use crate::ui::{
    Button, ButtonVariant, Input, Select, SelectOption,
    Spinner,
};

const DEFAULT_LANGUAGE: &str = "rust";
const DEFAULT_LEVEL: &str = "medium";
const DEFAULT_LANGUE: &str = "eng";

/// Reactive bag for all form fields.
#[derive(Clone, Copy, PartialEq)]
pub struct FormState {
    pub language: Signal<String>,
    pub subject: Signal<String>,
    pub level: Signal<String>,
    pub langue: Signal<String>,
}

/// Reactive bag for submission status.
#[derive(Clone, Copy, PartialEq)]
pub struct StatusState {
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

/// Initialise the form with sensible defaults.
pub fn use_form() -> FormState {
    FormState {
        language: use_signal(|| DEFAULT_LANGUAGE.into()),
        subject: use_signal(String::new),
        level: use_signal(|| DEFAULT_LEVEL.into()),
        langue: use_signal(|| DEFAULT_LANGUE.into()),
    }
}

/// Initialise an idle status (no error, not loading).
pub fn use_status() -> StatusState {
    StatusState {
        loading: use_signal(|| false),
        error: use_signal(|| None),
    }
}

/// `None` when value matches the default, else `Some`.
pub fn opt_if_not(s: String, default: &str) -> Option<String> {
    if s == default { None } else { Some(s) }
}

/// `None` when the trimmed string is empty, else `Some`.
pub fn opt_trim(s: String) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Default value for the "exercise language" field.
pub const LANGUE_DEFAULT: &str = DEFAULT_LANGUE;

#[component]
pub fn FormView(
    form: FormState,
    status: StatusState,
    on_submit: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            FormFields { form: form }
            ErrorLine { error: status.error }
            SubmitArea {
                loading: status.loading,
                on_click: on_submit,
            }
        }
    }
}

#[component]
fn FormFields(form: FormState) -> Element {
    let mut language = form.language;
    let mut subject = form.subject;
    let mut level = form.level;
    let mut langue = form.langue;
    rsx! {
        Select {
            id: "coding-lang",
            label: "Programming Language".to_string(),
            options: language_options(),
            value: language,
            on_change: move |v: String| language.set(v),
        }
        Input {
            input_type: "text",
            id: "coding-subject",
            label: "Subject (optional)".to_string(),
            placeholder: "e.g. loops, ownership, async...",
            value: subject,
            on_input: move |v: String| subject.set(v),
        }
        Select {
            id: "coding-level",
            label: "Difficulty".to_string(),
            options: difficulty_options(),
            value: level,
            on_change: move |v: String| level.set(v),
        }
        Select {
            id: "coding-langue",
            label: "Exercise Language".to_string(),
            options: langue_options(),
            value: langue,
            on_change: move |v: String| langue.set(v),
        }
    }
}

#[component]
fn ErrorLine(error: Signal<Option<String>>) -> Element {
    let Some(msg) = (error)() else {
        return rsx! {};
    };
    rsx! {
        p {
            class: "text-[var(--color-error)] text-sm m-0",
            "{msg}"
        }
    }
}

#[component]
fn SubmitArea(
    loading: Signal<bool>,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    if (loading)() {
        return rsx! {
            div {
                class: "flex items-center gap-3 \
                    text-[var(--color-text-secondary)]",
                Spinner {}
                span { "Generating..." }
            }
        };
    }
    rsx! {
        Button {
            text: "Generate",
            variant: ButtonVariant::Primary,
            disabled: false,
            on_click: move |e| on_click.call(e),
        }
    }
}

fn language_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("rust", "Rust"),
        SelectOption::new("python", "Python"),
        SelectOption::new("javascript", "JavaScript"),
        SelectOption::new("go", "Go"),
    ]
}

fn difficulty_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ]
}

fn langue_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("eng", "English"),
        SelectOption::new("fr", "Français"),
        SelectOption::new("nl", "Nederlands"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_trim_empty_returns_none() {
        assert_eq!(opt_trim("   ".into()), None);
    }

    #[test]
    fn test_opt_trim_non_empty_returns_some() {
        assert_eq!(
            opt_trim("loops".into()),
            Some("loops".into())
        );
    }

    #[test]
    fn test_opt_if_not_default_returns_none() {
        assert_eq!(opt_if_not("eng".into(), "eng"), None);
    }

    #[test]
    fn test_opt_if_not_other_returns_some() {
        assert_eq!(
            opt_if_not("fr".into(), "eng"),
            Some("fr".into())
        );
    }
}

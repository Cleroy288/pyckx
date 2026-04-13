//! Reusable form to launch AI generation of a game set.

use crate::ui::{
    Button, ButtonVariant, FileUpload, Input, Select,
    SelectOption, Spinner, Textarea,
};
use dioxus::prelude::*;

const DEFAULT_LANG: &str = "en";
const DEFAULT_LEVEL: &str = "medium";
const DEFAULT_QUESTIONS: u8 = 10;

/// Data emitted by the generate form on submit.
#[derive(Debug, Clone)]
pub struct GenerateFormData {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub language: String,
    pub level: String,
    pub subjects: Vec<String>,
    pub num_questions: u8,
    /// File names selected by the user.
    pub files: Vec<String>,
}

/// Aggregates every field signal of the form.
#[derive(Clone, Copy)]
struct FormFields {
    name: Signal<String>,
    desc: Signal<String>,
    instr: Signal<String>,
    lang: Signal<String>,
    level: Signal<String>,
    subjects: Signal<String>,
    num_q: Signal<String>,
    files: Signal<Vec<String>>,
}

impl FormFields {
    /// Collect the current values into a `GenerateFormData`.
    fn collect(&self) -> GenerateFormData {
        let subs = parse_subjects(&(self.subjects)());
        let nq = (self.num_q)()
            .parse::<u8>()
            .unwrap_or(DEFAULT_QUESTIONS);
        GenerateFormData {
            name: (self.name)(),
            description: (self.desc)(),
            instructions: (self.instr)(),
            language: (self.lang)(),
            level: (self.level)(),
            subjects: subs,
            num_questions: nq,
            files: (self.files)(),
        }
    }
}

/// Reusable AI-generation form.
#[component]
pub fn GenerateForm(
    /// Called on submit with the collected form data.
    on_submit: EventHandler<GenerateFormData>,
    /// Controls whether the submit is disabled.
    loading: Signal<bool>,
    /// Optional error to display.
    #[props(default)]
    error: Signal<Option<String>>,
) -> Element {
    let fields = use_form_fields();
    let mut file_count = use_signal(|| 0_usize);

    let disabled = use_memo(move || {
        (fields.name)().trim().is_empty()
            || (fields.desc)().trim().is_empty()
            || loading()
    });

    let on_files = move |names: Vec<String>| {
        file_count.set(names.len());
        fields.files.clone().set(names);
    };

    let handle_submit = move |_| on_submit.call(fields.collect());

    rsx! {
        div { class: "flex flex-col gap-4",
            {render_fields(fields)}
            FileUpload {
                on_files: on_files,
                accept: ".pdf,.txt,.doc,.docx",
                multiple: true,
            }
            if file_count() > 0 {
                p {
                    class: "m-0 text-[0.8125rem] \
                        text-[var(--color-text-secondary)]",
                    "{file_count()} file(s) selected"
                }
            }
            if let Some(e) = error() {
                p {
                    class: "text-[var(--color-error)] text-sm m-0",
                    "{e}"
                }
            }
            if loading() {
                div {
                    class: "flex items-center gap-3 \
                        text-[var(--color-text-secondary)]",
                    Spinner {}
                    span { "Generating..." }
                }
            } else {
                Button {
                    text: "Generate",
                    variant: ButtonVariant::Primary,
                    disabled: disabled,
                    on_click: handle_submit,
                }
            }
        }
    }
}

/// Allocate each field signal with sensible defaults.
fn use_form_fields() -> FormFields {
    FormFields {
        name: use_signal(String::new),
        desc: use_signal(String::new),
        instr: use_signal(String::new),
        lang: use_signal(|| DEFAULT_LANG.to_string()),
        level: use_signal(|| DEFAULT_LEVEL.to_string()),
        subjects: use_signal(String::new),
        num_q: use_signal(|| DEFAULT_QUESTIONS.to_string()),
        files: use_signal(Vec::new),
    }
}

/// Split a comma-separated subjects string into non-empty items.
fn parse_subjects(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Render every input field of the form.
fn render_fields(mut fields: FormFields) -> Element {
    let level_options = vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ];
    rsx! {
        Input {
            input_type: "text",
            id: "gen-name",
            placeholder: "Set name",
            value: fields.name,
            on_input: move |v: String| fields.name.set(v),
            required: true,
        }
        Textarea {
            id: "gen-desc",
            placeholder: "Describe what to generate...",
            value: fields.desc,
            on_input: move |v: String| fields.desc.set(v),
        }
        Textarea {
            id: "gen-instr",
            placeholder: "Additional instructions...",
            value: fields.instr,
            on_input: move |v: String| fields.instr.set(v),
        }
        div { class: "flex gap-4",
            div { class: "flex-1",
                Input {
                    input_type: "text",
                    id: "gen-lang",
                    placeholder: "Language",
                    value: fields.lang,
                    on_input: move |v: String| fields.lang.set(v),
                }
            }
            div { class: "flex-1",
                Select {
                    id: "gen-level",
                    placeholder: "Difficulty Level",
                    options: level_options,
                    value: fields.level,
                    on_change: move |v: String| fields.level.set(v),
                }
            }
        }
        Input {
            input_type: "text",
            id: "gen-subjects",
            placeholder: "Subjects (comma separated)",
            value: fields.subjects,
            on_input: move |v: String| fields.subjects.set(v),
        }
        Input {
            input_type: "number",
            id: "gen-num",
            placeholder: "Number of questions",
            value: fields.num_q,
            on_input: move |v: String| fields.num_q.set(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subjects_empty_string_returns_empty() {
        assert!(parse_subjects("").is_empty());
    }

    #[test]
    fn test_parse_subjects_trims_whitespace() {
        let result = parse_subjects(" math , science ");
        assert_eq!(result, vec!["math", "science"]);
    }

    #[test]
    fn test_parse_subjects_filters_empty_items() {
        let result = parse_subjects("a,,b,");
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn test_parse_subjects_single_item() {
        let result = parse_subjects("math");
        assert_eq!(result, vec!["math"]);
    }
}

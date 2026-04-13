//! Metadata + subjects section of the QCM create form.

use dioxus::prelude::*;

use crate::qcm::create::{FeedbackState, FormFields};
use crate::ui::{
    Button, ButtonSize, ButtonVariant, Icon, Input,
    SectionDivider, Select, SelectOption, Textarea,
};

/// Maximum number of subject tags per set.
pub const MAX_SUBJECTS: usize = 3;

/// Render the "Set Details" section (name, desc, lang, level, subjects).
pub fn render_meta_section(
    fields: FormFields,
    feedback: FeedbackState,
) -> Element {
    let mut name = fields.name;
    let mut description = fields.description;
    let mut language = fields.language;
    let mut level = fields.level;
    let disabled = feedback.submitting;
    rsx! {
        section { class: "flex flex-col gap-4",
            SectionDivider {
                label: "Set Details".to_string(),
            }
            Input {
                input_type: "text".to_string(),
                id: "name".to_string(),
                placeholder: "Set name".to_string(),
                value: name,
                on_input: move |v| name.set(v),
                required: true,
            }
            div { class: "flex gap-4",
                div { class: "flex-1",
                    Select {
                        id: "language".to_string(),
                        value: language,
                        on_change: move |v| language.set(v),
                        options: language_options(),
                        disabled: disabled,
                    }
                }
                div { class: "flex-1",
                    Select {
                        id: "level".to_string(),
                        placeholder: "Difficulty".to_string(),
                        value: level,
                        on_change: move |v| level.set(v),
                        options: level_options(),
                        disabled: disabled,
                    }
                }
            }
            Textarea {
                id: "description".to_string(),
                placeholder: "Description".to_string(),
                value: description,
                on_input: move |v| description.set(v),
                rows: 2_u32,
                required: true,
                disabled: disabled,
            }
            SubjectsField {
                subjects: fields.subjects,
                input: fields.subject_input,
            }
        }
    }
}

/// Tag-style subjects picker (input + tag list).
#[component]
fn SubjectsField(
    subjects: Signal<Vec<String>>,
    input: Signal<String>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2",
            SubjectAddRow {
                subjects: subjects, input: input,
            }
            SubjectTags { subjects: subjects }
        }
    }
}

/// Subject input + add button.
#[component]
fn SubjectAddRow(
    subjects: Signal<Vec<String>>,
    input: Signal<String>,
) -> Element {
    let subjects = subjects;
    let mut input = input;
    let add_disabled = use_memo(move || {
        (subjects)().len() >= MAX_SUBJECTS
            || (input)().trim().is_empty()
    });
    rsx! {
        div { class: "flex gap-2 items-stretch",
            div { class: "flex-1",
                Input {
                    input_type: "text".to_string(),
                    id: "subject".to_string(),
                    placeholder: "Add a subject..."
                        .to_string(),
                    value: input,
                    on_input: move |v| input.set(v),
                }
            }
            Button {
                text: "Add".to_string(),
                variant: ButtonVariant::Outline,
                size: ButtonSize::Small,
                on_click: move |_| {
                    add_subject(input, subjects);
                },
                disabled: add_disabled,
            }
        }
    }
}

/// Append the trimmed input to the subject list.
fn add_subject(
    mut input: Signal<String>,
    mut subjects: Signal<Vec<String>>,
) {
    let txt = (input)().trim().to_string();
    if txt.is_empty()
        || (subjects)().len() >= MAX_SUBJECTS
    {
        return;
    }
    let mut list = subjects.write();
    if !list.contains(&txt) {
        list.push(txt);
    }
    drop(list);
    input.set(String::new());
}

/// Render the list of subject tags with remove buttons.
#[component]
fn SubjectTags(
    subjects: Signal<Vec<String>>,
) -> Element {
    let mut subjects = subjects;
    rsx! {
        div { class: "flex flex-wrap gap-2",
            for (idx, subject) in
                (subjects)().iter().enumerate()
            {
                span {
                    class: "inline-flex items-center \
                        gap-1 px-2 py-1 border \
                        border-[var(--color-primary)] \
                        text-sm \
                        text-[var(--color-primary)]",
                    "{subject}"
                    button {
                        r#type: "button",
                        class: "bg-transparent border-none \
                            cursor-pointer ml-1",
                        onclick: move |_| {
                            let mut list = subjects.write();
                            if idx < list.len() {
                                list.remove(idx);
                            }
                        },
                        Icon { icon_name: "X" }
                    }
                }
            }
        }
    }
}

/// Language dropdown options.
pub fn language_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("en", "English"),
        SelectOption::new("fr", "French"),
        SelectOption::new("es", "Spanish"),
        SelectOption::new("de", "German"),
        SelectOption::new("nl", "Dutch"),
    ]
}

/// Difficulty dropdown options.
pub fn level_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_options_has_five_entries() {
        assert_eq!(language_options().len(), 5);
    }

    #[test]
    fn test_level_options_has_three_entries() {
        assert_eq!(level_options().len(), 3);
    }
}

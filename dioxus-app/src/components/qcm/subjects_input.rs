//! SubjectsInput — tag-style subject picker

use crate::components::ui::button::{
    Button, ButtonSize, ButtonVariant,
};
use crate::components::ui::icon::Icon;
use crate::components::ui::input::Input;
use dioxus::prelude::*;

/// Maximum number of subjects allowed
const MAX_SUBJECTS: usize = 3;

/// Tag-style subject input with add/remove
#[component]
pub fn SubjectsInput(
    subjects: Signal<Vec<String>>,
    subject_input: Signal<String>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2",
            SubjectAddRow {
                subjects: subjects,
                subject_input: subject_input,
            }
            SubjectTags {
                subjects: subjects,
            }
        }
    }
}

/// Input row with add button
#[component]
fn SubjectAddRow(
    subjects: Signal<Vec<String>>,
    subject_input: Signal<String>,
) -> Element {
    let mut subjects = subjects;
    rsx! {
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
                        || (subjects)().len()
                            >= MAX_SUBJECTS
                    {
                        return;
                    }
                    let mut s = subjects.write();
                    if !s.contains(&input) {
                        s.push(input);
                    }
                    subject_input
                        .set(String::new());
                },
                disabled: use_memo(move || {
                    (subjects)().len()
                        >= MAX_SUBJECTS
                        || (subject_input)()
                            .trim()
                            .is_empty()
                }),
            }
        }
    }
}

/// Rendered subject tags with remove buttons
#[component]
fn SubjectTags(
    subjects: Signal<Vec<String>>,
) -> Element {
    let mut subjects = subjects;
    rsx! {
        div {
            class: "flex flex-wrap gap-2",
            for (index, subject) in
                (subjects)().iter().enumerate()
            {
                span {
                    class: "inline-flex items-center \
                        gap-1 px-2 py-1 \
                        bg-[color-mix(in_srgb,\
                        var(--primary)_10%,\
                        transparent)] \
                        border \
                        border-[var(--color-primary)] \
                        text-sm \
                        text-[var(--color-primary)]",
                    "{subject}"
                    button {
                        r#type: "button",
                        class: "flex items-center \
                            justify-center \
                            bg-transparent \
                            border-none p-0 ml-1 \
                            cursor-pointer \
                            text-[var(\
                            --color-primary)] \
                            opacity-70 \
                            hover:opacity-100",
                        onclick: move |_| {
                            let mut s =
                                subjects.write();
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

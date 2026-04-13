// ** generate_form.rs **
// ==> Reusable AI generation form

use crate::ui::{
    Button, ButtonVariant,
};
use crate::ui::FileUpload;
use crate::ui::Input;
use crate::ui::{
    Select, SelectOption,
};
use crate::ui::Spinner;
use crate::ui::Textarea;
use dioxus::prelude::*;

/// Data emitted by the generate form
#[derive(Debug, Clone)]
pub struct GenerateFormData {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub language: String,
    pub level: String,
    pub subjects: Vec<String>,
    pub num_questions: u8,
    /// File names selected by user
    pub files: Vec<String>,
}

/// Reusable AI generation form
#[component]
pub fn GenerateForm(
    /// Called on submit with form data
    on_submit: EventHandler<GenerateFormData>,
    /// Loading state
    loading: Signal<bool>,
    /// Error message
    #[props(default)]
    error: Signal<Option<String>>,
) -> Element {
    let mut name = use_signal(String::new);
    let mut desc = use_signal(String::new);
    let mut instr = use_signal(String::new);
    let mut lang =
        use_signal(|| "en".to_string());
    let mut level =
        use_signal(|| "medium".to_string());
    let mut subjects = use_signal(String::new);
    let mut num_q =
        use_signal(|| "10".to_string());
    let mut file_count = use_signal(|| 0_usize);
    let mut file_names: Signal<Vec<String>> =
        use_signal(Vec::new);

    let disabled = use_memo(move || {
        (name)().trim().is_empty()
            || (desc)().trim().is_empty()
            || (loading)()
    });

    let handle = move |_| {
        let subs: Vec<String> = (subjects)()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let nq = (num_q)()
            .parse::<u8>()
            .unwrap_or(10);
        on_submit.call(GenerateFormData {
            name: (name)(),
            description: (desc)(),
            instructions: (instr)(),
            language: (lang)(),
            level: (level)(),
            subjects: subs,
            num_questions: nq,
            files: (file_names)(),
        });
    };

    let on_files = move |names: Vec<String>| {
        file_count.set(names.len());
        file_names.set(names);
    };

    rsx! {
        div { class: "flex flex-col gap-4",
            Input {
                input_type: "text",
                id: "gen-name",
                placeholder: "Set name",
                value: name,
                on_input: move |v: String| {
                    name.set(v)
                },
                required: true,
            }
            Textarea {
                id: "gen-desc",
                placeholder: "Describe what to \
                    generate...",
                value: desc,
                on_input: move |v: String| {
                    desc.set(v)
                },
            }
            Textarea {
                id: "gen-instr",
                placeholder: "Additional \
                    instructions...",
                value: instr,
                on_input: move |v: String| {
                    instr.set(v)
                },
            }
            div { class: "flex gap-4",
                div { class: "flex-1",
                    Input {
                        input_type: "text",
                        id: "gen-lang",
                        placeholder: "Language",
                        value: lang,
                        on_input: move |v: String| {
                            lang.set(v)
                        },
                    }
                }
                div { class: "flex-1",
                    Select {
                        id: "gen-level",
                        placeholder: "Difficulty Level",
                        options: vec![
                            SelectOption::new(
                                "easy", "Easy",
                            ),
                            SelectOption::new(
                                "medium", "Medium",
                            ),
                            SelectOption::new(
                                "hard", "Hard",
                            ),
                        ],
                        value: level,
                        on_change: move |v: String| {
                            level.set(v)
                        },
                    }
                }
            }
            Input {
                input_type: "text",
                id: "gen-subjects",
                placeholder: "Subjects (comma \
                    separated)",
                value: subjects,
                on_input: move |v: String| {
                    subjects.set(v)
                },
            }
            Input {
                input_type: "number",
                id: "gen-num",
                placeholder: "Number of questions",
                value: num_q,
                on_input: move |v: String| {
                    num_q.set(v)
                },
            }
            FileUpload {
                on_files: on_files,
                accept: ".pdf,.txt,.doc,.docx",
                multiple: true,
            }
            if (file_count)() > 0 {
                p {
                    class: "m-0 text-[0.8125rem] \
                        text-[var(--color-text-secondary)]",
                    "{(file_count)()} file(s) selected"
                }
            }
            if let Some(e) = (error)() {
                p {
                    class: "text-[var(--color-error)] \
                        text-sm m-0",
                    "{e}"
                }
            }
            if (loading)() {
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
                    on_click: handle,
                }
            }
        }
    }
}

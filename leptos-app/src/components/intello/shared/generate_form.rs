// ** generate_form.rs **
// ==> Reusable AI generation form

use crate::components::ui::button::{
    btn_class, ButtonSize, ButtonVariant,
};
use crate::components::ui::file_upload::FileUpload;
use crate::components::ui::input::Input;
use crate::components::ui::select::{
    Select, SelectOption,
};
use crate::components::ui::spinner::Spinner;
use crate::components::ui::textarea::Textarea;
use leptos::callback::{Callable, UnsyncCallback};
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

stylance::import_crate_style!(
    style,
    "src/components/intello/shared/\
     generate_form.module.css"
);

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
    pub files: Vec<web_sys::File>,
}

/// Reactive state for all form fields
#[derive(Clone, Copy)]
struct FormState {
    name: RwSignal<String>,
    desc: RwSignal<String>,
    instr: RwSignal<String>,
    lang: RwSignal<String>,
    level: RwSignal<String>,
    subjects: RwSignal<String>,
    num_q: RwSignal<String>,
}

impl FormState {
    fn new() -> Self {
        Self {
            name: RwSignal::new(String::new()),
            desc: RwSignal::new(String::new()),
            instr: RwSignal::new(String::new()),
            lang: RwSignal::new("en".to_string()),
            level: RwSignal::new(
                "medium".to_string(),
            ),
            subjects: RwSignal::new(String::new()),
            num_q: RwSignal::new("10".to_string()),
        }
    }
}

/// Build the submit click handler
fn build_submit_handler(
    fs: FormState,
    files: Rc<RefCell<Vec<web_sys::File>>>,
    on_submit: Callback<GenerateFormData>,
) -> impl Fn(leptos::ev::MouseEvent) + Clone {
    move |_: leptos::ev::MouseEvent| {
        let subs: Vec<String> = fs
            .subjects
            .get()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let nq = fs
            .num_q
            .get()
            .parse::<u8>()
            .unwrap_or(10);
        on_submit.run(GenerateFormData {
            name: fs.name.get(),
            description: fs.desc.get(),
            instructions: fs.instr.get(),
            language: fs.lang.get(),
            level: fs.level.get(),
            subjects: subs,
            num_questions: nq,
            files: files.borrow().clone(),
        });
    }
}

/// Build the file change callback
fn build_file_cb(
    files: Rc<RefCell<Vec<web_sys::File>>>,
    count: RwSignal<usize>,
) -> UnsyncCallback<Vec<web_sys::File>> {
    UnsyncCallback::new(
        move |f: Vec<web_sys::File>| {
            count.set(f.len());
            *files.borrow_mut() = f;
        },
    )
}

/// Reusable AI generation form
#[component]
pub fn GenerateForm(
    /// Called on submit with form data
    #[prop(into)]
    on_submit: Callback<GenerateFormData>,
    /// Loading state
    #[prop(into)]
    loading: Signal<bool>,
    /// Error message
    #[prop(optional, into)]
    error: Signal<Option<String>>,
) -> impl IntoView {
    // 5 locals: fs, files, file_count, disabled, handle
    let fs = FormState::new();
    let files = Rc::new(RefCell::new(Vec::new()));
    let file_count = RwSignal::new(0_usize);
    let disabled = Signal::derive(move || {
        fs.name.get().trim().is_empty()
            || fs.desc.get().trim().is_empty()
            || loading.get()
    });
    let handle = build_submit_handler(
        fs, files.clone(), on_submit,
    );

    view! {
        <div class=style::form>
            <Input type_="text" id="gen-name"
                placeholder="Set name".to_string()
                value=fs.name
                on_input=Callback::new(move |v| {
                    fs.name.set(v)
                })
                required=true />
            <Textarea id="gen-desc"
                placeholder=
                    "Describe what to generate..."
                    .to_string()
                value=fs.desc
                on_input=Callback::new(move |v| {
                    fs.desc.set(v)
                }) />
            <Textarea id="gen-instr"
                placeholder=
                    "Additional instructions..."
                    .to_string()
                value=fs.instr
                on_input=Callback::new(move |v| {
                    fs.instr.set(v)
                }) />
            <div class=style::row>
                <Input type_="text" id="gen-lang"
                    placeholder=
                        "Language".to_string()
                    value=fs.lang
                    on_input=Callback::new(
                        move |v| fs.lang.set(v),
                    ) />
                <Select id="gen-level"
                    placeholder=
                        "Difficulty Level"
                        .to_string()
                    options=vec![
                        SelectOption::new(
                            "easy", "Easy",
                        ),
                        SelectOption::new(
                            "medium", "Medium",
                        ),
                        SelectOption::new(
                            "hard", "Hard",
                        ),
                    ]
                    value=fs.level
                    on_change=Callback::new(
                        move |v| fs.level.set(v),
                    ) />
            </div>
            <Input type_="text" id="gen-subjects"
                placeholder=
                    "Subjects (comma separated)"
                    .to_string()
                value=fs.subjects
                on_input=Callback::new(move |v| {
                    fs.subjects.set(v)
                }) />
            <Input type_="number" id="gen-num"
                placeholder=
                    "Number of questions"
                    .to_string()
                value=fs.num_q
                on_input=Callback::new(move |v| {
                    fs.num_q.set(v)
                }) />
            <FileUpload
                on_files=build_file_cb(
                    files, file_count,
                )
                accept=
                    ".pdf,.txt,.doc,.docx".to_string()
                multiple=true
            />
            <p
                class=style::file_count
                style:display=move || {
                    if file_count.get() > 0 {
                        "block"
                    } else {
                        "none"
                    }
                }
            >
                {move || {
                    format!(
                        "{} file(s) selected",
                        file_count.get()
                    )
                }}
            </p>
            {move || error.get().map(|e| view! {
                <p class=style::error>{e}</p>
            })}
            <div
                class=style::loading
                style:display=move || {
                    if loading.get() {
                        "flex"
                    } else {
                        "none"
                    }
                }
            >
                <Spinner />
                <span>"Generating..."</span>
            </div>
            <button
                type="button"
                disabled=disabled
                style:display=move || {
                    if loading.get() {
                        "none"
                    } else {
                        "inline-flex"
                    }
                }
                class=btn_class(
                    ButtonVariant::Primary,
                    ButtonSize::Medium,
                )
                on:click=handle
            >
                "Generate"
            </button>
        </div>
    }
}

// ** file_upload.rs **
// ==> Drag-drop zone + file picker using web_sys::File

use leptos::callback::{Callable, UnsyncCallback};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

stylance::import_crate_style!(
    style,
    "src/components/ui/file_upload/file_upload.module.css"
);

/// File upload component with drag-drop
#[component]
pub fn FileUpload(
    /// Callback when files are selected
    #[prop(into)]
    on_files: UnsyncCallback<Vec<web_sys::File>>,
    /// Accepted file types (e.g., ".pdf,.txt")
    #[prop(optional, into)]
    accept: Option<String>,
    /// Whether multiple files are allowed
    #[prop(optional)]
    multiple: bool,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let dragging = RwSignal::new(false);

    let extract_files = move |file_list: web_sys::FileList| {
        let mut files = Vec::new();
        for i in 0..file_list.length() {
            if let Some(f) = file_list.get(i) {
                files.push(f);
            }
        }
        on_files.run(files);
    };

    let on_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        dragging.set(false);
        if let Some(dt) = ev.data_transfer() {
            if let Some(fl) = dt.files() {
                extract_files(fl);
            }
        }
    };

    let on_change = move |ev: leptos::ev::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement =
            target.unchecked_into();
        if let Some(fl) = input.files() {
            extract_files(fl);
        }
    };

    view! {
        <div
            class=move || format!(
                "{} {} {}",
                style::zone,
                if dragging.get() { style::dragging } else { "" },
                class.clone().unwrap_or_default()
            )
            on:dragover=move |ev: web_sys::DragEvent| {
                ev.prevent_default();
                dragging.set(true);
            }
            on:dragleave=move |_| dragging.set(false)
            on:drop=on_drop
        >
            <crate::components::ui::icon::Icon
                icon_name="Layers".to_string()
            />
            <p class=style::text>
                "Drop files here or "
                <label class=style::browse>
                    "browse"
                    <input
                        type="file"
                        class=style::hidden_input
                        accept=accept.unwrap_or_default()
                        multiple=multiple
                        on:change=on_change
                    />
                </label>
            </p>
        </div>
    }
}

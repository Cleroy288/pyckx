//! FileUpload — Drag-drop zone + file picker
//!
//! Uses Dioxus FileEngine for file access.
//! Filenames are passed to the callback.

use crate::components::ui::icon::Icon;
use dioxus::prelude::*;

/// Drop zone base classes
const ZONE_CLS: &str = "\
    flex flex-col items-center justify-center \
    gap-3 p-8 \
    border-2 border-dashed \
    border-[color-mix(in_srgb,var(--primary)_20%,transparent)] \
    bg-[var(--glass-bg)] \
    backdrop-blur-[var(--glass-blur-sm)] \
    text-[var(--color-text-secondary)] \
    cursor-pointer \
    transition-all duration-[var(--transition-base)] \
    [&_svg]:w-7 [&_svg]:h-7 \
    [&_svg]:text-[var(--color-primary)] \
    hover:border-[var(--color-primary)] \
    hover:bg-[var(--glass-bg-hover)]";

/// Dragging-active override
const ZONE_DRAG: &str = "\
    !border-[var(--color-primary)] \
    !bg-[var(--glass-bg-hover)]";

/// File upload component with drag-drop
#[component]
pub fn FileUpload(
    /// Callback with selected file names
    on_files: EventHandler<Vec<String>>,
    /// Accepted file types (e.g., ".pdf,.txt")
    #[props(default)]
    accept: String,
    /// Whether multiple files are allowed
    #[props(default = false)]
    multiple: bool,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let mut dragging = use_signal(|| false);

    let cls = format!(
        "{} {} {}",
        ZONE_CLS,
        if *dragging.read() { ZONE_DRAG } else { "" },
        class,
    );

    rsx! {
        div {
            class: "{cls}",
            ondragover: move |ev: DragEvent| {
                ev.prevent_default();
                dragging.set(true);
            },
            ondragleave: move |_| dragging.set(false),
            ondrop: move |ev: DragEvent| {
                ev.prevent_default();
                dragging.set(false);
                // Drag-drop file extraction is not
                // directly supported via DragData.
                // Users should use the browse input.
            },
            Icon { icon_name: "Layers".to_string() }
            p { class: "m-0 text-sm text-center",
                "Drop files here or "
                label {
                    class: "text-[var(--color-primary)] \
                        cursor-pointer underline \
                        hover:opacity-80",
                    "browse"
                    input {
                        r#type: "file",
                        class: "hidden",
                        accept: "{accept}",
                        multiple: multiple,
                        onchange: move |ev: Event<FormData>| {
                            { let fe = ev.files();
                                on_files.call(fe.iter().map(|f| f.name()).collect());
                            }
                        },
                    }
                }
            }
        }
    }
}

//! Progress — Horizontal fill bar with label

use dioxus::prelude::*;

/// Track bar classes
const TRACK_CLS: &str = "\
    w-full h-2 \
    bg-[var(--color-border,#333)] \
    rounded overflow-hidden";

/// Fill bar classes
const FILL_CLS: &str = "\
    h-full rounded \
    bg-gradient-to-r \
    from-[var(--color-primary,#6366f1)] \
    to-[color-mix(in_srgb,var(--color-primary,#6366f1)_80%,#fff)] \
    transition-[width] duration-300 ease-out";

/// Horizontal progress bar component
#[component]
pub fn Progress(
    /// Value between 0 and 100 (Signal or Memo)
    value: ReadSignal<f64>,
    /// Optional label above the bar
    #[props(default)]
    label: Option<String>,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let pct = value.read().clamp(0.0, 100.0);
    let width_style = format!("width:{pct:.0}%");

    rsx! {
        div { class: "w-full {class}",
            if let Some(lbl) = &label {
                div {
                    class: "flex justify-between \
                        mb-1.5",
                    span {
                        class: "text-[0.8125rem] \
                            font-medium \
                            text-[var(--color-text-secondary,#999)]",
                        "{lbl}"
                    }
                    span {
                        class: "text-[0.8125rem] \
                            font-semibold \
                            text-[var(--color-text-primary)]",
                        "{pct:.0}%"
                    }
                }
            }
            div { class: "{TRACK_CLS}",
                div {
                    class: "{FILL_CLS}",
                    style: "{width_style}",
                }
            }
        }
    }
}

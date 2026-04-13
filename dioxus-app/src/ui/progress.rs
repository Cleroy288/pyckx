//! Progress indicators and tabbed navigation.

use dioxus::prelude::*;

/// Horizontal progress bar with an optional label.
#[component]
pub fn Progress(
    value: ReadSignal<f64>,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let pct = value.read().clamp(0.0, 100.0);
    let width_style = format!("width:{pct:.0}%");
    rsx! {
        div { class: "w-full {class}",
            {render_label(label.as_deref(), pct)}
            div { class: "{TRACK_CLS}",
                div { class: "{FILL_CLS}", style: "{width_style}" }
            }
        }
    }
}

/// Horizontal tab bar with clickable labels.
#[component]
pub fn Tabs(
    labels: Vec<String>,
    active: Signal<usize>,
    on_change: EventHandler<usize>,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div { class: "{BAR_CLS} {class}",
            for (i, label) in labels.iter().enumerate() {
                button {
                    key: "{i}",
                    class: tab_class(*active.read() == i),
                    onclick: move |_| on_change.call(i),
                    "{label}"
                }
            }
        }
    }
}

/// Conditionally rendered content for a tab index.
#[component]
pub fn TabPanel(
    index: usize,
    active: Signal<usize>,
    children: Element,
) -> Element {
    let display = if *active.read() == index { "block" } else { "none" };
    rsx! {
        div { class: "py-5", style: "display:{display}", {children} }
    }
}

/// Render the label row above the progress bar.
fn render_label(text: Option<&str>, pct: f64) -> Element {
    match text {
        Some(lbl) => rsx! {
            div { class: "flex justify-between mb-1.5",
                span { class: "{LABEL_CLS}", "{lbl}" }
                span { class: "{PCT_CLS}", "{pct:.0}%" }
            }
        },
        None => rsx! {},
    }
}

/// Classes for a tab button based on active state.
fn tab_class(is_active: bool) -> String {
    if is_active {
        format!("{TAB_CLS} {TAB_ACTIVE}")
    } else {
        TAB_CLS.to_string()
    }
}

const TRACK_CLS: &str = "\
    w-full h-2.5 \
    border-2 border-[var(--color-border)] \
    rounded-full bg-[var(--card)] \
    overflow-hidden";

const FILL_CLS: &str = "\
    h-full rounded-full \
    bg-[var(--color-primary,#6366f1)] \
    transition-[width] duration-300 ease-out";

const LABEL_CLS: &str = "\
    text-[0.8125rem] font-medium \
    text-[var(--color-text-secondary,#999)]";

const PCT_CLS: &str = "\
    text-[0.8125rem] font-semibold \
    text-[var(--color-text-primary)]";

const BAR_CLS: &str = "\
    flex gap-0 border-b \
    border-[var(--color-border,#333)] \
    overflow-x-auto";

const TAB_CLS: &str = "\
    py-3 px-5 text-sm font-semibold \
    text-[var(--color-text-secondary,#999)] \
    bg-transparent border-none \
    border-b-2 border-b-transparent \
    cursor-pointer whitespace-nowrap \
    transition-all duration-200 \
    hover:text-[var(--color-text-primary)]";

const TAB_ACTIVE: &str = "\
    !text-[var(--color-primary,#6366f1)] \
    !border-b-[var(--color-primary,#6366f1)] \
    !border-b-2";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_class_active_contains_active_marker() {
        assert!(tab_class(true).contains("!text-"));
    }

    #[test]
    fn test_tab_class_inactive_has_base_only() {
        let cls = tab_class(false);
        assert!(!cls.contains("!border-b-"));
    }
}

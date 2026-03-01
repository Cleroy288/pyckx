//! Tabs — Tab header bar + panel

use dioxus::prelude::*;

/// Tab button base classes
const TAB_CLS: &str = "\
    py-3 px-5 text-sm font-medium \
    text-[var(--color-text-secondary,#999)] \
    bg-transparent border-none \
    border-b-2 border-b-transparent \
    cursor-pointer whitespace-nowrap \
    transition-all duration-[var(--transition-base,0.15s)] \
    hover:text-[var(--color-text-primary)] \
    hover:bg-white/[0.03]";

/// Active tab override
const TAB_ACTIVE: &str = "\
    !text-[var(--color-primary,#6366f1)] \
    !border-b-[var(--color-primary,#6366f1)]";

/// Tab header bar with clickable labels
#[component]
pub fn Tabs(
    /// Tab labels
    labels: Vec<String>,
    /// Currently active tab index
    active: Signal<usize>,
    /// Called when tab is clicked
    on_change: EventHandler<usize>,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "flex gap-0 \
                border-b border-[var(--color-border,#333)] \
                overflow-x-auto {class}",
            for (i, label) in labels.iter().enumerate() {
                button {
                    key: "{i}",
                    class: if *active.read() == i {
                        format!("{TAB_CLS} {TAB_ACTIVE}")
                    } else {
                        TAB_CLS.to_string()
                    },
                    onclick: move |_| {
                        on_change.call(i);
                    },
                    "{label}"
                }
            }
        }
    }
}

/// Conditionally renders content for a tab index
#[component]
pub fn TabPanel(
    /// Index this panel corresponds to
    index: usize,
    /// Active tab signal
    active: Signal<usize>,
    /// Panel content
    children: Element,
) -> Element {
    let display = if *active.read() == index {
        "block"
    } else {
        "none"
    };

    rsx! {
        div {
            class: "py-5",
            style: "display:{display}",
            {children}
        }
    }
}

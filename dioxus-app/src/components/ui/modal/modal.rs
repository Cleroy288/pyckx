//! Modal — Overlay dialog with backdrop

use crate::components::ui::icon::Icon;
use dioxus::prelude::*;

/// Backdrop overlay classes
const BACKDROP_CLS: &str = "\
    fixed inset-0 z-[1000] \
    flex items-center justify-center \
    bg-black/60 \
    animate-[fadeIn_0.2s_ease]";

/// Dialog panel classes
const MODAL_CLS: &str = "\
    relative w-[90%] max-w-[500px] \
    max-h-[85vh] overflow-y-auto \
    bg-[var(--color-background)] \
    border border-[var(--color-border)] \
    shadow-[0_8px_40px_rgba(0,0,0,0.4)] \
    animate-[slideUp_0.25s_ease]";

/// Header banner classes
const HEADER_CLS: &str = "\
    relative overflow-hidden flex \
    items-center justify-center p-6 \
    bg-[var(--glass-bg)] \
    border-b border-[color-mix(in_srgb,var(--primary)_15%,transparent)] \
    before:content-[''] before:absolute before:inset-0 \
    before:pointer-events-none before:z-0 \
    before:bg-[image:var(--grid-overlay-image)] \
    before:bg-[size:var(--grid-overlay-size)] \
    before:opacity-60";

/// Close button classes
const CLOSE_CLS: &str = "\
    absolute right-6 top-1/2 -translate-y-1/2 \
    z-[1] flex items-center justify-center \
    w-8 h-8 p-0 bg-transparent \
    border border-[var(--color-border)] \
    text-[var(--color-text-secondary)] \
    cursor-pointer shrink-0 \
    transition-all duration-[var(--transition-fast)] \
    hover:border-[var(--color-primary)] \
    hover:text-[var(--color-primary)]";

/// Overlay dialog component
#[component]
pub fn Modal(
    /// Whether the modal is visible
    open: Signal<bool>,
    /// Called when backdrop or X is clicked
    #[props(default)]
    on_close: EventHandler<()>,
    /// Optional title text
    #[props(default)]
    title: Option<String>,
    /// Modal body content
    children: Element,
) -> Element {
    if !*open.read() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "{BACKDROP_CLS}",
            onclick: move |_| on_close.call(()),
            div {
                class: "{MODAL_CLS}",
                onclick: move |ev| ev.stop_propagation(),
                div { class: "{HEADER_CLS}",
                    if let Some(t) = &title {
                        h2 {
                            class: "relative z-[1] m-0 \
                                text-2xl font-bold \
                                tracking-tight \
                                bg-gradient-to-br \
                                from-[var(--color-primary)] \
                                to-[color-mix(in_srgb,var(--primary)_70%,black)] \
                                bg-clip-text text-transparent",
                            "{t}"
                        }
                    }
                    button {
                        class: "{CLOSE_CLS}",
                        onclick: move |_| {
                            on_close.call(());
                        },
                        Icon { icon_name: "X".to_string() }
                    }
                }
                div { class: "p-0",
                    {children}
                }
            }
        }
    }
}

//! PalettePicker — color theme selector card
//!
//! Renders palette swatches + dark mode toggle.
//! Uses PaletteState from context to switch themes.

use crate::domain::palette_data::PALETTES;
use crate::state::palette::PaletteState;
use crate::state::palette_provider::use_palette;
use dioxus::prelude::*;

/// Color theme selector card for the dashboard
#[component]
pub fn PalettePicker() -> Element {
    let state = use_palette();

    rsx! {
        div {
            class: "flex flex-col gap-4 \
                px-6 py-8 w-full \
                bg-[var(--glass-bg)] \
                backdrop-blur-[20px] \
                border border-[var(--color-border)] \
                text-[var(--color-text-primary)]",
            // Swatches row
            div {
                class: "flex justify-center gap-3 \
                    flex-wrap",
                for p in PALETTES.iter() {
                    SwatchBtn {
                        id: p.id,
                        hex: p.preview_hex,
                        name: p.name,
                        state: state,
                    }
                }
            }
            // Dark mode toggle
            DarkToggle { state: state }
        }
    }
}

/// Single color swatch button
#[component]
fn SwatchBtn(
    id: &'static str,
    hex: &'static str,
    name: &'static str,
    mut state: PaletteState,
) -> Element {
    let is_active = state.palette_id() == id;
    let active_cls = if is_active {
        "border-[var(--color-text-primary)] \
         shadow-[0_0_0_3px_var(--glow),0_0_16px_var(--glow)]"
    } else {
        "border-transparent"
    };

    rsx! {
        div { class: "text-center",
            button {
                class: "w-10 h-10 rounded-full \
                    border-2 {active_cls} \
                    cursor-pointer p-0 outline-none \
                    relative \
                    transition-all duration-200 \
                    hover:scale-[1.15] \
                    hover:shadow-[0_0_12px_color-mix(in_srgb,var(--primary)_40%,transparent)]",
                style: "background: {hex}",
                title: "{name}",
                onclick: move |_| {
                    state.set_palette(id);
                },
            }
            div {
                class: "text-xs \
                    text-[var(--color-text-secondary)] \
                    text-center mt-1",
                "{name}"
            }
        }
    }
}

/// Dark mode toggle switch
#[component]
fn DarkToggle(mut state: PaletteState) -> Element {
    let is_dark = state.is_dark();
    let track_cls = if is_dark {
        "bg-[var(--color-primary)] \
         shadow-[0_0_10px_var(--glow)]"
    } else {
        "bg-[var(--muted)]"
    };
    let knob_cls = if is_dark {
        "translate-x-5"
    } else {
        "translate-x-0"
    };

    rsx! {
        div {
            class: "flex items-center justify-center \
                gap-2 pt-2 \
                border-t border-[var(--color-border)]",
            span {
                class: "text-sm \
                    text-[var(--color-text-secondary)]",
                "Dark mode"
            }
            button {
                class: "w-11 h-6 rounded-xl \
                    border border-[var(--color-border)] \
                    {track_cls} \
                    cursor-pointer p-0.5 relative \
                    transition-all duration-200",
                onclick: move |_| {
                    state.toggle_dark();
                },
                div {
                    class: "w-[18px] h-[18px] \
                        rounded-full bg-white \
                        shadow-[0_1px_3px_rgba(0,0,0,0.2)] \
                        transition-transform duration-300 \
                        {knob_cls}",
                }
            }
        }
    }
}

//! PalettePicker — color theme selector card
//!
//! Renders palette swatches + dark mode toggle.
//! Uses PaletteState from context to switch themes.

use crate::domain::palette_data::PALETTES;
use crate::state::palette_provider::use_palette;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/dashboard/palette_picker.module.css"
);

/// Color theme selector card for the dashboard
#[component]
pub fn PalettePicker() -> impl IntoView {
    let state = use_palette();

    view! {
        <div class=style::card>
            <div class=style::swatches>
                {PALETTES.iter().map(|p| {
                    let id = p.id;
                    let hex = p.preview_hex;
                    let name = p.name;
                    view! {
                        <SwatchBtn
                            id=id
                            hex=hex
                            name=name
                            state=state
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
            <DarkToggle state=state />
        </div>
    }
}

/// Single color swatch button
#[component]
fn SwatchBtn(
    id: &'static str,
    hex: &'static str,
    name: &'static str,
    state: crate::state::palette::PaletteState,
) -> impl IntoView {
    let is_active = move || state.palette_id() == id;
    let class = move || {
        if is_active() {
            format!(
                "{} {}",
                style::swatch, style::active
            )
        } else {
            style::swatch.to_string()
        }
    };

    view! {
        <div style="text-align: center">
            <button
                class=class
                style=format!("background: {hex}")
                title=name
                on:click=move |_| {
                    state.set_palette(id);
                }
            />
            <div class=style::label>{name}</div>
        </div>
    }
}

/// Dark mode toggle switch
#[component]
fn DarkToggle(
    state: crate::state::palette::PaletteState,
) -> impl IntoView {
    let toggle_class = move || {
        if state.is_dark() {
            format!(
                "{} {}",
                style::toggle, style::toggle_on
            )
        } else {
            style::toggle.to_string()
        }
    };
    let knob_class = move || {
        if state.is_dark() {
            format!(
                "{} {}",
                style::toggle_knob, style::knob_on
            )
        } else {
            style::toggle_knob.to_string()
        }
    };

    view! {
        <div class=style::toggle_row>
            <span class=style::toggle_label>
                "Dark mode"
            </span>
            <button
                class=toggle_class
                on:click=move |_| state.toggle_dark()
            >
                <div class=knob_class />
            </button>
        </div>
    }
}

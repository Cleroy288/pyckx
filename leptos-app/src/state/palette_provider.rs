//! PaletteProvider component + use_palette hook

use crate::domain::DEFAULT_PALETTE_ID;
use crate::state::palette::PaletteState;
use crate::state::palette_dom;
use leptos::prelude::*;

/// Context provider — reads localStorage, applies CSS
/// vars, re-applies on signal change.
#[component]
pub fn PaletteProvider(children: Children) -> impl IntoView {
    // Read persisted values or fall back to defaults
    let id = palette_dom::read_palette_id()
        .unwrap_or(DEFAULT_PALETTE_ID.to_string());
    let dark = palette_dom::read_is_dark()
        .unwrap_or(false);

    let state = PaletteState::new(id, dark);
    provide_context(state);

    // Apply palette on mount + whenever signals change
    Effect::new(move |_| {
        let colors = state.colors();
        let dark = state.is_dark();
        palette_dom::apply_palette_to_dom(&colors, dark);
        palette_dom::write_palette_id(
            &state.palette_id(),
        );
        palette_dom::write_is_dark(dark);
    });

    children()
}

/// Hook to get palette state from context
pub fn use_palette() -> PaletteState {
    use_context::<PaletteState>().expect(
        "PaletteState not in context. \
         Wrap your app with PaletteProvider.",
    )
}

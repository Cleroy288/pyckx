//! Theme facade — palette catalog, persistence, DOM
//! injection, and the Dioxus `ThemeProvider` / `use_theme`
//! entry points.
//!
//! ```ignore
//! rsx! { ThemeProvider { App {} } }
//! let theme = use_theme();
//! theme.set_palette("ocean-blue");
//! theme.toggle_dark();
//! ```

pub mod dom;
pub mod palette;
pub mod storage;

pub use palette::{
    colors_for_mode, default_palette, palette_by_id,
    resolve, ModeColors, Palette, ThemeColors, PALETTES,
    DEFAULT_PALETTE_ID,
};

use dioxus::prelude::*;

/// Reactive theme handle: current palette id + dark flag.
///
/// Cheaply `Copy` — holds two Dioxus signals. Obtain one
/// via [`use_theme`] inside any component rendered under a
/// [`ThemeProvider`].
#[derive(Clone, Copy, PartialEq)]
pub struct Theme {
    palette_id: Signal<String>,
    is_dark: Signal<bool>,
}

impl Theme {
    /// The currently selected palette (defaulted if the
    /// stored id is unknown).
    pub fn palette(&self) -> &'static Palette {
        resolve(&(self.palette_id)())
    }

    /// Colors for the active mode (light or dark).
    pub fn colors(&self) -> ModeColors {
        colors_for_mode(self.palette(), self.is_dark())
    }

    /// Current palette id.
    pub fn palette_id(&self) -> String {
        (self.palette_id)()
    }

    /// Whether dark mode is active.
    pub fn is_dark(&self) -> bool {
        (self.is_dark)()
    }

    /// Switch to a different palette.
    pub fn set_palette(&mut self, id: &str) {
        self.palette_id.set(id.to_string());
    }

    /// Flip dark mode on/off.
    pub fn toggle_dark(&mut self) {
        let next = !self.is_dark();
        self.is_dark.set(next);
    }
}

/// Root theme provider: loads persisted choices, exposes a
/// [`Theme`] via context, and keeps DOM + storage in sync.
#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let theme = init_theme();
    use_context_provider(|| theme);
    use_effect(move || sync_theme(theme));
    children
}

/// Hook: fetch the [`Theme`] provided by [`ThemeProvider`].
pub fn use_theme() -> Theme {
    use_context::<Theme>()
}

/// Build a [`Theme`] from persisted storage (or defaults).
fn init_theme() -> Theme {
    let id = storage::load_palette_id()
        .unwrap_or_else(|| DEFAULT_PALETTE_ID.to_string());
    let dark = storage::load_is_dark().unwrap_or(false);
    Theme {
        palette_id: use_signal(|| id),
        is_dark: use_signal(|| dark),
    }
}

/// Apply the current theme to the DOM and persist it.
fn sync_theme(theme: Theme) {
    let colors = theme.colors();
    let is_dark = theme.is_dark();
    dom::apply(&colors, is_dark);
    storage::save(&theme.palette_id(), is_dark);
}

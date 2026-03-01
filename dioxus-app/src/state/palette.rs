//! Palette State — reactive palette + dark mode signals

use crate::domain::{
    default_palette, palette_by_id, ModeColors, Palette,
};
use dioxus::prelude::*;

/// Reactive palette state (Clone + Copy via signals)
#[derive(Clone, Copy, PartialEq)]
pub struct PaletteState {
    /// Currently selected palette id
    palette_id: Signal<String>,
    /// Whether dark mode is active
    is_dark: Signal<bool>,
}

impl PaletteState {
    /// Create state from pre-built signals
    pub fn from_signals(
        palette_id: Signal<String>,
        is_dark: Signal<bool>,
    ) -> Self {
        Self { palette_id, is_dark }
    }

    /// Get the current palette (falls back to default)
    pub fn palette(&self) -> &'static Palette {
        let id = (self.palette_id)();
        palette_by_id(&id).unwrap_or(default_palette())
    }

    /// Get colors for the current mode
    pub fn colors(&self) -> ModeColors {
        let p = self.palette();
        if (self.is_dark)() {
            p.colors.dark
        } else {
            p.colors.light
        }
    }

    /// Current palette id
    pub fn palette_id(&self) -> String {
        (self.palette_id)()
    }

    /// Whether dark mode is active (untracked read)
    pub fn is_dark(&self) -> bool {
        self.is_dark.peek().clone()
    }

    /// Switch to a different palette by id
    pub fn set_palette(&mut self, id: &str) {
        self.palette_id.set(id.to_string());
    }

    /// Toggle dark mode on/off
    pub fn toggle_dark(&mut self) {
        let current = (self.is_dark)();
        self.is_dark.set(!current);
    }
}

// TODO: Tests commented out — Dioxus signals require
// a component runtime. PaletteState::new() used
// RwSignal::new() outside components.
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_new_returns_given_id() { ... }
//
//     #[test]
//     fn test_new_dark_mode_flag() { ... }
//
//     #[test]
//     fn test_default_uses_teal_tech() { ... }
//
//     #[test]
//     fn test_palette_returns_correct_palette() { ... }
//
//     #[test]
//     fn test_palette_unknown_id_fallback() { ... }
//
//     #[test]
//     fn test_set_palette_changes_id() { ... }
//
//     #[test]
//     fn test_toggle_dark_flips_mode() { ... }
//
//     #[test]
//     fn test_colors_light_mode() { ... }
//
//     #[test]
//     fn test_colors_dark_mode() { ... }
// }

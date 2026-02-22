//! Palette State — reactive palette + dark mode signals

use crate::domain::{
    default_palette, palette_by_id, ModeColors, Palette,
    DEFAULT_PALETTE_ID,
};
use leptos::prelude::*;

/// Reactive palette state (Clone + Copy via signals)
#[derive(Clone, Copy)]
pub struct PaletteState {
    /// Currently selected palette id
    palette_id: RwSignal<String>,
    /// Whether dark mode is active
    is_dark: RwSignal<bool>,
}

impl PaletteState {
    /// Create state with given palette id and dark mode
    pub fn new(id: String, dark: bool) -> Self {
        Self {
            palette_id: RwSignal::new(id),
            is_dark: RwSignal::new(dark),
        }
    }

    /// Get the current palette (falls back to default)
    pub fn palette(&self) -> &'static Palette {
        let id = self.palette_id.get();
        palette_by_id(&id).unwrap_or(default_palette())
    }

    /// Get colors for the current mode
    pub fn colors(&self) -> ModeColors {
        let p = self.palette();
        if self.is_dark.get() {
            p.colors.dark
        } else {
            p.colors.light
        }
    }

    /// Current palette id
    pub fn palette_id(&self) -> String {
        self.palette_id.get()
    }

    /// Whether dark mode is active
    pub fn is_dark(&self) -> bool {
        self.is_dark.get()
    }

    /// Switch to a different palette by id
    pub fn set_palette(&self, id: &str) {
        self.palette_id
            .set(id.to_string());
    }

    /// Toggle dark mode on/off
    pub fn toggle_dark(&self) {
        self.is_dark.set(!self.is_dark.get());
    }
}

impl Default for PaletteState {
    fn default() -> Self {
        Self::new(
            DEFAULT_PALETTE_ID.to_string(),
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_given_id() {
        let state = PaletteState::new(
            "ocean-blue".into(),
            false,
        );
        assert_eq!(state.palette_id(), "ocean-blue");
    }

    #[test]
    fn test_new_dark_mode_flag() {
        let state =
            PaletteState::new("teal-tech".into(), true);
        assert!(state.is_dark());
    }

    #[test]
    fn test_default_uses_teal_tech() {
        let state = PaletteState::default();
        assert_eq!(state.palette_id(), DEFAULT_PALETTE_ID);
        assert!(!state.is_dark());
    }

    #[test]
    fn test_palette_returns_correct_palette() {
        let state = PaletteState::default();
        assert_eq!(state.palette().id, "teal-tech");
    }

    #[test]
    fn test_palette_unknown_id_fallback() {
        let state = PaletteState::new(
            "nonexistent".into(),
            false,
        );
        let p = state.palette();
        assert_eq!(p.id, default_palette().id);
    }

    #[test]
    fn test_set_palette_changes_id() {
        let state = PaletteState::default();
        state.set_palette("purple-dream");
        assert_eq!(state.palette_id(), "purple-dream");
    }

    #[test]
    fn test_toggle_dark_flips_mode() {
        let state = PaletteState::default();
        assert!(!state.is_dark());
        state.toggle_dark();
        assert!(state.is_dark());
        state.toggle_dark();
        assert!(!state.is_dark());
    }

    #[test]
    fn test_colors_light_mode() {
        let state = PaletteState::new(
            "teal-tech".into(),
            false,
        );
        let colors = state.colors();
        let expected = state.palette().colors.light;
        assert_eq!(colors, expected);
    }

    #[test]
    fn test_colors_dark_mode() {
        let state = PaletteState::new(
            "teal-tech".into(),
            true,
        );
        let colors = state.colors();
        let expected = state.palette().colors.dark;
        assert_eq!(colors, expected);
    }
}

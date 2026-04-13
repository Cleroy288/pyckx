//! Palette catalog — re-exports domain palette types and
//! the builtin palette list. The domain layer owns the raw
//! data; this module is the theme-layer entry point.

pub use crate::domain::palette_types::{
    ModeColors, Palette, ThemeColors, DEFAULT_PALETTE_ID,
};
pub use crate::domain::{default_palette, palette_by_id};

pub use crate::domain::palette_data::PALETTES;

/// Resolve a palette id to a palette, falling back to the
/// default when the id is unknown or empty.
pub fn resolve(id: &str) -> &'static Palette {
    palette_by_id(id).unwrap_or_else(default_palette)
}

/// Pick the active color set (light or dark) from a palette.
pub fn colors_for_mode(
    palette: &'static Palette,
    is_dark: bool,
) -> ModeColors {
    if is_dark {
        palette.colors.dark
    } else {
        palette.colors.light
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_with_known_id_returns_palette() {
        let p = resolve("teal-tech");
        assert_eq!(p.id, "teal-tech");
    }

    #[test]
    fn test_resolve_with_unknown_id_returns_default() {
        let p = resolve("does-not-exist");
        assert_eq!(p.id, DEFAULT_PALETTE_ID);
    }

    #[test]
    fn test_resolve_with_empty_id_returns_default() {
        let p = resolve("");
        assert_eq!(p.id, DEFAULT_PALETTE_ID);
    }

    #[test]
    fn test_colors_for_mode_light_returns_light() {
        let p = resolve("teal-tech");
        let c = colors_for_mode(p, false);
        assert_eq!(c, p.colors.light);
    }

    #[test]
    fn test_colors_for_mode_dark_returns_dark() {
        let p = resolve("teal-tech");
        let c = colors_for_mode(p, true);
        assert_eq!(c, p.colors.dark);
    }
}

//! Palette domain types — pure data, no framework deps

use crate::domain::palette_data::PALETTES;

/// Default palette identifier
pub const DEFAULT_PALETTE_ID: &str = "teal-tech";

/// Colors for one mode (light or dark)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModeColors {
    pub primary: &'static str,
    pub primary_fg: &'static str,
    pub secondary: &'static str,
    pub secondary_fg: &'static str,
    pub accent: &'static str,
    pub accent_fg: &'static str,
    pub muted: &'static str,
    pub muted_fg: &'static str,
    pub background: &'static str,
    pub foreground: &'static str,
    pub card: &'static str,
    pub card_fg: &'static str,
    pub border: &'static str,
    pub input: &'static str,
    pub ring: &'static str,
    pub destructive: &'static str,
    pub destructive_fg: &'static str,
    pub glow: &'static str,
    pub grid: &'static str,
}

/// Light + dark color sets for a palette
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColors {
    pub light: ModeColors,
    pub dark: ModeColors,
}

/// A named color palette
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub preview_hex: &'static str,
    pub colors: ThemeColors,
}

/// Lookup a palette by id, returns None if not found
pub fn palette_by_id(id: &str) -> Option<&'static Palette> {
    PALETTES.iter().find(|p| p.id == id)
}

/// Returns the default palette (teal-tech)
pub fn default_palette() -> &'static Palette {
    palette_by_id(DEFAULT_PALETTE_ID)
        .unwrap_or(&PALETTES[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_by_id_with_valid_id_returns_palette() {
        let cases = vec![
            ("teal-tech", "Teal Tech"),
            ("purple-dream", "Purple Dream"),
            ("ocean-blue", "Ocean Blue"),
            ("emerald-forest", "Emerald Forest"),
            ("sunset-orange", "Sunset Orange"),
        ];
        for (id, expected_name) in cases {
            let p = palette_by_id(id);
            assert!(
                p.is_some(),
                "palette '{}' should exist",
                id,
            );
            assert_eq!(
                p.unwrap().name, expected_name,
                "palette '{}' name mismatch",
                id,
            );
        }
    }

    #[test]
    fn palette_by_id_with_unknown_returns_none() {
        assert!(palette_by_id("nonexistent").is_none());
    }

    #[test]
    fn default_palette_returns_teal_tech() {
        let p = default_palette();
        assert_eq!(p.id, "teal-tech");
        assert_eq!(p.name, "Teal Tech");
    }

    #[test]
    fn palettes_have_distinct_ids() {
        let ids: Vec<&str> =
            PALETTES.iter().map(|p| p.id).collect();
        for (i, id) in ids.iter().enumerate() {
            assert!(
                !ids[i + 1..].contains(id),
                "duplicate palette id: {}",
                id,
            );
        }
    }

    #[test]
    fn palettes_all_have_valid_fields() {
        for p in PALETTES.iter() {
            assert!(
                !p.id.is_empty(),
                "palette id must not be empty",
            );
            assert!(
                !p.name.is_empty(),
                "{}: name must not be empty",
                p.id,
            );
            assert!(
                !p.description.is_empty(),
                "{}: description must not be empty",
                p.id,
            );
            assert!(
                p.preview_hex.starts_with('#'),
                "{}: preview_hex must start with #",
                p.id,
            );
        }
    }
}

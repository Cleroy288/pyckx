//! Palette domain types — pure data, no framework deps

/// Default palette identifier
pub const DEFAULT_PALETTE_ID: &str = "purple-dream";

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


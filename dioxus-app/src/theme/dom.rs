//! Injects the active palette into the document as CSS
//! custom properties on the `:root` element.

use crate::theme::palette::ModeColors;
use wasm_bindgen::JsCast;

/// Apply a full theme (palette colors + glass effect vars)
/// to `:root`. Silently no-ops outside a browser context.
pub fn apply(colors: &ModeColors, is_dark: bool) {
    let Some(style) = root_style() else {
        return;
    };
    for (prop, val) in palette_vars(colors) {
        let _ = style.set_property(prop, val);
    }
    for (prop, val) in glass_vars(is_dark) {
        let _ = style.set_property(prop, val);
    }
}

/// Build the list of CSS variables for a color set.
fn palette_vars(c: &ModeColors) -> [(&'static str, &str); 19]
{
    [
        ("--primary", c.primary),
        ("--primary-foreground", c.primary_fg),
        ("--secondary", c.secondary),
        ("--secondary-foreground", c.secondary_fg),
        ("--accent", c.accent),
        ("--accent-foreground", c.accent_fg),
        ("--muted", c.muted),
        ("--muted-foreground", c.muted_fg),
        ("--background", c.background),
        ("--foreground", c.foreground),
        ("--card", c.card),
        ("--card-foreground", c.card_fg),
        ("--border", c.border),
        ("--input", c.input),
        ("--ring", c.ring),
        ("--destructive", c.destructive),
        ("--destructive-foreground", c.destructive_fg),
        ("--glow", c.glow),
        ("--grid", c.grid),
    ]
}

/// Glass-effect variables for the active mode.
fn glass_vars(is_dark: bool) -> [(&'static str, &'static str); 4] {
    if is_dark {
        [
            ("--glass-bg", "rgba(255,255,255,0.08)"),
            ("--glass-bg-hover", "rgba(255,255,255,0.12)"),
            (
                "--glass-border",
                "1px solid rgba(255,255,255,0.10)",
            ),
            (
                "--glass-border-hover",
                "1px solid rgba(255,255,255,0.18)",
            ),
        ]
    } else {
        [
            ("--glass-bg", "rgba(255,255,255,0.5)"),
            ("--glass-bg-hover", "rgba(255,255,255,0.6)"),
            (
                "--glass-border",
                "1px solid rgba(255,255,255,0.12)",
            ),
            (
                "--glass-border-hover",
                "1px solid rgba(255,255,255,0.20)",
            ),
        ]
    }
}

/// Fetch the `:root` element's style declaration.
fn root_style() -> Option<web_sys::CssStyleDeclaration> {
    let doc = web_sys::window()?.document()?;
    let el = doc.document_element()?;
    let html: web_sys::HtmlElement = el.dyn_into().ok()?;
    Some(html.style())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_colors() -> ModeColors {
        ModeColors {
            primary: "p",
            primary_fg: "pf",
            secondary: "s",
            secondary_fg: "sf",
            accent: "a",
            accent_fg: "af",
            muted: "m",
            muted_fg: "mf",
            background: "bg",
            foreground: "fg",
            card: "c",
            card_fg: "cf",
            border: "b",
            input: "i",
            ring: "r",
            destructive: "d",
            destructive_fg: "df",
            glow: "g",
            grid: "gr",
        }
    }

    #[test]
    fn test_palette_vars_has_nineteen_entries() {
        let colors = sample_colors();
        let vars = palette_vars(&colors);
        assert_eq!(vars.len(), 19);
    }

    #[test]
    fn test_palette_vars_all_prefixed_with_dash_dash() {
        let colors = sample_colors();
        let vars = palette_vars(&colors);
        for (prop, _) in vars {
            assert!(prop.starts_with("--"));
        }
    }

    #[test]
    fn test_palette_vars_maps_primary_correctly() {
        let colors = sample_colors();
        let vars = palette_vars(&colors);
        assert_eq!(vars[0], ("--primary", "p"));
    }

    #[test]
    fn test_glass_vars_dark_uses_low_alpha() {
        let vars = glass_vars(true);
        assert!(vars[0].1.contains("0.08"));
    }

    #[test]
    fn test_glass_vars_light_uses_high_alpha() {
        let vars = glass_vars(false);
        assert!(vars[0].1.contains("0.5"));
    }
}

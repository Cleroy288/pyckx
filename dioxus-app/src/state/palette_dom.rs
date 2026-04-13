//! DOM helpers — apply palette CSS variables to :root

use crate::domain::ModeColors;
use wasm_bindgen::JsCast;

pub use super::palette_storage::{
    read_is_dark, read_last_route, read_palette_id,
    write_is_dark, write_last_route, write_palette_id,
};

/// Set all palette CSS variables on :root
pub fn apply_palette_to_dom(
    colors: &ModeColors,
    is_dark: bool,
) {
    let Some(style) = root_style() else {
        return;
    };
    let vars: [(&str, &str); 19] = [
        ("--primary", colors.primary),
        ("--primary-foreground", colors.primary_fg),
        ("--secondary", colors.secondary),
        ("--secondary-foreground", colors.secondary_fg),
        ("--accent", colors.accent),
        ("--accent-foreground", colors.accent_fg),
        ("--muted", colors.muted),
        ("--muted-foreground", colors.muted_fg),
        ("--background", colors.background),
        ("--foreground", colors.foreground),
        ("--card", colors.card),
        ("--card-foreground", colors.card_fg),
        ("--border", colors.border),
        ("--input", colors.input),
        ("--ring", colors.ring),
        ("--destructive", colors.destructive),
        ("--destructive-foreground", colors.destructive_fg),
        ("--glow", colors.glow),
        ("--grid", colors.grid),
    ];
    for (prop, val) in vars {
        let _ = style.set_property(prop, val);
    }
    apply_glass_vars(&style, is_dark);
}

/// Set glass variables for the current mode
fn apply_glass_vars(
    style: &web_sys::CssStyleDeclaration,
    is_dark: bool,
) {
    let glass = if is_dark {
        [
            ("--glass-bg", "rgba(255,255,255,0.08)"),
            ("--glass-bg-hover", "rgba(255,255,255,0.12)"),
            ("--glass-border", "1px solid rgba(255,255,255,0.10)"),
            ("--glass-border-hover", "1px solid rgba(255,255,255,0.18)"),
        ]
    } else {
        [
            ("--glass-bg", "rgba(255,255,255,0.5)"),
            ("--glass-bg-hover", "rgba(255,255,255,0.6)"),
            ("--glass-border", "1px solid rgba(255,255,255,0.12)"),
            ("--glass-border-hover", "1px solid rgba(255,255,255,0.20)"),
        ]
    };
    for (prop, val) in glass {
        let _ = style.set_property(prop, val);
    }
}

/// Get the CSSStyleDeclaration of documentElement
fn root_style() -> Option<web_sys::CssStyleDeclaration>
{
    let doc = web_sys::window()?.document()?;
    let el = doc.document_element()?;
    let html: web_sys::HtmlElement =
        el.dyn_into().ok()?;
    Some(html.style())
}

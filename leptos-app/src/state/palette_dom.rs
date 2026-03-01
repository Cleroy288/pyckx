//! DOM helpers — apply palette CSS vars, localStorage I/O

use crate::domain::ModeColors;
use wasm_bindgen::JsCast;

/// localStorage key for persisted palette id
const STORAGE_KEY_PALETTE: &str = "pyckx-palette-id";
/// localStorage key for persisted dark mode flag
const STORAGE_KEY_DARK: &str = "pyckx-dark-mode";
/// localStorage key for last visited route
const STORAGE_KEY_LAST_ROUTE: &str =
    "pyckx-last-route";

/// Read saved palette id from localStorage
pub fn read_palette_id() -> Option<String> {
    read_storage(STORAGE_KEY_PALETTE)
}

/// Read saved dark mode flag from localStorage
pub fn read_is_dark() -> Option<bool> {
    read_storage(STORAGE_KEY_DARK)
        .and_then(|v| v.parse::<bool>().ok())
}

/// Persist palette id to localStorage
pub fn write_palette_id(id: &str) {
    write_storage(STORAGE_KEY_PALETTE, id);
}

/// Persist dark mode flag to localStorage
pub fn write_is_dark(dark: bool) {
    write_storage(STORAGE_KEY_DARK, &dark.to_string());
}

/// Read last visited route from localStorage
pub fn read_last_route() -> Option<String> {
    read_storage(STORAGE_KEY_LAST_ROUTE)
}

/// Persist last visited route to localStorage
pub fn write_last_route(path: &str) {
    write_storage(STORAGE_KEY_LAST_ROUTE, path);
}

/// Set all palette CSS variables on :root
pub fn apply_palette_to_dom(
    colors: &ModeColors,
    is_dark: bool,
) {
    let Some(style) = root_style() else {
        return;
    };
    // Palette color variables
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
    // Glass variables adapt to light/dark mode
    apply_glass_vars(&style, is_dark);
}

/// Set glass variables for the current mode
fn apply_glass_vars(
    style: &web_sys::CssStyleDeclaration,
    is_dark: bool,
) {
    let glass = if is_dark {
        [
            ("--glass-bg", "rgba(255,255,255,0.06)"),
            ("--glass-bg-hover", "rgba(255,255,255,0.10)"),
            ("--glass-border", "1px solid rgba(255,255,255,0.08)"),
            ("--glass-border-hover", "1px solid rgba(255,255,255,0.15)"),
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

// ── private helpers ─────────────────────────

/// Get the CSSStyleDeclaration of documentElement
fn root_style() -> Option<web_sys::CssStyleDeclaration> {
    let doc = web_sys::window()?.document()?;
    let el = doc.document_element()?;
    let html: web_sys::HtmlElement =
        el.dyn_into().ok()?;
    Some(html.style())
}

/// Read a value from localStorage
fn read_storage(key: &str) -> Option<String> {
    let storage = web_sys::window()?
        .local_storage()
        .ok()??;
    storage.get_item(key).ok()?
}

/// Write a value to localStorage
fn write_storage(key: &str, value: &str) {
    let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok()?)
    else {
        return;
    };
    let _ = storage.set_item(key, value);
}

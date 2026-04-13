//! localStorage persistence for the user's theme choices
//! (selected palette id + dark mode flag).

const KEY_PALETTE: &str = "pyckx-palette-id";
const KEY_DARK: &str = "pyckx-dark-mode";

/// Read the saved palette id, if any.
pub fn load_palette_id() -> Option<String> {
    read(KEY_PALETTE)
}

/// Read the saved dark-mode flag, if any.
pub fn load_is_dark() -> Option<bool> {
    read(KEY_DARK).and_then(|v| v.parse::<bool>().ok())
}

/// Persist the palette id and the dark-mode flag.
pub fn save(palette_id: &str, is_dark: bool) {
    write(KEY_PALETTE, palette_id);
    write(KEY_DARK, &is_dark.to_string());
}

/// Read a string value from localStorage.
fn read(key: &str) -> Option<String> {
    let storage = web_sys::window()?.local_storage().ok()??;
    storage.get_item(key).ok()?
}

/// Write a string value to localStorage; ignore failures.
fn write(key: &str, value: &str) {
    let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok()?)
    else {
        return;
    };
    let _ = storage.set_item(key, value);
}

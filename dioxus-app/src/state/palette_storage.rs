//! localStorage I/O for palette and route state

/// localStorage key for persisted palette id
const KEY_PALETTE: &str = "pyckx-palette-id";
/// localStorage key for persisted dark mode flag
const KEY_DARK: &str = "pyckx-dark-mode";
/// localStorage key for last visited route
const KEY_LAST_ROUTE: &str = "pyckx-last-route";
/// localStorage key for last used app name
const KEY_LAST_APP: &str = "pyckx-last-app";
/// localStorage key for last visited page path
const KEY_LAST_PAGE: &str = "pyckx-last-page";

/// Read saved palette id from localStorage
pub fn read_palette_id() -> Option<String> {
    read_storage(KEY_PALETTE)
}

/// Read saved dark mode flag from localStorage
pub fn read_is_dark() -> Option<bool> {
    read_storage(KEY_DARK)
        .and_then(|v| v.parse::<bool>().ok())
}

/// Persist palette id to localStorage
pub fn write_palette_id(id: &str) {
    write_storage(KEY_PALETTE, id);
}

/// Persist dark mode flag to localStorage
pub fn write_is_dark(dark: bool) {
    write_storage(KEY_DARK, &dark.to_string());
}

/// Read last visited route from localStorage
pub fn read_last_route() -> Option<String> {
    read_storage(KEY_LAST_ROUTE)
}

/// Persist last visited route to localStorage
pub fn write_last_route(path: &str) {
    write_storage(KEY_LAST_ROUTE, path);
}

/// Read last used app name from localStorage
pub fn read_last_app() -> Option<String> {
    read_storage(KEY_LAST_APP)
}

/// Persist last used app name to localStorage
pub fn write_last_app(app_name: &str) {
    write_storage(KEY_LAST_APP, app_name);
}

/// Read last visited page path from localStorage
pub fn read_last_page() -> Option<String> {
    read_storage(KEY_LAST_PAGE)
}

/// Persist last visited page path to localStorage
pub fn write_last_page(path: &str) {
    write_storage(KEY_LAST_PAGE, path);
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

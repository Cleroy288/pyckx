//! PageNav — helpers for page navigation
//! Provides page list and pathname utilities
//! used by HomeTopBar.

/// (path, label) pair for a navigable page
pub type PageEntry = (&'static str, &'static str);

/// Static nav entries shown in the top bar
pub const NAV_PAGES: [PageEntry; 2] = [
    ("/home", "Accueil"),
    ("/courses", "Cours"),
];

/// Read browser pathname via web_sys
pub fn current_pathname() -> String {
    web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| "/home".to_string())
}

//! Mobile detection hook
//!
//! Returns a reactive signal that tracks whether
//! the viewport width is below the mobile threshold.

use dioxus::prelude::*;

/// Max width considered "mobile" (px)
const MOBILE_MAX_WIDTH: f64 = 768.0;

/// Returns true if the viewport is mobile-sized.
/// Checks on mount only (no resize listener needed
/// for a desktop-only gate — user won't resize
/// mid-exercise).
pub fn use_is_mobile() -> bool {
    use_memo(check_mobile).read().clone()
}

/// Check current viewport width
fn check_mobile() -> bool {
    web_sys::window()
        .and_then(|w| w.inner_width().ok())
        .and_then(|v| v.as_f64())
        .map(|w| w < MOBILE_MAX_WIDTH)
        .unwrap_or(false)
}

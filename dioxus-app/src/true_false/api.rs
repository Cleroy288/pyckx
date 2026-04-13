//! True/False — HTTP client (list + AI generation).
//!
//! Thin facade over `crate::api::true_false` so the
//! feature exposes a self-contained call surface and
//! callers never reach into the legacy api module.

use crate::api::true_false as legacy;

use super::types::{CreateResponse, TrueFalseSet};

/// Fetch every true/false set for the current user.
pub async fn fetch_sets() -> Result<Vec<TrueFalseSet>, String> {
    legacy::get_true_false_sets().await
}

/// Create a true/false set via AI from a multipart form.
pub async fn create_set(
    form: &web_sys::FormData,
) -> Result<CreateResponse, String> {
    legacy::create_true_false(form).await
}

/// Find a single set by id within the user collection.
pub async fn find_set_by_id(
    id: String,
) -> Option<TrueFalseSet> {
    fetch_sets()
        .await
        .ok()?
        .into_iter()
        .find(|set| set.id == id)
}

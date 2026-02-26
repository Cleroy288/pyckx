//! Keywords API — list and AI generation

use super::{endpoints, helpers};
use crate::domain::keywords_types::*;

helpers::impl_sets_response!(
    KeywordSetListResponse, KeywordSet
);

/// Get all keyword sets
pub async fn get_keyword_sets(
) -> Result<Vec<KeywordSet>, String> {
    helpers::get_sets::<KeywordSetListResponse>(
        endpoints::KEYWORDS,
    )
    .await
}

/// Create keywords via AI (multipart upload)
pub async fn create_keywords(
    form: &web_sys::FormData,
) -> Result<CreateKeywordsResponse, String> {
    helpers::post_multipart(
        endpoints::KEYWORDS,
        form,
    )
    .await
}

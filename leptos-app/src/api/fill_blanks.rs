//! Fill Blanks API — list and AI generation

use super::{endpoints, helpers};
use crate::domain::fill_blank_types::*;

helpers::impl_sets_response!(
    FillBlankSetListResponse, FillBlankSet
);

/// Get all fill blank sets
pub async fn get_fill_blank_sets(
) -> Result<Vec<FillBlankSet>, String> {
    helpers::get_sets::<FillBlankSetListResponse>(
        endpoints::FILL_BLANKS,
    )
    .await
}

/// Create fill blanks via AI (multipart upload)
pub async fn create_fill_blanks(
    form: &web_sys::FormData,
) -> Result<CreateFillBlankResponse, String> {
    helpers::post_multipart(
        endpoints::FILL_BLANKS,
        form,
    )
    .await
}

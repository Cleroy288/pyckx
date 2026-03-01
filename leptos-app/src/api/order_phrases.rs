//! Order Phrases API — list and AI generation

use super::{endpoints, helpers};
use crate::domain::order_phrase_types::*;

helpers::impl_sets_response!(
    OrderPhraseSetListResponse, OrderPhraseSet
);

/// Get all order phrase sets
pub async fn get_order_phrase_sets(
) -> Result<Vec<OrderPhraseSet>, String> {
    helpers::get_sets::<OrderPhraseSetListResponse>(
        endpoints::ORDER_PHRASES,
    )
    .await
}

/// Create order phrases via AI (multipart upload)
pub async fn create_order_phrases(
    form: &web_sys::FormData,
) -> Result<CreateOrderPhraseResponse, String> {
    helpers::post_multipart(
        endpoints::ORDER_PHRASES,
        form,
    )
    .await
}

//! Order Phrases — HTTP client (list + AI generation).

use crate::api::{endpoints, helpers};

use super::types::{
    CreateOrderPhraseResponse, OrderPhraseSet,
    OrderPhraseSetListResponse,
};

helpers::impl_sets_response!(
    OrderPhraseSetListResponse,
    OrderPhraseSet
);

/// Fetch every order phrase set for the current user.
pub async fn fetch_sets(
) -> Result<Vec<OrderPhraseSet>, String> {
    helpers::get_sets::<OrderPhraseSetListResponse>(
        endpoints::ORDER_PHRASES,
    )
    .await
}

/// Create order phrases via AI from a multipart form.
pub async fn create_set(
    form: &web_sys::FormData,
) -> Result<CreateOrderPhraseResponse, String> {
    helpers::post_multipart(
        endpoints::ORDER_PHRASES,
        form,
    )
    .await
}

/// Find a set by id within the current user collection.
pub async fn find_set_by_id(
    id: String,
) -> Option<OrderPhraseSet> {
    fetch_sets()
        .await
        .ok()?
        .into_iter()
        .find(|set| set.id == id)
}

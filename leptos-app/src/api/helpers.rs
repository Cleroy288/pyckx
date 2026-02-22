//! Shared HTTP helpers for API calls

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};

use crate::domain::auth_types::ApiError;

/// GET request returning JSON
pub async fn get_json<T: DeserializeOwned>(
    url: &str,
) -> Result<T, String> {
    let resp = Request::get(url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        resp.json::<T>()
            .await
            .map_err(|e| format!("Parse error: {e}"))
    } else {
        Err(parse_error(resp).await)
    }
}

/// POST request with JSON body returning JSON
pub async fn post_json<B, T>(
    url: &str,
    body: &B,
) -> Result<T, String>
where
    B: Serialize,
    T: DeserializeOwned,
{
    let resp = Request::post(url)
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(body)
        .map_err(|e| format!("Serialize error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        resp.json::<T>()
            .await
            .map_err(|e| format!("Parse error: {e}"))
    } else {
        Err(parse_error(resp).await)
    }
}

/// PUT request with JSON body returning JSON
pub async fn put_json<B, T>(
    url: &str,
    body: &B,
) -> Result<T, String>
where
    B: Serialize,
    T: DeserializeOwned,
{
    let resp = Request::put(url)
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(body)
        .map_err(|e| format!("Serialize error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        resp.json::<T>()
            .await
            .map_err(|e| format!("Parse error: {e}"))
    } else {
        Err(parse_error(resp).await)
    }
}

/// DELETE request
pub async fn delete_request(
    url: &str,
) -> Result<bool, String> {
    let resp = Request::delete(url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        Ok(true)
    } else if resp.status() == 404 {
        Ok(false)
    } else {
        Err(parse_error(resp).await)
    }
}

/// POST multipart form data returning JSON
pub async fn post_multipart<T: DeserializeOwned>(
    url: &str,
    form: &web_sys::FormData,
) -> Result<T, String> {
    let resp = Request::post(url)
        .credentials(web_sys::RequestCredentials::Include)
        .body(form.clone())
        .map_err(|e| format!("Body error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        resp.json::<T>()
            .await
            .map_err(|e| format!("Parse error: {e}"))
    } else {
        Err(parse_error(resp).await)
    }
}

/// Trait for list responses with a `sets` field
pub trait SetsResponse: DeserializeOwned {
    type Item;
    fn into_sets(self) -> Vec<Self::Item>;
}

/// DRY macro for SetsResponse impls where the
/// response struct has a `sets: Vec<Item>` field.
macro_rules! impl_sets_response {
    ($resp:ty, $item:ty) => {
        impl $crate::api::helpers::SetsResponse
            for $resp
        {
            type Item = $item;
            fn into_sets(self) -> Vec<$item> {
                self.sets
            }
        }
    };
}
pub(crate) use impl_sets_response;

/// Generic helper: GET a list endpoint,
/// extract the `sets` vec
pub async fn get_sets<R: SetsResponse>(
    url: &str,
) -> Result<Vec<R::Item>, String> {
    let resp: R = get_json(url).await?;
    Ok(resp.into_sets())
}

/// Extract error message from a failed response
async fn parse_error(
    resp: gloo_net::http::Response,
) -> String {
    let status = resp.status();
    resp.json::<ApiError>()
        .await
        .map(|e| e.error)
        .unwrap_or_else(|_| {
            format!("Request failed ({status})")
        })
}

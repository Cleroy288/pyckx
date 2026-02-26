//! Intello API — QCM CRUD operations

use super::{endpoints, helpers};
use crate::domain::intello_types::IntelloApiError;
use crate::domain::qcm_types::{
    QcmSet, QcmSetListResponse, QcmSuccessResponse,
    QuickQcmApiResponse,
};
use gloo_net::http::Request;

// Re-export domain types for backwards compat
pub use crate::domain::qcm_types::{
    CreateQcmQuestionInput, CreateQcmSetRequest,
};

/// Create a new QCM set
/// POST /api/intello/qcm
pub async fn create_qcm_set(
    input: CreateQcmSetRequest,
) -> Result<QcmSet, String> {
    let resp = Request::post(endpoints::QCM)
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&input)
        .map_err(|e| format!("Serialize error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        let data: QcmSuccessResponse = resp
            .json()
            .await
            .map_err(|e| format!("Parse error: {e}"))?;
        data.set.ok_or_else(|| {
            "No set returned from API".to_string()
        })
    } else {
        let msg = resp
            .json::<IntelloApiError>()
            .await
            .map(|e| {
                if let Some(f) = e.field {
                    format!("{f}: {}", e.error)
                } else {
                    e.error
                }
            })
            .unwrap_or_else(|_| {
                format!(
                    "Failed to create QCM set ({})",
                    resp.status()
                )
            });
        Err(msg)
    }
}

/// Get all QCM sets for current user
pub async fn get_qcm_sets(
) -> Result<Vec<QcmSet>, String> {
    let data: QcmSetListResponse =
        helpers::get_json(endpoints::QCM).await?;
    Ok(data.sets)
}

/// Get a specific QCM set by ID
pub async fn get_qcm_set(
    id: &str,
) -> Result<Option<QcmSet>, String> {
    let url = format!("{}/{}", endpoints::QCM, id);
    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.ok() {
        let set: QcmSet = resp
            .json()
            .await
            .map_err(|e| format!("Parse error: {e}"))?;
        Ok(Some(set))
    } else if resp.status() == 404 {
        Ok(None)
    } else {
        Err(format!(
            "Failed to fetch QCM set ({})",
            resp.status()
        ))
    }
}

/// Delete a QCM set by ID
pub async fn delete_qcm_set(
    id: &str,
) -> Result<bool, String> {
    let url = format!("{}/{}", endpoints::QCM, id);
    helpers::delete_request(&url).await
}

/// Owned-string variant for use in closures
pub async fn delete_qcm_set_owned(
    id: String,
) -> Result<bool, String> {
    delete_qcm_set(&id).await
}

/// Generate ephemeral QCM from uploaded documents
pub async fn generate_quick_qcm(
    form: &web_sys::FormData,
) -> Result<QcmSet, String> {
    let resp: QuickQcmApiResponse =
        helpers::post_multipart(
            endpoints::QCM_QUICK,
            form,
        )
        .await?;
    Ok(resp.into_qcm_set())
}

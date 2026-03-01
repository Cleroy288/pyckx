//! Collection API — DVD CRUD operations

use super::{endpoints, helpers};
use crate::domain::collection_types::*;

/// Get all DVDs for current user
pub async fn get_dvds() -> Result<Vec<Dvd>, String> {
    let resp: DvdListResponse =
        helpers::get_json(endpoints::DVDS).await?;
    Ok(resp.dvds)
}

/// Add a new DVD
pub async fn add_dvd(
    req: &AddDvdRequest,
) -> Result<Dvd, String> {
    let resp: DvdSuccessResponse =
        helpers::post_json(endpoints::DVDS, req).await?;
    Ok(resp.dvd)
}

/// Update an existing DVD
pub async fn update_dvd(
    id: &str,
    req: &UpdateDvdRequest,
) -> Result<Dvd, String> {
    let url = format!("{}/{}", endpoints::DVDS, id);
    let resp: DvdSuccessResponse =
        helpers::put_json(&url, req).await?;
    Ok(resp.dvd)
}

/// Delete a DVD by ID
pub async fn delete_dvd(
    id: &str,
) -> Result<bool, String> {
    let url = format!("{}/{}", endpoints::DVDS, id);
    helpers::delete_request(&url).await
}

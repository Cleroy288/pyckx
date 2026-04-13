//! HTTP client for the DVD collection endpoints.

use crate::api::{endpoints, helpers};
use crate::collection::types::{
    AddDvdRequest, Dvd, DvdListResponse,
    DvdSuccessResponse, UpdateDvdRequest,
};

/// Fetch every DVD owned by the current user.
pub async fn get_dvds() -> Result<Vec<Dvd>, String> {
    let resp: DvdListResponse =
        helpers::get_json(endpoints::DVDS).await?;
    Ok(resp.dvds)
}

/// Create a DVD and return the stored record.
pub async fn add_dvd(
    req: &AddDvdRequest,
) -> Result<Dvd, String> {
    let resp: DvdSuccessResponse =
        helpers::post_json(endpoints::DVDS, req).await?;
    Ok(resp.dvd)
}

/// Update an existing DVD by id.
pub async fn update_dvd(
    id: &str,
    req: &UpdateDvdRequest,
) -> Result<Dvd, String> {
    let url = dvd_url(id);
    let resp: DvdSuccessResponse =
        helpers::put_json(&url, req).await?;
    Ok(resp.dvd)
}

/// Delete a DVD by id. Returns true when the server
/// confirms the deletion.
pub async fn delete_dvd(
    id: &str,
) -> Result<bool, String> {
    let url = dvd_url(id);
    helpers::delete_request(&url).await
}

/// Build the URL for a single DVD resource.
fn dvd_url(id: &str) -> String {
    format!("{}/{}", endpoints::DVDS, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dvd_url_appends_id() {
        let url = dvd_url("abc");
        let expected =
            format!("{}/abc", endpoints::DVDS);
        assert_eq!(url, expected);
    }

    #[test]
    fn test_dvd_url_with_empty_id() {
        let url = dvd_url("");
        let expected = format!("{}/", endpoints::DVDS);
        assert_eq!(url, expected);
    }
}

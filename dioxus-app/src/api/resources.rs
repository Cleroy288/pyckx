//! Resources API — user resources management

use super::{endpoints, helpers};
use crate::domain::course_types::ResourceData;

/// Get all user resources
pub async fn get_user_resources(
) -> Result<Vec<ResourceData>, String> {
    helpers::get_json(endpoints::RESOURCES).await
}

/// Check if a resource exists by name
pub async fn check_resource_exists(
    name: &str,
) -> Result<bool, String> {
    let url = format!(
        "{}?name={}",
        endpoints::RESOURCES_CHECK,
        name
    );
    helpers::get_json::<bool>(&url).await
}

/// Link an existing resource to a course
pub async fn link_resource(
    course_id: &str,
    resource_id: &str,
) -> Result<(), String> {
    let url = format!(
        "{}/{}/resources/link",
        endpoints::COURSES,
        course_id
    );
    #[derive(serde::Serialize)]
    struct Req {
        resource_id: String,
    }
    let req = Req {
        resource_id: resource_id.to_string(),
    };
    helpers::post_json::<Req, serde_json::Value>(
        &url, &req,
    )
    .await?;
    Ok(())
}

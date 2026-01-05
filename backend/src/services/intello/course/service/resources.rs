//! Resource Management Services

use chrono::Utc;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::services::intello::course::domain::{ResourceSummary, UserResource};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;

impl IntelloService {
    /// List course resources (via junction table)
    pub async fn list_course_resources(
        &self,
        _user_id: &str,
        course_id: &str,
    ) -> Result<Vec<UserResource>, IntelloError> {
        self.course_repo
            .get_course_resources(course_id)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))
    }

    /// List all resources for a user (summaries only, no content)
    #[instrument(skip(self))]
    pub async fn list_user_resources(
        &self,
        user_id: &str,
    ) -> Result<Vec<ResourceSummary>, IntelloError> {
        self.course_repo
            .get_user_resources(user_id)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))
    }

    /// Get full resource content with ownership check
    #[instrument(skip(self))]
    pub async fn get_resource_content(
        &self,
        user_id: &str,
        resource_id: &str,
    ) -> Result<UserResource, IntelloError> {
        let resource = self
            .course_repo
            .get_resource_by_id(resource_id)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?
            .ok_or(IntelloError::NotFound)?;

        // Ownership check
        if resource.user_id != user_id {
            return Err(IntelloError::Forbidden);
        }

        Ok(resource)
    }

    /// Check if resource filename exists for user
    #[instrument(skip(self))]
    pub async fn resource_exists(
        &self,
        user_id: &str,
        filename: &str,
    ) -> Result<bool, IntelloError> {
        self.course_repo
            .resource_exists(user_id, filename)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))
    }

    /// Create a new user resource
    #[instrument(skip(self, content), fields(filename = %filename))]
    pub async fn create_resource(
        &self,
        user_id: &str,
        filename: String,
        content: String,
        token_count: i32,
    ) -> Result<UserResource, IntelloError> {
        // Check if resource already exists
        if self.resource_exists(user_id, &filename).await? {
            return Err(IntelloError::Conflict(format!(
                "Resource '{}' already exists",
                filename
            )));
        }

        let resource = UserResource {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            filename,
            content,
            token_count,
            created_at: Utc::now().to_rfc3339(),
        };

        let created = self
            .course_repo
            .create_user_resource(&resource)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

        info!(resource_id = %created.id, "User resource created");
        Ok(created)
    }

    /// Link existing resource to course
    #[instrument(skip(self))]
    pub async fn link_resource_to_course(
        &self,
        user_id: &str,
        course_id: &str,
        resource_id: &str,
    ) -> Result<(), IntelloError> {
        // Verify course ownership
        let courses = self.list_user_courses(user_id).await?;
        if !courses.iter().any(|c| c.id == course_id) {
            return Err(IntelloError::Forbidden);
        }

        // Verify resource ownership
        let _resource = self.get_resource_content(user_id, resource_id).await?;

        // Link them
        self.course_repo
            .link_resource_to_course(course_id, resource_id)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

        info!(course_id = %course_id, resource_id = %resource_id, "Resource linked to course");
        Ok(())
    }
}

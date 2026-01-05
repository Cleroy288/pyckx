use crate::services::intello::course::domain::{Course, ResourceSummary, UserResource};
use crate::shared::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository: Send + Sync {
    /// Create a new course
    async fn create_course(&self, course: &Course) -> Result<Course, AppError>;

    /// Get all courses for a user
    async fn get_user_courses(&self, user_id: &str) -> Result<Vec<Course>, AppError>;

    /// Get all resources for a course (via junction table)
    async fn get_course_resources(&self, course_id: &str) -> Result<Vec<UserResource>, AppError>;

    /// Fetch resources by their IDs (used for context loading)
    async fn fetch_resources(&self, resource_ids: &[String])
        -> Result<Vec<UserResource>, AppError>;

    // == User Resource Management ==

    /// Get all resources belonging to a user (summary only, no content)
    async fn get_user_resources(&self, user_id: &str) -> Result<Vec<ResourceSummary>, AppError>;

    /// Get full resource by ID
    async fn get_resource_by_id(&self, resource_id: &str)
        -> Result<Option<UserResource>, AppError>;

    /// Check if resource with filename exists for user
    async fn resource_exists(&self, user_id: &str, filename: &str) -> Result<bool, AppError>;

    /// Create a user-scoped resource (not tied to course)
    async fn create_user_resource(&self, resource: &UserResource)
        -> Result<UserResource, AppError>;

    /// Link existing resource to a course
    async fn link_resource_to_course(
        &self,
        course_id: &str,
        resource_id: &str,
    ) -> Result<(), AppError>;

    // == Deletion ==

    /// Delete a course by ID
    async fn delete_course(&self, course_id: &str) -> Result<(), AppError>;

    /// Delete all resource links for a course
    async fn delete_resource_links(&self, course_id: &str) -> Result<(), AppError>;

    /// Delete a user resource by ID
    async fn delete_resource(&self, resource_id: &str) -> Result<(), AppError>;
}

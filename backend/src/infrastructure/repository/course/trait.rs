use async_trait::async_trait;
use crate::error::AppError;
use crate::domain::intello::course::{Course, Resource, ResourceSummary, UserResource};

#[async_trait]
pub trait CourseRepository: Send + Sync {
    /// Create a new course
    async fn create_course(&self, course: &Course) -> Result<Course, AppError>;

    /// Get all courses for a user
    async fn get_user_courses(&self, user_id: &str) -> Result<Vec<Course>, AppError>;

    /// Add a resource to a course
    async fn add_resource(&self, resource: &Resource) -> Result<Resource, AppError>;

    /// Get all resources for a course
    async fn get_course_resources(&self, course_id: &str) -> Result<Vec<Resource>, AppError>;

    /// Fetch resources by their IDs (used for context loading)
    async fn fetch_resources(&self, resource_ids: &[String]) -> Result<Vec<Resource>, AppError>;

    // == User Resource Management ==

    /// Get all resources belonging to a user (summary only, no content)
    async fn get_user_resources(&self, user_id: &str) -> Result<Vec<ResourceSummary>, AppError>;

    /// Get full resource by ID
    async fn get_resource_by_id(&self, resource_id: &str) -> Result<Option<UserResource>, AppError>;

    /// Check if resource with filename exists for user
    async fn resource_exists(&self, user_id: &str, filename: &str) -> Result<bool, AppError>;

    /// Create a user-scoped resource (not tied to course)
    async fn create_user_resource(&self, resource: &UserResource) -> Result<UserResource, AppError>;

    /// Link existing resource to a course
    async fn link_resource_to_course(&self, course_id: &str, resource_id: &str) -> Result<(), AppError>;
}

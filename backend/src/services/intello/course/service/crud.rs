//! Course CRUD Operations

use chrono::Utc;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::services::intello::course::domain::{Course, CreateCourseInput};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;

impl IntelloService {
    /// Create a new course
    #[instrument(skip(self), fields(name = %input.name))]
    pub async fn create_course(
        &self,
        user_id: &str,
        input: CreateCourseInput,
    ) -> Result<Course, IntelloError> {
        let course = Course {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name,
            description: input.description,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let created = self
            .course_repo
            .create_course(&course)
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))?;

        info!(course_id = %created.id, "Course created successfully");
        Ok(created)
    }

    /// List user courses
    pub async fn list_user_courses(
        &self,
        user_id: &str,
    ) -> Result<Vec<Course>, IntelloError> {
        self.course_repo
            .get_user_courses(user_id)
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))
    }

    /// Delete a course and its associated sessions and resources
    #[instrument(skip(self))]
    pub async fn delete_course(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<(), IntelloError> {
        // 1. Verify course ownership
        let courses = self.list_user_courses(user_id).await?;
        if !courses.iter().any(|c| c.id == course_id) {
            return Err(IntelloError::Forbidden);
        }

        info!(course_id = %course_id, "Deleting course and cascading data");

        // 2. Delete all study sessions
        self.study_session_repo
            .delete_by_course(course_id)
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))?;

        // 3. Get resources to delete later
        let resources = self
            .course_repo
            .get_course_resources(course_id)
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))?;

        // 4. Delete resource links
        self.course_repo
            .delete_resource_links(course_id)
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))?;

        // 5. Delete actual resources
        for resource in resources {
            self.course_repo
                .delete_resource(&resource.id)
                .await
                .map_err(|err| IntelloError::storage(err.to_string()))?;
        }

        // 6. Delete the course itself
        self.course_repo
            .delete_course(course_id)
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))?;

        info!("Course deleted successfully");
        Ok(())
    }
}

//! Course CRUD Operations

use chrono::Utc;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::services::course::domain::{Course, CreateCourseInput};
use crate::services::error_domain::StudyError;
use crate::services::StudyService;

impl StudyService {
    /// Create a new course
    #[instrument(skip(self), fields(name = %input.name))]
    pub async fn create_course(
        &self,
        user_id: &str,
        input: CreateCourseInput,
    ) -> Result<Course, StudyError> {
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
            .map_err(|err| StudyError::storage(err.to_string()))?;

        info!(course_id = %created.id, "Course created successfully");
        Ok(created)
    }

    /// List user courses
    pub async fn list_user_courses(
        &self,
        user_id: &str,
    ) -> Result<Vec<Course>, StudyError> {
        self.course_repo
            .get_user_courses(user_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))
    }

    /// Delete a course and its associated sessions and resources
    #[instrument(skip(self))]
    pub async fn delete_course(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<(), StudyError> {
        // 1. Verify course ownership
        let courses = self.list_user_courses(user_id).await?;
        if !courses.iter().any(|c| c.id == course_id) {
            return Err(StudyError::Forbidden);
        }

        info!(course_id = %course_id, "Deleting course and cascading data");

        // 2. Delete all study sessions
        self.study_session_repo
            .delete_by_course(course_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        // 3. Get resources to delete later
        let resources = self
            .course_repo
            .get_course_resources(course_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        // 4. Delete resource links
        self.course_repo
            .delete_resource_links(course_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        // 5. Delete actual resources
        for resource in resources {
            self.course_repo
                .delete_resource(&resource.id)
                .await
                .map_err(|err| StudyError::storage(err.to_string()))?;
        }

        // 6. Delete the course itself
        self.course_repo
            .delete_course(course_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        info!("Course deleted successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::test_support::{
        build_test_service, make_course, make_resource,
        make_session, MemCourseRepo, MemSessionRepo,
    };
    use std::sync::Arc;

    // ── create_course ────────────────────────────

    #[tokio::test]
    async fn test_create_course_success() {
        let svc = build_test_service(
            Arc::new(MemCourseRepo::new()),
            Arc::new(MemSessionRepo::new()),
        );
        let input = CreateCourseInput {
            name: "Math 101".into(),
            description: "Algebra basics".into(),
        };

        let result =
            svc.create_course("u1", input).await.unwrap();

        assert_eq!(result.name, "Math 101");
    }

    #[tokio::test]
    async fn test_create_course_sets_user_id() {
        let svc = build_test_service(
            Arc::new(MemCourseRepo::new()),
            Arc::new(MemSessionRepo::new()),
        );
        let input = CreateCourseInput {
            name: "Physics".into(),
            description: "Mechanics".into(),
        };

        let result =
            svc.create_course("u42", input).await.unwrap();

        assert_eq!(result.user_id, "u42");
    }

    // ── list_user_courses ────────────────────────

    #[tokio::test]
    async fn test_list_user_courses_empty() {
        let svc = build_test_service(
            Arc::new(MemCourseRepo::new()),
            Arc::new(MemSessionRepo::new()),
        );

        let result =
            svc.list_user_courses("u1").await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_list_user_courses_returns_owned() {
        let c1 = make_course("c1", "u1");
        let c2 = make_course("c2", "u2");
        let repo =
            MemCourseRepo::with_courses(vec![c1, c2]);
        let svc = build_test_service(
            Arc::new(repo),
            Arc::new(MemSessionRepo::new()),
        );

        let result =
            svc.list_user_courses("u1").await.unwrap();

        assert_eq!(result.len(), 1);
    }

    // ── delete_course ────────────────────────────

    #[tokio::test]
    async fn test_delete_course_success() {
        let c = make_course("c1", "u1");
        let repo = MemCourseRepo::with_courses(vec![c]);
        let svc = build_test_service(
            Arc::new(repo),
            Arc::new(MemSessionRepo::new()),
        );

        let result =
            svc.delete_course("u1", "c1").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_course_forbidden_wrong_user() {
        let c = make_course("c1", "owner");
        let repo = MemCourseRepo::with_courses(vec![c]);
        let svc = build_test_service(
            Arc::new(repo),
            Arc::new(MemSessionRepo::new()),
        );

        let err = svc
            .delete_course("intruder", "c1")
            .await
            .unwrap_err();

        assert!(matches!(err, StudyError::Forbidden));
    }

    #[tokio::test]
    async fn test_delete_course_cascades_sessions() {
        let c = make_course("c1", "u1");
        let s = make_session("s1", "c1");
        let course_repo =
            MemCourseRepo::with_courses(vec![c]);
        let session_repo =
            Arc::new(MemSessionRepo::with_sessions(
                vec![s],
            ));
        let svc = build_test_service(
            Arc::new(course_repo),
            session_repo.clone(),
        );

        svc.delete_course("u1", "c1").await.unwrap();

        let remaining =
            session_repo.sessions.lock().unwrap();
        assert!(remaining.is_empty());
    }

    #[tokio::test]
    async fn test_delete_course_cascades_resources() {
        let c = make_course("c1", "u1");
        let r = make_resource("r1", "u1", "notes.pdf");
        let course_repo = Arc::new(MemCourseRepo {
            courses: std::sync::Mutex::new(vec![c]),
            resources: std::sync::Mutex::new(vec![r]),
            links: std::sync::Mutex::new(vec![
                ("c1".into(), "r1".into()),
            ]),
        });
        let svc = build_test_service(
            course_repo.clone(),
            Arc::new(MemSessionRepo::new()),
        );

        svc.delete_course("u1", "c1").await.unwrap();

        let remaining =
            course_repo.resources.lock().unwrap();
        assert!(remaining.is_empty());
    }

    #[tokio::test]
    async fn test_delete_course_removes_links() {
        let c = make_course("c1", "u1");
        let r = make_resource("r1", "u1", "f.pdf");
        let course_repo = Arc::new(MemCourseRepo {
            courses: std::sync::Mutex::new(vec![c]),
            resources: std::sync::Mutex::new(vec![r]),
            links: std::sync::Mutex::new(vec![
                ("c1".into(), "r1".into()),
            ]),
        });
        let svc = build_test_service(
            course_repo.clone(),
            Arc::new(MemSessionRepo::new()),
        );

        svc.delete_course("u1", "c1").await.unwrap();

        let remaining =
            course_repo.links.lock().unwrap();
        assert!(remaining.is_empty());
    }
}

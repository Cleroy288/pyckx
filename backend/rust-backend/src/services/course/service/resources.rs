//! Resource Management Services

use chrono::Utc;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::services::course::domain::{ResourceSummary, UserResource};
use crate::services::error_domain::StudyError;
use crate::services::StudyService;

impl StudyService {
    /// List course resources (via junction table)
    pub async fn list_course_resources(
        &self,
        _user_id: &str,
        course_id: &str,
    ) -> Result<Vec<UserResource>, StudyError> {
        self.course_repo
            .get_course_resources(course_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))
    }

    /// List all resources for a user (summaries only, no content)
    #[instrument(skip(self))]
    pub async fn list_user_resources(
        &self,
        user_id: &str,
    ) -> Result<Vec<ResourceSummary>, StudyError> {
        self.course_repo
            .get_user_resources(user_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))
    }

    /// Get full resource content with ownership check
    #[instrument(skip(self))]
    pub async fn get_resource_content(
        &self,
        user_id: &str,
        resource_id: &str,
    ) -> Result<UserResource, StudyError> {
        let resource = self
            .course_repo
            .get_resource_by_id(resource_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?
            .ok_or(StudyError::NotFound)?;

        // Ownership check
        if resource.user_id != user_id {
            return Err(StudyError::Forbidden);
        }

        Ok(resource)
    }

    /// Check if resource filename exists for user
    #[instrument(skip(self))]
    pub async fn resource_exists(
        &self,
        user_id: &str,
        filename: &str,
    ) -> Result<bool, StudyError> {
        self.course_repo
            .resource_exists(user_id, filename)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))
    }

    /// Create a new user resource
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self, content), fields(filename = %filename))]
    pub async fn create_resource(
        &self,
        user_id: &str,
        filename: String,
        content: String,
        token_count: i32,
    ) -> Result<UserResource, StudyError> {
        // Check if resource already exists
        if self.resource_exists(user_id, &filename).await? {
            return Err(StudyError::Conflict(format!(
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
            .map_err(|err| StudyError::storage(err.to_string()))?;

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
    ) -> Result<(), StudyError> {
        // Verify course ownership
        let courses = self.list_user_courses(user_id).await?;
        if !courses.iter().any(|c| c.id == course_id) {
            return Err(StudyError::Forbidden);
        }

        // Verify resource ownership
        let _resource = self.get_resource_content(user_id, resource_id).await?;

        // Link them
        self.course_repo
            .link_resource_to_course(course_id, resource_id)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        info!(course_id = %course_id, resource_id = %resource_id, "Resource linked to course");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::test_support::{
        build_test_service, make_course, make_resource,
        MemCourseRepo, MemSessionRepo,
    };
    use std::sync::Arc;

    // ── Helpers ──────────────────────────────────

    /// Build service with pre-seeded resources
    fn svc_with_resources(
        resources: Vec<UserResource>,
    ) -> StudyService {
        let repo = MemCourseRepo {
            courses: std::sync::Mutex::new(vec![]),
            resources: std::sync::Mutex::new(resources),
            links: std::sync::Mutex::new(vec![]),
        };
        build_test_service(
            Arc::new(repo),
            Arc::new(MemSessionRepo::new()),
        )
    }

    // ── list_course_resources ────────────────────

    #[tokio::test]
    async fn test_list_course_resources_returns_empty() {
        let svc = svc_with_resources(vec![]);

        let result = svc
            .list_course_resources("u1", "c1")
            .await
            .unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_list_course_resources_returns_linked() {
        let r = make_resource("r1", "u1", "file.pdf");
        let repo = MemCourseRepo {
            courses: std::sync::Mutex::new(vec![]),
            resources: std::sync::Mutex::new(vec![r]),
            links: std::sync::Mutex::new(vec![
                ("c1".into(), "r1".into()),
            ]),
        };
        let svc = build_test_service(
            Arc::new(repo),
            Arc::new(MemSessionRepo::new()),
        );

        let result = svc
            .list_course_resources("u1", "c1")
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
    }

    // ── list_user_resources ──────────────────────

    #[tokio::test]
    async fn test_list_user_resources_returns_empty() {
        let svc = svc_with_resources(vec![]);

        let result =
            svc.list_user_resources("u1").await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_list_user_resources_returns_summaries() {
        let r = make_resource("r1", "u1", "notes.md");
        let svc = svc_with_resources(vec![r]);

        let result =
            svc.list_user_resources("u1").await.unwrap();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_list_user_resources_filters_by_user() {
        let r1 = make_resource("r1", "u1", "a.md");
        let r2 = make_resource("r2", "u2", "b.md");
        let svc = svc_with_resources(vec![r1, r2]);

        let result =
            svc.list_user_resources("u1").await.unwrap();

        assert_eq!(result.len(), 1);
    }

    // ── get_resource_content ─────────────────────

    #[tokio::test]
    async fn test_get_resource_content_found() {
        let r = make_resource("r1", "u1", "file.pdf");
        let svc = svc_with_resources(vec![r.clone()]);

        let result = svc
            .get_resource_content("u1", "r1")
            .await
            .unwrap();

        assert_eq!(result.id, "r1");
    }

    #[tokio::test]
    async fn test_get_resource_content_not_found() {
        let svc = svc_with_resources(vec![]);

        let err = svc
            .get_resource_content("u1", "missing")
            .await
            .unwrap_err();

        assert!(matches!(err, StudyError::NotFound));
    }

    #[tokio::test]
    async fn test_get_resource_content_wrong_owner() {
        let r = make_resource("r1", "owner", "f.pdf");
        let svc = svc_with_resources(vec![r]);

        let err = svc
            .get_resource_content("intruder", "r1")
            .await
            .unwrap_err();

        assert!(matches!(err, StudyError::Forbidden));
    }

    // ── resource_exists ──────────────────────────

    #[tokio::test]
    async fn test_resource_exists_true() {
        let r = make_resource("r1", "u1", "doc.pdf");
        let svc = svc_with_resources(vec![r]);

        let exists = svc
            .resource_exists("u1", "doc.pdf")
            .await
            .unwrap();

        assert!(exists);
    }

    #[tokio::test]
    async fn test_resource_exists_false() {
        let svc = svc_with_resources(vec![]);

        let exists = svc
            .resource_exists("u1", "nope.pdf")
            .await
            .unwrap();

        assert!(!exists);
    }

    // ── create_resource ──────────────────────────

    #[tokio::test]
    async fn test_create_resource_success() {
        let svc = svc_with_resources(vec![]);

        let result = svc
            .create_resource(
                "u1",
                "new.pdf".into(),
                "content".into(),
                50,
            )
            .await
            .unwrap();

        assert_eq!(result.filename, "new.pdf");
    }

    #[tokio::test]
    async fn test_create_resource_duplicate_conflict() {
        let r = make_resource("r1", "u1", "dup.pdf");
        let svc = svc_with_resources(vec![r]);

        let err = svc
            .create_resource(
                "u1",
                "dup.pdf".into(),
                "content".into(),
                50,
            )
            .await
            .unwrap_err();

        assert!(matches!(err, StudyError::Conflict(_)));
    }

    // ── link_resource_to_course ──────────────────

    #[tokio::test]
    async fn test_link_resource_to_course_success() {
        let c = make_course("c1", "u1");
        let r = make_resource("r1", "u1", "f.pdf");
        let repo = MemCourseRepo {
            courses: std::sync::Mutex::new(vec![c]),
            resources: std::sync::Mutex::new(vec![r]),
            links: std::sync::Mutex::new(vec![]),
        };
        let svc = build_test_service(
            Arc::new(repo),
            Arc::new(MemSessionRepo::new()),
        );

        let result = svc
            .link_resource_to_course("u1", "c1", "r1")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_link_resource_wrong_course_owner() {
        let c = make_course("c1", "owner");
        let r = make_resource("r1", "intruder", "f.pdf");
        let repo = MemCourseRepo {
            courses: std::sync::Mutex::new(vec![c]),
            resources: std::sync::Mutex::new(vec![r]),
            links: std::sync::Mutex::new(vec![]),
        };
        let svc = build_test_service(
            Arc::new(repo),
            Arc::new(MemSessionRepo::new()),
        );

        let err = svc
            .link_resource_to_course("intruder", "c1", "r1")
            .await
            .unwrap_err();

        assert!(matches!(err, StudyError::Forbidden));
    }
}

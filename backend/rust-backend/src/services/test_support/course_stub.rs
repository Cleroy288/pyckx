//! In-memory stub for CourseRepository

use crate::infra::CourseRepository;
use crate::services::course::domain::{
    Course, ResourceSummary, UserResource,
};
use crate::shared::AppError;
use async_trait::async_trait;
use std::sync::Mutex;

/// In-memory stub for CourseRepository
///
/// Stores courses and resources in Mutex-wrapped Vecs.
pub struct MemCourseRepo {
    /// Stored courses
    pub courses: Mutex<Vec<Course>>,
    /// Stored user resources
    pub resources: Mutex<Vec<UserResource>>,
    /// Course-resource links (course_id, resource_id)
    pub links: Mutex<Vec<(String, String)>>,
}

impl MemCourseRepo {
    /// Empty repo
    pub fn new() -> Self {
        Self {
            courses: Mutex::new(vec![]),
            resources: Mutex::new(vec![]),
            links: Mutex::new(vec![]),
        }
    }

    /// Pre-seed courses
    pub fn with_courses(
        courses: Vec<Course>,
    ) -> Self {
        Self {
            courses: Mutex::new(courses),
            resources: Mutex::new(vec![]),
            links: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl CourseRepository for MemCourseRepo {
    async fn create_course(
        &self,
        course: &Course,
    ) -> Result<Course, AppError> {
        self.courses.lock().unwrap().push(course.clone());
        Ok(course.clone())
    }

    async fn get_user_courses(
        &self,
        user_id: &str,
    ) -> Result<Vec<Course>, AppError> {
        let store = self.courses.lock().unwrap();
        Ok(store
            .iter()
            .filter(|c| c.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn get_course_resources(
        &self,
        course_id: &str,
    ) -> Result<Vec<UserResource>, AppError> {
        let links = self.links.lock().unwrap();
        let res = self.resources.lock().unwrap();
        let ids: Vec<String> = links
            .iter()
            .filter(|(cid, _)| cid == course_id)
            .map(|(_, rid)| rid.clone())
            .collect();
        Ok(res
            .iter()
            .filter(|r| ids.contains(&r.id))
            .cloned()
            .collect())
    }

    async fn fetch_resources(
        &self,
        ids: &[String],
    ) -> Result<Vec<UserResource>, AppError> {
        let res = self.resources.lock().unwrap();
        Ok(res
            .iter()
            .filter(|r| ids.contains(&r.id))
            .cloned()
            .collect())
    }

    async fn get_user_resources(
        &self,
        user_id: &str,
    ) -> Result<Vec<ResourceSummary>, AppError> {
        let res = self.resources.lock().unwrap();
        Ok(res
            .iter()
            .filter(|r| r.user_id == user_id)
            .map(|r| ResourceSummary {
                id: r.id.clone(),
                filename: r.filename.clone(),
                token_count: r.token_count,
                created_at: r.created_at.clone(),
            })
            .collect())
    }

    async fn get_resource_by_id(
        &self,
        resource_id: &str,
    ) -> Result<Option<UserResource>, AppError> {
        let res = self.resources.lock().unwrap();
        Ok(res
            .iter()
            .find(|r| r.id == resource_id)
            .cloned())
    }

    async fn resource_exists(
        &self,
        user_id: &str,
        filename: &str,
    ) -> Result<bool, AppError> {
        let res = self.resources.lock().unwrap();
        Ok(res.iter().any(|r| {
            r.user_id == user_id
                && r.filename == filename
        }))
    }

    async fn create_user_resource(
        &self,
        resource: &UserResource,
    ) -> Result<UserResource, AppError> {
        self.resources
            .lock()
            .unwrap()
            .push(resource.clone());
        Ok(resource.clone())
    }

    async fn link_resource_to_course(
        &self,
        course_id: &str,
        resource_id: &str,
    ) -> Result<(), AppError> {
        self.links.lock().unwrap().push((
            course_id.to_string(),
            resource_id.to_string(),
        ));
        Ok(())
    }

    async fn delete_course(
        &self,
        course_id: &str,
    ) -> Result<(), AppError> {
        self.courses
            .lock()
            .unwrap()
            .retain(|c| c.id != course_id);
        Ok(())
    }

    async fn delete_resource_links(
        &self,
        course_id: &str,
    ) -> Result<(), AppError> {
        self.links
            .lock()
            .unwrap()
            .retain(|(cid, _)| cid != course_id);
        Ok(())
    }

    async fn delete_resource(
        &self,
        resource_id: &str,
    ) -> Result<(), AppError> {
        self.resources
            .lock()
            .unwrap()
            .retain(|r| r.id != resource_id);
        Ok(())
    }
}

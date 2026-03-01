/*
 * Supabase Course Repository
 *
 * CRUD for course management with resources.
 * Uses shared SupabaseHttpClient.
 *
 * Tables: intello_courses, intello_user_resources, intello_course_resource_links
 */

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::infra::database::course::CourseRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::intello::course::domain::{
    Course, ResourceSummary, UserResource,
};
use crate::shared::AppError;

const TABLE_COURSES: &str = "intello_courses";
const TABLE_RESOURCES: &str = "intello_user_resources";
const TABLE_LINKS: &str = "intello_course_resource_links";

#[derive(Debug, Serialize, Deserialize)]
struct UserResourceRow {
    id: String,
    user_id: String,
    filename: String,
    content: String,
    token_count: i32,
    created_at: String,
}

#[derive(Debug, Serialize)]
struct InsertUserResourceRow {
    id: String,
    user_id: String,
    filename: String,
    content: String,
    token_count: i32,
}

#[derive(Debug, Serialize)]
struct LinkResourceRow {
    id: String,
    course_id: String,
    resource_id: String,
}

#[derive(Clone, Debug)]
pub struct SupabaseCourseRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseCourseRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        Self { client }
    }

    fn map_error(err: SupabaseError) -> AppError {
        AppError::Internal(crate::http_api::utils::InternalError::new(
            err.to_string(),
        ))
    }
}

#[async_trait]
impl CourseRepository for SupabaseCourseRepository {
    async fn create_course(&self, course: &Course) -> Result<Course, AppError> {
        let url = self.client.rest_url(TABLE_COURSES);
        let created: Vec<Course> = self
            .client
            .post(&url, course)
            .await
            .map_err(Self::map_error)?;
        created.into_iter().next().ok_or_else(|| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                "No course returned".to_string(),
            ))
        })
    }

    async fn get_user_courses(
        &self,
        user_id: &str,
    ) -> Result<Vec<Course>, AppError> {
        let query = format!("user_id=eq.{}&order=created_at.desc", user_id);
        let url = self.client.rest_url_with_query(TABLE_COURSES, &query);
        self.client.get(&url).await.map_err(Self::map_error)
    }

    async fn get_course_resources(
        &self,
        course_id: &str,
    ) -> Result<Vec<UserResource>, AppError> {
        let query = format!("course_id=eq.{}&select=resource_id", course_id);
        let url = self.client.rest_url_with_query(TABLE_LINKS, &query);

        #[derive(Deserialize)]
        struct LinkRow {
            resource_id: String,
        }

        let links: Vec<LinkRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        let mut resources = Vec::new();
        for link in links {
            if let Some(r) = self.get_resource_by_id(&link.resource_id).await? {
                resources.push(r);
            }
        }
        Ok(resources)
    }

    async fn fetch_resources(
        &self,
        resource_ids: &[String],
    ) -> Result<Vec<UserResource>, AppError> {
        let mut resources = Vec::new();
        for rid in resource_ids {
            if let Some(r) = self.get_resource_by_id(rid).await? {
                resources.push(r);
            }
        }
        Ok(resources)
    }

    async fn get_user_resources(
        &self,
        user_id: &str,
    ) -> Result<Vec<ResourceSummary>, AppError> {
        let query = format!(
            "user_id=eq.{}&select=id,filename,token_count,created_at&order=created_at.desc",
            user_id
        );
        let url = self.client.rest_url_with_query(TABLE_RESOURCES, &query);
        self.client.get(&url).await.map_err(Self::map_error)
    }

    async fn get_resource_by_id(
        &self,
        resource_id: &str,
    ) -> Result<Option<UserResource>, AppError> {
        let query = format!("id=eq.{}", resource_id);
        let url = self.client.rest_url_with_query(TABLE_RESOURCES, &query);
        let resources: Vec<UserResourceRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;
        Ok(resources.into_iter().next().map(|r| UserResource {
            id: r.id,
            user_id: r.user_id,
            filename: r.filename,
            content: r.content,
            token_count: r.token_count,
            created_at: r.created_at,
        }))
    }

    async fn resource_exists(
        &self,
        user_id: &str,
        filename: &str,
    ) -> Result<bool, AppError> {
        let query = format!(
            "user_id=eq.{}&filename=eq.{}&select=id",
            user_id, filename
        );
        let url = self.client.rest_url_with_query(TABLE_RESOURCES, &query);

        #[derive(Deserialize)]
        struct IdOnly {
            #[allow(dead_code)]
            id: String,
        }

        let rows: Vec<IdOnly> =
            self.client.get(&url).await.map_err(Self::map_error)?;
        Ok(!rows.is_empty())
    }

    async fn create_user_resource(
        &self,
        resource: &UserResource,
    ) -> Result<UserResource, AppError> {
        let row = InsertUserResourceRow {
            id: resource.id.to_string(),
            user_id: resource.user_id.to_string(),
            filename: resource.filename.clone(),
            content: resource.content.clone(),
            token_count: resource.token_count,
        };
        let url = self.client.rest_url(TABLE_RESOURCES);
        let created: Vec<UserResourceRow> = self
            .client
            .post(&url, &row)
            .await
            .map_err(Self::map_error)?;
        let row = created.into_iter().next().ok_or_else(|| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                "No resource returned".to_string(),
            ))
        })?;
        Ok(UserResource {
            id: row.id,
            user_id: row.user_id,
            filename: row.filename,
            content: row.content,
            token_count: row.token_count,
            created_at: row.created_at,
        })
    }

    async fn link_resource_to_course(
        &self,
        course_id: &str,
        resource_id: &str,
    ) -> Result<(), AppError> {
        let link = LinkResourceRow {
            id: uuid::Uuid::new_v4().to_string(),
            course_id: course_id.to_string(),
            resource_id: resource_id.to_string(),
        };
        let url = self.client.rest_url(TABLE_LINKS);
        self.client
            .post::<(), _>(&url, &link)
            .await
            .map_err(Self::map_error)?;
        Ok(())
    }

    async fn delete_course(&self, course_id: &str) -> Result<(), AppError> {
        let query = format!("id=eq.{}", course_id);
        let url = self.client.rest_url_with_query(TABLE_COURSES, &query);
        self.client.delete(&url).await.map_err(Self::map_error)?;
        Ok(())
    }

    async fn delete_resource_links(
        &self,
        course_id: &str,
    ) -> Result<(), AppError> {
        let query = format!("course_id=eq.{}", course_id);
        let url = self.client.rest_url_with_query(TABLE_LINKS, &query);
        self.client.delete(&url).await.map_err(Self::map_error)?;
        Ok(())
    }

    async fn delete_resource(&self, resource_id: &str) -> Result<(), AppError> {
        let query = format!("id=eq.{}", resource_id);
        let url = self.client.rest_url_with_query(TABLE_RESOURCES, &query);
        self.client.delete(&url).await.map_err(Self::map_error)?;
        Ok(())
    }
}

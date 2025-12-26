use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, debug};

use crate::config::Config;
use crate::error::AppError;
use crate::infrastructure::repository::course::CourseRepository;
use crate::domain::intello::course::{Course, Resource, ResourceSummary, UserResource};

pub struct SupabaseCourseRepository {
    url: String,
    anon_key: String,
    service_role: String,
    client: Client,
}

impl SupabaseCourseRepository {
    pub fn new(config: &Config) -> Self {
        Self {
            url: config.sp_url.clone(),
            anon_key: config.sp_anon.clone(),
            service_role: config.sp_service_role.clone(),
            client: Client::new(),
        }
    }
}

// == Row types for Supabase ==

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

#[async_trait]
impl CourseRepository for SupabaseCourseRepository {
    async fn create_course(&self, course: &Course) -> Result<Course, AppError> {
        let url = format!("{}/rest/v1/intello_courses", self.url);
        
        // Note: Course struct matches JSON expected by Supabase (snake_case)
        let response = self.client
            .post(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(course)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create course: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase create course error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let created: Vec<Course> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse created course: {}", e)))?;

        created.into_iter().next().ok_or_else(|| AppError::Internal("No course returned".to_string()))
    }

    async fn get_user_courses(&self, user_id: &str) -> Result<Vec<Course>, AppError> {
        let url = format!(
            "{}/rest/v1/intello_courses?user_id=eq.{}&order=created_at.desc",
            self.url, user_id
        );

        let response = self.client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch courses: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase get courses error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let courses: Vec<Course> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse courses: {}", e)))?;

        Ok(courses)
    }

    async fn add_resource(&self, resource: &Resource) -> Result<Resource, AppError> {
        let url = format!("{}/rest/v1/intello_course_resources", self.url);
        
        let response = self.client
            .post(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(resource)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to add resource: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase add resource error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let created: Vec<Resource> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse created resource: {}", e)))?;

        created.into_iter().next().ok_or_else(|| AppError::Internal("No resource returned".to_string()))
    }

    async fn get_course_resources(&self, course_id: &str) -> Result<Vec<Resource>, AppError> {
        let url = format!(
            "{}/rest/v1/intello_course_resources?course_id=eq.{}&order=created_at.desc",
            self.url, course_id
        );

        let response = self.client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch resources: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase get resources error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let resources: Vec<Resource> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse resources: {}", e)))?;

        Ok(resources)
    }

    async fn fetch_resources(&self, resource_ids: &[String]) -> Result<Vec<Resource>, AppError> {
        if resource_ids.is_empty() {
            return Ok(vec![]);
        }

        // Build PostgREST query: /intello_course_resources?id=in.(id1,id2,id3)
        let ids_param = format!("({})", resource_ids.join(","));
        let url = format!(
            "{}/rest/v1/intello_course_resources?id=in.{}",
            self.url, ids_param
        );

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch resources: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase fetch resources error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let resources: Vec<Resource> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse resources: {}", e)))?;

        debug!("Fetched {} resources from Supabase", resources.len());
        Ok(resources)
    }

    // == User Resource Management ==

    async fn get_user_resources(&self, user_id: &str) -> Result<Vec<ResourceSummary>, AppError> {
        let url = format!(
            "{}/rest/v1/intello_user_resources?user_id=eq.{}&select=id,filename,token_count,created_at&order=created_at.desc",
            self.url, user_id
        );

        let response = self.client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch user resources: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase get user resources error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let summaries: Vec<ResourceSummary> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse resource summaries: {}", e)))?;

        Ok(summaries)
    }

    async fn get_resource_by_id(&self, resource_id: &str) -> Result<Option<UserResource>, AppError> {
        let url = format!(
            "{}/rest/v1/intello_user_resources?id=eq.{}",
            self.url, resource_id
        );

        let response = self.client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch resource: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase get resource error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let rows: Vec<UserResourceRow> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse resource: {}", e)))?;

        Ok(rows.into_iter().next().map(|r| UserResource {
            id: r.id,
            user_id: r.user_id,
            filename: r.filename,
            content: r.content,
            token_count: r.token_count,
            created_at: r.created_at,
        }))
    }

    async fn resource_exists(&self, user_id: &str, filename: &str) -> Result<bool, AppError> {
        let url = format!(
            "{}/rest/v1/intello_user_resources?user_id=eq.{}&filename=eq.{}&select=id",
            self.url, user_id, urlencoding::encode(filename)
        );

        let response = self.client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to check resource existence: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase check resource error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let rows: Vec<serde_json::Value> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse response: {}", e)))?;

        Ok(!rows.is_empty())
    }

    async fn create_user_resource(&self, resource: &UserResource) -> Result<UserResource, AppError> {
        let url = format!("{}/rest/v1/intello_user_resources", self.url);
        
        let row = InsertUserResourceRow {
            id: resource.id.clone(),
            user_id: resource.user_id.clone(),
            filename: resource.filename.clone(),
            content: resource.content.clone(),
            token_count: resource.token_count,
        };

        let response = self.client
            .post(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&row)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create user resource: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase create user resource error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        let created: Vec<UserResourceRow> = response.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse created resource: {}", e)))?;

        let created_row = created.into_iter().next()
            .ok_or_else(|| AppError::Internal("No resource returned".to_string()))?;

        Ok(UserResource {
            id: created_row.id,
            user_id: created_row.user_id,
            filename: created_row.filename,
            content: created_row.content,
            token_count: created_row.token_count,
            created_at: created_row.created_at,
        })
    }

    async fn link_resource_to_course(&self, course_id: &str, resource_id: &str) -> Result<(), AppError> {
        let url = format!("{}/rest/v1/intello_course_resource_links", self.url);
        
        let row = LinkResourceRow {
            id: uuid::Uuid::new_v4().to_string(),
            course_id: course_id.to_string(),
            resource_id: resource_id.to_string(),
        };

        let response = self.client
            .post(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.service_role))
            .header("Content-Type", "application/json")
            .json(&row)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to link resource: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Supabase link resource error: {} - {}", status, body);
            return Err(AppError::Internal(format!("Supabase error: {}", status)));
        }

        debug!("Linked resource {} to course {}", resource_id, course_id);
        Ok(())
    }
}

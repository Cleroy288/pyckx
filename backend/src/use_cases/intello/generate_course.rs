use std::sync::Arc;
use tracing::info;

use crate::api::dto::intello::course::{GenerateCourseRequest, GenerateCourseResponse};
use crate::error::AppError;
use crate::services::IntelloService;

pub struct GenerateCourseUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateCourseUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    pub async fn execute(&self, request: GenerateCourseRequest) -> Result<GenerateCourseResponse, AppError> {
        info!(
            topic = %request.topic,
            keywords = %request.keywords.len(),
            "Executing GenerateCourseUseCase"
        );

        // 1. Fetch resources via repository if IDs provided
        let mut resource_content = request.resources.clone();
        
        if !request.resource_ids.is_empty() {
            let resources = self
                .intello_service
                .course_repo
                .fetch_resources(&request.resource_ids)
                .await?;

            let fetched_content = resources
                .iter()
                .map(|r| format!("=== {} ===\n{}", r.filename, r.content))
                .collect::<Vec<_>>()
                .join("\n\n");

            if !resource_content.is_empty() {
                resource_content.push_str("\n\n");
            }
            resource_content.push_str(&fetched_content);
        }

        // 2. Call AI Service
        let course = self
            .intello_service
            .openrouter_service
            .generate_course_unified(
                &request.topic,
                &request.keywords,
                &request.instructions,
                &resource_content,
            )
            .await
            .map_err(|e| AppError::Internal(format!("AI generation failed: {}", e)))?;

        Ok(GenerateCourseResponse {
            success: true,
            course,
        })
    }
}

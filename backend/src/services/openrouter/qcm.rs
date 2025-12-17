//! QCM generation and parsing

use super::types::OpenRouterService;
use super::utils::extract_json_from_response;
use crate::domain::intello::{CustomQuestion, QcmQuestion};
use crate::error::IntelloError;
use crate::shared::build_prompt;
use serde::Deserialize;
use tracing::{debug, info};

// == QCM Response Parser ==

#[derive(Debug, Deserialize)]
struct QcmAiResponse {
    questions: Vec<QcmAiQuestion>,
}

#[derive(Debug, Deserialize)]
struct QcmAiQuestion {
    question: String,
    wrong_answers: Vec<String>,
    right_answer: String,
    explanation: String,
}

impl OpenRouterService {
    /// Generate QCM questions from a CustomQuestion
    pub async fn generate_qcm(
        &self,
        custom_question: &CustomQuestion,
        model: Option<&str>,
    ) -> Result<Vec<QcmQuestion>, IntelloError> {
        let prompt = build_prompt(custom_question);
        
        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending QCM generation request to OpenRouter"
        );

        let response = self.send_chat_request_with_model(&prompt, model).await?;
        
        debug!(response_length = response.len(), "Received OpenRouter response");

        let questions = self.parse_qcm_response(&response)?;
        
        info!(
            questions_generated = questions.len(),
            "Successfully parsed QCM questions"
        );

        Ok(questions)
    }

    /// Parse QCM response from AI
    fn parse_qcm_response(&self, response: &str) -> Result<Vec<QcmQuestion>, IntelloError> {
        // Try to extract JSON from the response (AI might include markdown code blocks)
        let json_str = extract_json_from_response(response);
        
        let ai_response: QcmAiResponse = serde_json::from_str(&json_str)
            .map_err(|e| IntelloError::validation(
                "ai_response",
                format!("Failed to parse AI response as QCM: {}. Response was: {}", e, json_str),
            ))?;

        // Convert to domain QcmQuestion
        let questions: Vec<QcmQuestion> = ai_response
            .questions
            .into_iter()
            .map(|q| QcmQuestion {
                id: uuid::Uuid::new_v4().to_string(),
                question: q.question,
                wrong_answers: q.wrong_answers,
                right_answer: q.right_answer,
                explanation: q.explanation,
            })
            .collect();

        Ok(questions)
    }
}

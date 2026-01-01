use super::error_domain::IntelloError;
use crate::services::OpenRouterService;
use crate::services::openrouter::models_domain::DEFAULT_MODEL;
use crate::http_api::data_transfer_object::intello::course::CourseModule;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

// == INPUT TYPES ==

/// Context required by the AI to generate targeted learning content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnModeDemand {
    /// The general topic of the course
    pub topic: String,
    /// Context or summary of the course to ground the AI
    pub context: String,
    /// List of specific concepts the user struggled with
    pub failed_concepts: Vec<String>,
    /// Optional score from the previous attempt (0-100)
    pub previous_score: Option<u32>,
    /// Language for the generated content
    pub language: String,
    
    // == EXTENDED CONTEXT FROM PERSISTENCE ==
    
    /// Structured extracted knowledge (Stage 1)
    pub extracted_knowledge: Option<serde_json::Value>,
    /// Expanded AI knowledge text (Stage 0.5)
    pub expanded_knowledge: Option<String>,
}

// == PROMPT BUILDER ==

fn build_learn_mode_prompt(demand: &LearnModeDemand) -> String {
    let failed_concepts_str = if demand.failed_concepts.is_empty() {
        "General review of the topic".to_string()
    } else {
        demand.failed_concepts.join(", ")
    };

    let score_context = match demand.previous_score {
        Some(score) => format!("The student previously scored {}/100.", score),
        None => "This is the student's first focused attempt on these concepts.".to_string(),
    };

    let extended_context = if let Some(ref ek) = demand.expanded_knowledge {
        format!("\nEXTENDED KNOWLEDGE BASE:\n{}\n", ek)
    } else {
        String::new()
    };

    let extracted_context = if let Some(ref ex) = demand.extracted_knowledge {
        format!("\nSTRUCTURED CONCEPTS:\n{}\n", serde_json::to_string_pretty(ex).unwrap_or_default())
    } else {
        String::new()
    };

    format!(
        r#"You are an Expert Remedial Tutor.
Your goal is to help a student master specific concepts they struggled with in a course.

TOPIC: {}
CONTEXT: {}
FAILED CONCEPTS: {}
LANGUAGE: {}
STUDENT STATUS: {}
{}
{}

=== YOUR TASK ===
Generate a targeted "Micro-Learning Module" to fix these specific gaps.
The module must contain:
1. **EXPLANATION (Text)**: Clear, remedial explanations of the failed concepts. Use analogies and simpler terms than the original course if possible.
2. **VISUALIZATION (Schema)**: A Mermaid diagram specifically illustrating the tricky parts of these concepts.
3. **PRACTICE (Games)**: New exercises (QCM, True/False, Flashcards) to verify understanding of THESE specific points.

=== OUTPUT FORMAT (JSON) ===
Ouput a single `CourseModule` JSON object:

{{
  "title": "Remedial Session: [Key Concept Name]",
  "blocks": [
    {{
      "type": "text",
      "content": "Detailed remedial explanation..."
    }},
    {{
      "type": "schema",
      "language": "mermaid",
      "content": "graph TD..."
    }},
    {{
      "type": "qcm_set",
      "data": {{ ... }}
    }},
    {{
      "type": "flashcard_set",
      "data": {{ ... }}
    }}
  ]
}}

RULES:
- Focus ONLY on the failed concepts.
- Be encouraging but rigorous.
- Ensure JSON is valid.
- NO markdown fences (```json), just raw JSON.
"#,
        demand.topic,
        demand.context,
        failed_concepts_str,
        demand.language,
        score_context,
        extended_context,
        extracted_context
    )
}

// == SERVICE LOGIC ==

/// Generates a Learn Mode module based on the user's demand (failed concepts).
pub async fn generate_learn_content(
    service: &OpenRouterService,
    demand: &LearnModeDemand,
) -> Result<CourseModule, IntelloError> {
    info!(
        "Generating Learn Mode content for topic: '{}', failed concepts: {:?}",
        demand.topic, demand.failed_concepts
    );

    let prompt = build_learn_mode_prompt(demand);
    
    // Use the default high-intelligence model for this complex task
    let model = DEFAULT_MODEL;

    let response = service
        .send_chat_request_with_model(&prompt, Some(model))
        .await
        .map_err(|e| IntelloError::external("OpenRouter", e.to_string()))?;

    // Borrowed utility from existing code (would need to import or reimplement, here reusing logic)
    // Assuming we can parse the content directly or need cleaning (using a simple cleaner here for safety)
    let cleaned_json = clean_json_markers(&response.content);

    let module: CourseModule = serde_json::from_str(&cleaned_json).map_err(|e| {
        error!("Failed to parse Learn Mode JSON: {}", e);
        error!("Raw content: {}", response.content);
        IntelloError::validation("learn_mode_generation", format!("Invalid JSON: {}", e))
    })?;

    info!("Successfully generated Learn Mode module: '{}'", module.title);

    Ok(module)
}

/// Helper to remove markdown code blocks if the AI adds them despite instructions
fn clean_json_markers(content: &str) -> String {
    content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
        .to_string()
}

// == ORCHESTRATION ==

use crate::infra::database::StudySessionRepository;
use std::sync::Arc;

/// Orchestrates the entire Learn Mode generation process:
/// 1. Fetches the source session to get context (including extended knowledge).
/// 2. Builds a comprehensive demand.
/// 3. Generating the content via AI.
pub async fn orchestrate_learn_mode_generation(
    service: &OpenRouterService,
    repo: &Arc<dyn StudySessionRepository>,
    session_id: &str,
    failed_concepts: Vec<String>,
    previous_score: Option<u32>,
) -> Result<CourseModule, IntelloError> {
    // 1. Fetch Session
    let session = repo
        .get(session_id)
        .await
        .map_err(|e| IntelloError::validation("session_lookup", e.to_string()))?;

    // 2. Build Demand
    let demand = LearnModeDemand {
        topic: session.topic,
        context: session.instructions, // Using instructions as context, or we could use generated summary if available
        failed_concepts,
        previous_score,
        language: session.language,
        extracted_knowledge: session.extracted_knowledge,
        expanded_knowledge: session.expanded_knowledge,
    };

    // 3. Generate Content
    generate_learn_content(service, &demand).await
}

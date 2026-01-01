//! Block Protocol Course Generation with Two-Stage Pipeline
//!
//! Stage 1: Knowledge Extraction - Analyzes resources, extracts key concepts
//! Stage 2: Course Generation - Creates course using structured knowledge

use super::types_domain::OpenRouterService;
use super::utils_service::extract_json_from_response;
use crate::services::intello::error_domain::IntelloError;
use serde::de::DeserializeOwned;

use tracing::{error, info, warn};

use crate::http_api::data_transfer_object::intello::course::*;
use crate::services::intello::prompt_builder_service::{get_game_format, GAME_FORMATS};
use super::models_domain::DEFAULT_MODEL;
use super::course_generation_prompts::{self as prompts, ModulePromptInput};
// Note: join_all replaced by stream::iter().buffer_unordered() for rate limiting
use futures_util::stream::{self, StreamExt};
use std::sync::LazyLock;
use regex::Regex;

const MODEL: &str = DEFAULT_MODEL;

// == TYPE-SAFE ENUMS FOR COURSE GENERATION // ==

/// Content length for educational text generation
#[derive(Debug, Clone, Copy, Default)]
pub enum ContentLength {
    Short,
    #[default]
    Medium,
    Long,
}

impl ContentLength {
    /// Returns (section_count, word_count_per_section)
    pub fn constraints(&self) -> (&'static str, &'static str) {
        match self {
            Self::Short => ("3-5", "200-300"),
            Self::Medium => ("5-8", "400-600"),
            Self::Long => ("6-8", "800-1200"),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "short" => Self::Short,
            "long" => Self::Long,
            _ => Self::Medium,
        }
    }
}

/// Exercise depth for quiz/game generation
#[derive(Debug, Clone, Copy, Default)]
pub enum ExerciseDepth {
    Light,
    #[default]
    Medium,
    Deep,
}

impl ExerciseDepth {
    /// Returns (qcm_count, true_false_count, flashcard_count)
    pub fn exercise_counts(&self) -> (u8, u8, u8) {
        match self {
            Self::Light => (3, 3, 3),
            Self::Medium => (5, 5, 5),
            Self::Deep => (8, 8, 8),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "short" | "light" => Self::Light,
            "long" | "deep" => Self::Deep,
            _ => Self::Medium,
        }
    }
}




// == MERMAID SANITIZATION // ==

const MERMAID_DIRECTIVES: &[&str] = &[
    "graph ",
    "graph\n",
    "flowchart ",
    "flowchart\n",
    "sequencediagram",
    "mindmap",
    "classdiagram",
    "statediagram",
    "erdiagram",
    "journey",
    "gantt",
    "pie",
    "timeline",
    "gitgraph",
];

/// High-contrast theme header for dark mode compatibility
const MERMAID_THEME_HEADER: &str = "%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#1e293b', 'primaryTextColor': '#f8fafc', 'primaryBorderColor': '#64748b', 'lineColor': '#94a3b8', 'secondaryColor': '#334155', 'tertiaryColor': '#475569', 'background': '#0f172a', 'mainBkg': '#1e293b', 'nodeBorder': '#64748b', 'clusterBkg': '#1e293b', 'clusterBorder': '#475569', 'titleColor': '#f8fafc', 'edgeLabelBackground': '#1e293b' }}}%%\n";

/// Static regex for fixing unquoted labels with parentheses in Mermaid diagrams.
/// Compiled once at runtime, reused forever (performance optimization).
static MERMAID_PAREN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\[\]\x22]*\([^\[\]\x22]*\)[^\[\]\x22]*)\]")
        .expect("Invalid Mermaid parenthesis regex")
});

fn sanitize_mermaid_code(code: &str) -> String {
    // 1. Basic cleanup - convert escaped newlines
    let mut code = code.replace("\\n", "\n");

    // 2. Extract content if AI wrapped it in markdown code blocks
    if code.contains("```mermaid") {
        if let Some(start) = code.find("```mermaid") {
            code = code[start + 10..].to_string();
        }
        if let Some(end) = code.rfind("```") {
            code = code[..end].to_string();
        }
    }
    code = code.replace("```", "");

    // 3. Strip style commands that cause dark-on-dark issues
    let lines: Vec<&str> = code
        .lines()
        .filter(|line| {
            let trimmed = line.trim().to_lowercase();
            // Remove style, linkStyle, classDef lines
            !trimmed.starts_with("style ")
                && !trimmed.starts_with("linkstyle ")
                && !trimmed.starts_with("classdef ")
        })
        .collect();
    code = lines.join("\n");

    // 4. Find the actual diagram start
    let mut diagram_start = None;
    for line in code.lines() {
        let trimmed = line.trim().to_lowercase();
        for dir in MERMAID_DIRECTIVES {
            if trimmed.starts_with(dir) {
                if let Some(pos) = code.find(line) {
                    diagram_start = Some(pos);
                    break;
                }
            }
        }
        if diagram_start.is_some() {
            break;
        }
    }

    let mut cleaned_code = match diagram_start {
        Some(pos) => code[pos..].trim().to_string(),
        None => code.trim().to_string(),
    };

    // 5. Fix Unquoted Labels with Parentheses (Backend Safety Net)
    // Uses static regex - compiled once, reused forever
    let cleaned_code = MERMAID_PAREN_REGEX
        .replace_all(&cleaned_code, r#"["$1"]"#)
        .to_string();

    // 6. Inject theme header if not already present
    if !cleaned_code.contains("%%{init") && !cleaned_code.is_empty() {
        format!("{}{}", MERMAID_THEME_HEADER, cleaned_code)
    } else {
        cleaned_code
    }
}

// == COURSE GENERATION IMPLEMENTATION // ==

impl OpenRouterService {
    // == GENERIC PIPELINE HANDLER // ==
    
    /// Generic method to handle the LLM structured request lifecycle:
    /// Log → Send Request → Extract JSON → Sanitize → Deserialize
    /// 
    /// This eliminates ~20 lines of boilerplate per stage function.
    async fn execute_structured_prompt<T: DeserializeOwned>(
        &self, 
        stage_name: &str, 
        prompt: &str, 
        model: Option<&str>
    ) -> Result<(T, Option<super::types_domain::Usage>), IntelloError> {
        info!("\n=== {} ===", stage_name);
        
        // 1. Send Request
        let response = self.send_chat_request_with_model(prompt, model).await?;
        
        // 2. Extract & Sanitize JSON
        let json_str = extract_json_from_response(&response.content);
        let json_str = super::utils_service::sanitize_ai_json(&json_str);
        
        // 3. Deserialize
        let data: T = serde_json::from_str(&json_str).map_err(|e| {
            error!(error = %e, "Failed to parse JSON in {}", stage_name);
            IntelloError::validation(
                &format!("{}_parsing", stage_name.to_lowercase().replace(' ', "_")), 
                format!("Failed to parse: {}", e)
            )
        })?;

        info!("--- {} complete ---", stage_name);
        Ok((data, response.usage))
    }

    // == STAGE 0: DEMAND DECRYPTION // ==

    async fn decrypt_user_demand(
        &self,
        topic: &str,
        keywords: &[String],
        instructions: &str,
    ) -> Result<(DecryptedUserDemand, Option<super::types_domain::Usage>), IntelloError> {
        info!("\n============================================================");
        info!("STAGE 0: DEMAND DECRYPTION");
        info!("============================================================");
        info!("Topic: {}", topic);
        info!("Keywords: {:?}", keywords);
        info!(
            "Instructions: {}",
            if instructions.is_empty() {
                "(none)"
            } else {
                instructions
            }
        );

        let keywords_str = keywords.join(", ");
        let prompt = prompts::build_demand_decryption_prompt(topic, &keywords_str, instructions);

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;

        let json_str = extract_json_from_response(&response.content);

        let demand: DecryptedUserDemand = serde_json::from_str(&json_str).map_err(|e| {
            error!(error = %e, "Failed to parse Demand Decryption JSON");
            IntelloError::validation(
                "demand_decryption",
                format!("Failed to parse decrypted demand: {}", e),
            )
        })?;

        info!("\n--- Stage 0 Complete ---");
        info!("Intent: {}", demand.core_intent);
        info!("Audience: {}", demand.target_audience_profile);
        info!("Style: {}", demand.pedagogical_style);

        Ok((demand, response.usage))
    }

    // == STAGE 0.5: KNOWLEDGE EXPANSION // ==

    async fn expand_knowledge_base(
        &self,
        topic: &str,
        demand: &DecryptedUserDemand,
    ) -> Result<(String, Vec<(String, super::types_domain::Usage)>), IntelloError> {
        info!("\n============================================================");
        info!("STAGE 0.5: KNOWLEDGE EXPANSION (Gemini 3 Flash)");
        info!("============================================================");

        let prompt = prompts::build_knowledge_expansion_prompt(
            topic,
            &demand.target_audience_profile,
            &demand.core_intent,
        );

        info!("Querying expansion model: {}", MODEL);
        let response = self.send_chat_request_with_model(&prompt, Some(MODEL)).await?;
        
        let content = response.content;
        let usage = response.usage;
        
        info!("Knowledge expansion completed ({} chars)", content.len());

        let mut usages = Vec::new();
        if let Some(u) = usage {
            usages.push((MODEL.to_string(), u));
        }

        Ok((content, usages))
    }

    // == STAGE 1: KNOWLEDGE EXTRACTION // ==

    /// Analyzes raw resources and extracts a structured Knowledge Graph.
    /// This compresses and organizes information before course generation.
    async fn extract_core_knowledge(
        &self,
        topic: &str,
        demand: &DecryptedUserDemand,
        resources: &str,
    ) -> Result<(ExtractedKnowledge, Option<super::types_domain::Usage>), IntelloError> {
        info!("\n============================================================");
        info!("STAGE 1: KNOWLEDGE EXTRACTION (Gemini 3 Flash)");
        info!("============================================================");
        info!("Topic: {}", topic);
        info!(
            "Context: {} (Audience: {})",
            demand.core_intent, demand.target_audience_profile
        );

        let mandatory_topics = demand.mandatory_topics.join(", ");
        let constraints = demand.key_constraints.join(", ");
        let prompt = prompts::build_knowledge_extraction_prompt(
            topic,
            &demand.target_audience_profile,
            &demand.core_intent,
            &mandatory_topics,
            &constraints,
            resources,
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;

        let json_str = extract_json_from_response(&response.content);

        let knowledge: ExtractedKnowledge = serde_json::from_str(&json_str).map_err(|e| {
            error!(error = %e, "Failed to parse Knowledge Extraction JSON");
            IntelloError::validation(
                "knowledge_extraction",
                format!("Failed to parse extracted knowledge: {}", e),
            )
        })?;

        info!("\n--- Stage 1 Complete ---");
        info!("Extracted {} concepts", knowledge.key_concepts.len());
        for (i, concept) in knowledge.key_concepts.iter().enumerate() {
            info!(
                "  Concept {}: {} ({} key points)",
                i + 1,
                concept.name,
                concept.key_points.len()
            );
        }

        Ok((knowledge, response.usage))
    }

    /// Stage 1.5: Generate detailed educational content
    /// 
    /// Creates rich, in-depth educational text content WITHOUT exercises.
    /// This content serves as the foundation for Stage 2 to build exercises around.
    async fn generate_educational_content(
        &self,
        topic: &str,
        demand: &DecryptedUserDemand,
        knowledge_base: &ExtractedKnowledge,
        text_length: ContentLength,
    ) -> Result<(DetailedCourseContent, Option<super::types_domain::Usage>), IntelloError> {
        info!("Generating educational content ({:?}) structure...", text_length);

        let knowledge_json = serde_json::to_string_pretty(knowledge_base)
            .unwrap_or_else(|_| "{}".to_string());

        let (section_count, word_count) = text_length.constraints();

        let prompt = prompts::build_educational_content_prompt(
            topic,
            &demand.target_audience_profile,
            &demand.pedagogical_style,
            &knowledge_json,
            section_count,
            word_count,
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;

        let json_str = extract_json_from_response(&response.content);
        
        let content: DetailedCourseContent = serde_json::from_str(&json_str)
            .map_err(|e| {
                error!("Failed to parse DetailedCourseContent: {}", e);
                IntelloError::external("AI", format!("Failed to parse educational content: {}", e))
            })?;

        info!("Educational content generated:");
        info!("  Title: {}", content.title);
        info!("  Introduction: {} chars", content.introduction.len());
        info!("  Sections: {}", content.sections.len());
        for (i, section) in content.sections.iter().enumerate() {
            info!(
                "    Section {}: {} ({} chars, {} key points, {} examples)",
                i + 1,
                section.title,
                section.content.len(),
                section.key_points.len(),
                section.examples.len()
            );
        }
        info!("  Conclusion: {} chars", content.conclusion.len());

        Ok((content, response.usage))
    }

    // == STAGE 2.1: MODULE GENERATION (Per Section) // ==

    async fn generate_module(
        &self,
        topic: &str,
        section: &ContentSection,
        context_str: &str,
        exercise_depth: ExerciseDepth,
    ) -> Result<(CourseModule, Option<super::types_domain::Usage>), IntelloError> {
        info!("Generating module for section: {} (Depth: {:?})", section.title, exercise_depth);

        let qcm_fmt = get_game_format("qcm").unwrap_or(&GAME_FORMATS[0]);
        let tf_fmt = get_game_format("true_false").unwrap_or(&GAME_FORMATS[3]);
        let fc_fmt = get_game_format("flashcard").unwrap_or(&GAME_FORMATS[2]);

        let (qcm_count, tf_count, fc_count) = exercise_depth.exercise_counts();

        let prompt = prompts::build_module_prompt(&ModulePromptInput {
            topic,
            section_title: &section.title,
            section_content: &section.content,
            context_str,
            qcm_count,
            tf_count,
            fc_count,
            qcm_format_desc: qcm_fmt.format_description,
            tf_format_desc: tf_fmt.format_description,
            fc_format_desc: fc_fmt.format_description,
            qcm_schema: qcm_fmt.json_schema,
            tf_schema: tf_fmt.json_schema,
            fc_schema: fc_fmt.json_schema,
        });

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;

        let json_str = extract_json_from_response(&response.content);
        let json_str = super::utils_service::sanitize_ai_json(&json_str);
        
        // We use a temporary struct or basic parsing because CourseModule is inside GeneratedCourse usually
        // But here we want just CourseModule.
        let module: CourseModule = serde_json::from_str(&json_str).map_err(|e| {
            error!(error = %e, "Failed to parse CourseModule JSON");
             // Attempt repair if needed, but for now strict fail to keep it simple or implement retry
             IntelloError::validation("module_generation", format!("Failed to parse module: {}", e))
        })?;

        Ok((module, response.usage))
    }

    // == STAGE 2.2: SYNTHESIS GENERATION // ==

    async fn generate_synthesis(
        &self,
        topic: &str,
        educational_content_json: &str,
        context_str: &str,
    ) -> Result<(CourseModule, Option<super::types_domain::Usage>), IntelloError> {
        info!("Generating synthesis module...");

        let qcm_fmt = get_game_format("qcm").unwrap_or(&GAME_FORMATS[0]);

        let prompt = prompts::build_synthesis_prompt(
            topic,
            context_str,
            educational_content_json,
            qcm_fmt.format_description,
            qcm_fmt.json_schema,
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;

        let json_str = extract_json_from_response(&response.content);
        let json_str = super::utils_service::sanitize_ai_json(&json_str);
        
        let module: CourseModule = serde_json::from_str(&json_str).map_err(|e| {
            error!(error = %e, "Failed to parse Synthesis Module JSON");
            IntelloError::validation("synthesis_generation", format!("Failed to parse synthesis: {}", e))
        })?;

        Ok((module, response.usage))
    }

    // == JSON REPAIR (AI Recovery) // ==
    
    /// Attempts to repair malformed course JSON using AI


    // == STAGE 2: COURSE GENERATION // ==

    pub async fn generate_course_unified(
        &self,
        topic: &str,
        keywords: &[String],
        instructions: &str,
        resources: &str,
        text_length: Option<String>,
        exercise_depth: Option<String>,
    ) -> Result<(CourseGenerationResult, super::types_domain::CourseGenerationUsage), IntelloError> {
        use super::types_domain::CourseGenerationUsage;
        
        let mut usage = CourseGenerationUsage::new();
        
        // Convert string parameters to type-safe enums
        let content_length = text_length
            .as_deref()
            .map(ContentLength::from_str)
            .unwrap_or_default();
        let depth = exercise_depth
            .as_deref()
            .map(ExerciseDepth::from_str)
            .unwrap_or_default();
        
        info!("\n============================================================");
        info!("COURSE GENERATION STARTED (MODULAR PIPELINE)");
        info!("Params: Text={:?}, Exercises={:?}", content_length, depth);
        info!("============================================================");

        // == STAGE 0: Decrypt Demand ==
        let (demand, decrypt_usage) = self
            .decrypt_user_demand(topic, keywords, instructions)
            .await?;
        usage.decrypt = decrypt_usage;

        // == STAGE 0.5: Expand Knowledge (Multi-Model) ==
        let (external_knowledge_content, expand_usages) = self.expand_knowledge_base(topic, &demand).await?;
        usage.expand = expand_usages;
        
        // Merge resources: Input resources + AI Expansion
        let enhanced_resources = if !external_knowledge_content.is_empty() {
            format!(
                "{}\n\n=== SUPPLEMENTAL EXPERT KNOWLEDGE (AI ENSEMBLE) ===\n{}", 
                resources, 
                external_knowledge_content
            )
        } else {
            resources.to_string()
        };

        // == STAGE 1: Extract Knowledge ==
        let (knowledge_base, extract_usage) = self
            .extract_core_knowledge(topic, &demand, &enhanced_resources)
            .await?;
        usage.extract = extract_usage;

        // == STAGE 1.5: Generate Educational Content ==
        let (educational_content, content_usage) = self
            .generate_educational_content(topic, &demand, &knowledge_base, content_length)
            .await?;
        usage.content = content_usage;

        // Serialize content contexts for AI
        let educational_content_json = serde_json::to_string_pretty(&educational_content)
            .unwrap_or_else(|_| "Error serializing educational content".to_string());
        let structured_context = serde_json::to_string_pretty(&knowledge_base)
            .unwrap_or_else(|_| "Error serializing knowledge".to_string());

        info!("\n============================================================");
        info!("STAGE 2: MODULAR GENERATION ({} sections)", educational_content.sections.len());
        info!("============================================================");

        // == STAGE 2: Rate-Limited Parallel Module Generation ==
        // Uses buffer_unordered(5) to limit concurrent LLM requests and avoid rate limiting
        
        let results: Vec<_> = stream::iter(educational_content.sections.iter())
            .map(|section| self.generate_module(topic, section, &structured_context, depth))
            .buffer_unordered(5) // Max 5 concurrent requests
            .collect()
            .await;

        let mut modules = Vec::new();
        let mut fail_count = 0;

        for (idx, result) in results.into_iter().enumerate() {
            match result {
                Ok((mut module, _u)) => {
                    info!("Module {} generated successfully", idx + 1);
                    // Add usage if tracked (simple sum or push)
                    // We'll just log it for now to avoid complexity in usage struct
                    // sanitize schema
                    for block in &mut module.blocks {
                        if let ContentBlock::Schema { content, .. } = block {
                            *content = sanitize_mermaid_code(content.as_str());
                        }
                    }
                    modules.push(module);
                }
                Err(e) => {
                    error!("Failed to generate module {}: {}", idx + 1, e);
                    fail_count += 1;
                    // We could implement retry here or pushing a placeholder failure module
                    // For now, we skip it, which is better than failing the whole course?
                    // Or we default to just text content no games.
                }
            }
        }

        if fail_count > 0 {
            warn!("{} modules failed to generate", fail_count);
        }

        // == STAGE 3: Synthesis ==
        info!("Generating Synthesis...");
        let (mut synthesis_module, _synthesis_usage) = self.generate_synthesis(topic, &educational_content_json, &structured_context).await?;
        // Sanitize synthesis schema if any (unlikely but safe)
        for block in &mut synthesis_module.blocks {
            if let ContentBlock::Schema { content, .. } = block {
                *content = sanitize_mermaid_code(content.as_str());
            }
        }
        
        // Assemble Final Course
        let course = GeneratedCourse {
            course_metadata: CourseMetadata {
                title: educational_content.title.clone(),
                description: educational_content.introduction.clone(),
                level: "Adaptive".to_string(), // Could infer from demand
            },
            modules,
            synthesis: synthesis_module,
        };

        info!("\n============================================================");
        info!("COURSE GENERATION COMPLETE");
        info!("============================================================");
        info!("Modules Generated: {}", course.modules.len());

        let result = CourseGenerationResult {
            course,
            extracted_knowledge: knowledge_base,
            educational_content,
            expanded_knowledge: external_knowledge_content,
        };

        Ok((result, usage))
    }
}



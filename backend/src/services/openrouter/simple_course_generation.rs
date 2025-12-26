//! Block Protocol Course Generation with Two-Stage Pipeline
//!
//! Stage 1: Knowledge Extraction - Analyzes resources, extracts key concepts
//! Stage 2: Course Generation - Creates course using structured knowledge

use super::types::OpenRouterService;
use super::utils::{extract_json_from_response, sanitize_json_duplicates};
use crate::error::IntelloError;
use futures_util::future::join_all;

use std::panic::{catch_unwind, AssertUnwindSafe};
use tracing::{debug, error, info, warn};

const MODEL: &str = "google/gemini-3-flash-preview";

use crate::api::dto::intello::course::*;
use crate::shared::prompt_builder::{get_game_format, GAME_FORMATS};

// Models for Knowledge Expansion (Stage 0.5)
// NOTE: I am using standard IDs like "deepseek/deepseek-chat" because "v3.2" might not be the exact API slug.
// However, the prompt asked for SPECIFIC strings. I will respect the user's specific strings if I can, 
// but sticking to standard OpenRouter slugs is safer. 
// WAIT, the user explicitly said: 'make a call to the model : "xiaomi/mimo-v2-flash:free" and after that "mistralai/devstral-2512:free" and "deepseek/deepseek-v3.2"'
// I MUST USE THE EXACT STRINGS requested by the user.

const USER_REQUESTED_EXPANSION_MODELS: &[&str] = &[
    "xiaomi/mimo-v2-flash:free",
    "mistralai/devstral-2512:free", // As requested
    "deepseek/deepseek-v3.2",       // As requested (assuming available via OpenRouter/proxy)
];



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
    // Matches ID[Content (Parens)] and converts to ID["Content (Parens)"]
    // Using Regex to find: \[ (anything not " or ] containing ( ... )) (anything not " or ]) \]
    // Rust Regex doesn't support lookarounds easily, but we can capture groups.
    use regex::Regex;
    // We initialized regex lazy_static elsewhere or we can compile it here (perf penalty is negligible for this frequency)
    // PATTERN: `\[([^"\]]*\([^"\]]*\)[^"\]]*)\]` -> matches brackets containing parens but no quotes
    if let Ok(re) = Regex::new(r"\[([^\[\]\x22]*\([^\[\]\x22]*\)[^\[\]\x22]*)\]") {
        cleaned_code = re.replace_all(&cleaned_code, r#"["$1"]"#).to_string();
    }

    // 6. Inject theme header if not already present
    if !cleaned_code.contains("%%{init") && !cleaned_code.is_empty() {
        format!("{}{}", MERMAID_THEME_HEADER, cleaned_code)
    } else {
        cleaned_code
    }
}

fn sanitize_course_schemas(course: &mut GeneratedCourse) {
    for module in &mut course.modules {
        for block in &mut module.blocks {
            if let ContentBlock::Schema { content, .. } = block {
                *content = sanitize_mermaid_code(content);
            }
        }
    }
}

// == COURSE GENERATION IMPLEMENTATION // ==

impl OpenRouterService {
    // == STAGE 0: DEMAND DECRYPTION // ==

    async fn decrypt_user_demand(
        &self,
        topic: &str,
        keywords: &[String],
        instructions: &str,
    ) -> Result<DecryptedUserDemand, IntelloError> {
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
        let prompt = format!(
            r#"You are an Expert Pedagogical Consultant. Analyze the User's Request to determine specific educational requirements.

USER INPUT:
- TOPIC: {}
- KEYWORDS: {}
- INSTRUCTIONS: {}

Your goal is to DECRYPT the user's hidden intent.
Output ONLY valid JSON matching this structure:

{{
  "core_intent": "Why does the user want this? (e.g. 'To pass an exam', 'To build a project', 'Curiosity')",
  "target_audience_profile": "Who is this for? (e.g. 'Complete Beginner', 'Senior Engineer', 'Child')",
  "mandatory_topics": ["List of topics implied by keywords/instructions that MUST be covered"],
  "key_constraints": ["List of what NOT to do or specific limits (e.g. 'No Python', 'Under 5 mins')"],
  "pedagogical_style": "How should we teach? (e.g. 'Socratic', 'Hands-on', 'Academic', 'Humorous')"
}}

Rules:
1. Infer the audience from the complexity of keywords and tone of instructions.
2. If instructions are empty, assume a generalist/beginner audience unless keywords imply expertise.
3. Be specific in 'mandatory_topics' - expanding 1 keyword into related sub-topics if needed.
4. JSON ONLY."#,
            topic, keywords_str, instructions
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;

        let json_str = extract_json_from_response(&response);

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

        Ok(demand)
    }

    // == STAGE 0.5: KNOWLEDGE EXPANSION // ==

    async fn expand_knowledge_base(
        &self,
        topic: &str,
        demand: &DecryptedUserDemand,
    ) -> Result<String, IntelloError> {
        info!("\n============================================================");
        info!("STAGE 0.5: KNOWLEDGE EXPANSION (Multi-Model)");
        info!("============================================================");

        let prompt = format!(
            r#"You are an Expert Subject Matter Expert.
TOPIC: {}
TARGET AUDIENCE: {}
INTENT: {}

Provide a comprehensive, high-level technical overview of this topic.
Cover:
1. Key Concepts & Definitions
2. Best Practices & Patterns
3. Common Pitfalls & Anti-patterns
4. Modern ecosystem tools (if applicable)

Be concise but dense with factual information.
Do not output markdown formatting like bolding * or headers #, just plain text paragraphs."#,
            topic, demand.target_audience_profile, demand.core_intent
        );

        // Launch parallel requests
        
        // Correct approach for &self async calls:
        // We create the futures using the reference in the current scope.
        let expanded_knowledge_futures = USER_REQUESTED_EXPANSION_MODELS.iter().map(|&model| {
            let prompt = prompt.clone();
            async move {
                info!("Querying model: {}", model);
                match self.send_chat_request_with_model(&prompt, Some(model)).await {
                    Ok(response) => {
                        info!("Model {} responded ({} chars)", model, response.len());
                        Some((model, response))
                    },
                    Err(e) => {
                        warn!("Model {} failed: {}", model, e);
                        None
                    }
                }
            }
        });

        let results = join_all(expanded_knowledge_futures).await;

        let mut combined_output = String::new();
        for (model, content) in results.into_iter().flatten() {
            combined_output.push_str(&format!("\n\n--- Source: {} ---\n{}", model, content));
        }

        if combined_output.is_empty() {
             warn!("All expansion models failed! Proceeding with base resources only.");
        } else {
             info!("Knowledge expansion successful. Added {} chars of context.", combined_output.len());
        }

        Ok(combined_output)
    }

    // == STAGE 1: KNOWLEDGE EXTRACTION // ==

    /// Analyzes raw resources and extracts a structured Knowledge Graph.
    /// This compresses and organizes information before course generation.
    async fn extract_core_knowledge(
        &self,
        topic: &str,
        demand: &DecryptedUserDemand,
        resources: &str,
    ) -> Result<ExtractedKnowledge, IntelloError> {
        info!("\n============================================================");
        info!("STAGE 1: KNOWLEDGE EXTRACTION");
        info!("============================================================");
        info!("Topic: {}", topic);
        info!("\n============================================================");
        info!("STAGE 1: KNOWLEDGE EXTRACTION");
        info!("============================================================");
        info!("Topic: {}", topic);
        info!(
            "Context: {} (Audience: {})",
            demand.core_intent, demand.target_audience_profile
        );

        let mandatory_topics = demand.mandatory_topics.join(", ");
        let constraints = demand.key_constraints.join(", ");
        let prompt = format!(
            r#"You are a Senior Data Analyst. Analyze the provided resources and extract the core knowledge structure.

TOPIC: {}
AUDIENCE PROFILE: {}
INTENT: {}
MANDATORY FOCUS: {}
CONSTRAINTS: {}

RESOURCES TO ANALYZE:
{}

Your goal is to compress these resources into a structured Knowledge Base TAILORED to the audience and intent.
Output ONLY valid JSON matching this exact structure:

{{
  "topic_summary": "A concise 2-3 sentence summary of the overall topic based on resources.",
  "key_concepts": [
    {{
      "name": "Concept Name (e.g. Ownership)",
      "definition": "Precise technical definition in 1-2 sentences.",
      "key_points": [
        "Important point 1 (critical for testing)",
        "Important point 2 (good for flashcards)",
        "Important point 3 (common misconception)"
      ]
    }}
  ]
}}

Rules:
1. Extract ALL KEY CONCEPTS that match the MANDATORY FOCUS and AUDIENCE PROFILE.
2. Filter out information that violates CONSTRAINTS or is irrelevant to the INTENT.
3. Dimensions and definitions should be adapted to the AUDIENCE (e.g. simple for beginners, technical for experts).
4. Be thorough but focused.
5. Output ONLY JSON."#,
            topic,
            demand.target_audience_profile,
            demand.core_intent,
            mandatory_topics,
            constraints,
            resources
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;

        let json_str = extract_json_from_response(&response);

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

        Ok(knowledge)
    }

    // == STAGE 2: COURSE GENERATION // ==

    pub async fn generate_course_unified(
        &self,
        topic: &str,
        keywords: &[String],
        instructions: &str,
        resources: &str,
    ) -> Result<GeneratedCourse, IntelloError> {
        info!("\n============================================================");
        info!("COURSE GENERATION STARTED (3.5-STAGE PIPELINE)");
        info!("============================================================");

        // == STAGE 0: Decrypt Demand ==
        let demand = self
            .decrypt_user_demand(topic, keywords, instructions)
            .await?;

        // == STAGE 0.5: Expand Knowledge (Multi-Model) ==
        let external_knowledge = self.expand_knowledge_base(topic, &demand).await?;
        
        // Merge resources: Input resources + AI Expansion
        let enhanced_resources = if !external_knowledge.is_empty() {
            format!(
                "{}\n\n=== SUPPLEMENTAL EXPERT KNOWLEDGE (AI ENSEMBLE) ===\n{}", 
                resources, 
                external_knowledge
            )
        } else {
            resources.to_string()
        };

        // == STAGE 1: Extract Knowledge ==
        let knowledge_base = self
            .extract_core_knowledge(topic, &demand, &enhanced_resources)
            .await?;

        // Serialize knowledge to inject into Stage 2 prompt
        let structured_context = serde_json::to_string_pretty(&knowledge_base)
            .unwrap_or_else(|_| "Error serializing knowledge".to_string());

        info!("\n============================================================");
        info!("STAGE 2: COURSE GENERATION");
        info!("============================================================");
        info!(
            "Structured context size: {} chars",
            structured_context.len()
        );

        // == STAGE 2: Generate Course ==
        let qcm_fmt = get_game_format("qcm").unwrap_or(&GAME_FORMATS[0]);
        let tf_fmt = get_game_format("true_false").unwrap_or(&GAME_FORMATS[3]);
        let fc_fmt = get_game_format("flashcard").unwrap_or(&GAME_FORMATS[2]);

        let prompt = format!(
            r#"You are an expert educational architect. Generate a JSON course structure.

TOPIC: {}
INSTRUCTIONS: {}

=== SOURCE MATERIAL (STRICTLY FOLLOW THIS) ===
Use the following Structured Knowledge Base to generate content.
Do NOT hallucinate information outside this scope.

{}

=== CRITICAL: VERIFICATION TRIAD ===

Every module MUST end with these THREE verification blocks in this EXACT order:
1. `qcm_set` (as much questions as needed to test the knowledge gotten in that section) - Based on key_points from source material
2. `true_false_set` (as much questions as needed to test the knowledge gotten in that section) - Challenge misconceptions from source
3. `flashcard_set` (as much questions as needed to test the knowledge gotten in that section) - Use definitions from key_concepts

Every game should have as much questions as needed to test the knowledge gotten in that section.

=== CRITICAL: TEXTUAL CONTENT ===

Every module MUST contain substantial `text` blocks to explain concepts in depth.
- Do NOT rely solely on diagrams or exercises. You are a teacher first.
- Use clear paragraphs, analogies, and detailed explanations.
- The standard flow for a concept is:
  1. **Explain** (Text): Introduce the concept, why it matters, and how it works.
  2. **Visualize** (Schema): Show the concept in a diagram.
  3. **Verify** (Exercises): Test understanding.
- A module without detailed text explanations is a FAILURE.

=== CRITICAL: VISUAL DIAGRAMS ===

Every module MUST include 1-3 `schema` blocks to VISUALIZE concepts:
- Use Mermaid diagrams: flowchart, mindmap, graph TD, sequenceDiagram, classDiagram
- Place diagrams AFTER explaining a concept to reinforce understanding
- Diagrams should be DETAILED with multiple nodes and relationships
- Choose appropriate diagram type:
  * flowchart/graph: for processes, workflows, decision trees
  * NO mindmap: for concept hierarchies, topic overviews
  * sequenceDiagram: for interactions, request flows
  * classDiagram: for data structures, relationships

A module WITHOUT visual diagrams is INCOMPLETE.

=== MERMAID.JS SYNTAX GUIDE ===

DIAGRAM TYPES (start with header):
- flowchart LR/TD/RL/BT (Left-Right, Top-Down, etc.)
- graph TD/LR
- sequenceDiagram
- stateDiagram
- classDiagram
- NO mindmap

NODE SHAPES:
- A[Rectangle] - standard box
- B{{Decision}} - diamond shape
- C((Circle)) - circular node
- D[(Database)] - cylinder
- E>Flag] - asymmetric

CONNECTIONS:
- A --> B (solid arrow)
- A -.-> B (dashed arrow)
- A -- text --> B (labeled arrow)
- A --- B (solid line, no arrow)

EXAMPLE FLOWCHART:
graph TD
    Start[Start] --> Process[Process Data]
    Process --> Decision{{Valid?}}
    Decision -->|Yes| Success[Success]
    Decision -->|No| Error[Handle Error]
    Error --> Process

EXAMPLE MINDMAP:
mindmap
  root((Main Topic))
    Branch1
      Leaf1
      Leaf2
    Branch2
      Leaf3

COMMON ERRORS TO AVOID:
1. Reserved words: wrap 'end', 'graph', 'subgraph' in quotes if used as labels
2. Inconsistent case: node names are case-sensitive (A vs a are different)
3. Missing diagram type header: always start with flowchart/graph/sequenceDiagram
4. Special characters: escape or avoid parentheses/brackets in labels
5. Missing newlines: use \\n between lines in JSON content field
7. Unquoted labels: ID[Label(text)] -> ID["Label(text)"] - ALWAYS quote if special chars present

=== VISUAL ACCESSIBILITY STANDARDS (CRITICAL) ===

You are STRICTLY FORBIDDEN from adding visual styling to Mermaid diagrams.
The generated schemas must be PURE STRUCTURE.
We will automatically color them BLUE and WHITE.

FORBIDDEN - DO NOT USE:
- style commands (e.g., style A fill:#f9f,stroke:#333)
- linkStyle commands
- classDef commands
- fill:, stroke:, color: attributes
- Any hex color codes (#000000, #ffffff, etc.)
- NO MIND MAP SHEMA's !

=== CRITICAL: LABEL QUOTING ===
If a node label contains parentheses `()`, brackets `[]`, braces `{{}}`, or special characters, YOU MUST SURROUND THE LABEL IN DOUBLE QUOTES.
- WRONG: `A[Some(Option)]`
- CORRECT: `A["Some(Option)"]`
- CORRECT: `B["Array<T>"]`

REASONING: The frontend application handles Dark Mode theming automatically.
If you hardcode colors, you create unreadable "Dark-on-Dark" artifacts that break accessibility.
Your job is LOGIC (A --> B); the frontend handles pixel perfection.

CORRECT EXAMPLE:
graph TD
    A[Start] --> B[Process]
    B --> C{{Decision}}
    C -->|Yes| D[End]
    E["Result<T,E>"] --> F["Option(Some)"]

WRONG EXAMPLE (DO NOT DO THIS):
graph TD
    A[Start] --> B[Process]
    style A fill:#000000,stroke:#333333
    ^^^^ FORBIDDEN - This causes dark-on-dark issues
    C[Function(Args)]
    ^^^^ FORBIDDEN - Needs quotes: C["Function(Args)"]
=== IMPORTANT INFO ABOUT QCMSETS ===
{}

=== IMPORTANT INFO ABOUT FLASHCARDSETS ===
{}

=== BLOCK TYPES (7 types) ===

1. `title`: {{"type": "title", "content": "Header"}}
2. `subtitle`: {{"type": "subtitle", "content": "Subheader"}}
3. `text`: {{"type": "text", "content": "Markdown content..."}}
4. `schema`: {{"type": "schema", "language": "mermaid", "content": "graph TD\\n  A[Start]-->B[Process]\\n  B-->C{{Decision}}\\n  C-->|Yes|D[End]"}}
5. `qcm_set`: Multiple choice quiz
6. `true_false_set`: True/False statements
7. `flashcard_set`: Flashcard review

=== FORMAT: QCM SET ===
Use type: "qcm_set".
The "data" field must contain:
1. Metadata: "name", "description", "level", "subjects"
2. Content: Follow this schema:
{}

=== FORMAT: TRUE/FALSE SET ===
Use type: "true_false_set".
The "data" field must contain:
1. Metadata: "name", "description", "level", "subjects"
2. Content: Follow this schema:
{}

=== FORMAT: FLASHCARD SET ===
Use type: "flashcard_set".
The "data" field must contain:
1. Metadata: "name", "description", "level", "subjects"
2. Content: Follow this schema:
{}

=== JSON OUTPUT STRUCTURE ===
{{
  "course_metadata": {{
    "title": "Course Title",
    "description": "Based on topic_summary",
    "level": "medium"
  }},
  "modules": [
    {{
      "title": "Module 1: Introduction",
      "blocks": [
        {{"type": "title", "content": "Getting Started"}},
        {{"type": "text", "content": "Explanation from key_concepts..."}},
        {{"type": "schema", "language": "mermaid", "content": "graph TD\n  A-->B"}},
        {{"type": "qcm_set", "data": {{...}}}},
        {{"type": "true_false_set", "data": {{...}}}},
        {{"type": "flashcard_set", "data": {{...}}}}
      ]
    }}
  ]
}}

=== RULES ===
1. Generate AS MANY modules as needed to cover ALL concepts IN DEPTH - do NOT limit to 2-3 modules
2. Create one module per major concept or logical grouping
3. Each module ends with: QCM → True/False → Flashcards
4. qcm_set: AS MUCH QUESTIONS AS NEEDED
5. true_false_set: AS MUCH QUESTIONS AS NEEDED
6. flashcard_set: AS MUCH QUESTIONS AS NEEDED
7. Be COMPREHENSIVE - cover every concept from the source material
8. Output ONLY JSON - start with {{ immediately
9. NO markdown, NO trailing commas

Generate the JSON course now:"#,
            topic,
            instructions,
            structured_context,
            qcm_fmt.format_description,
            fc_fmt.format_description,
            qcm_fmt.json_schema,
            tf_fmt.json_schema,
            fc_fmt.json_schema
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;
        let json_str = extract_json_from_response(&response);

        debug!(
            raw_json_len = json_str.len(),
            "Extracted JSON from AI response"
        );

        let sanitized = catch_unwind(AssertUnwindSafe(|| sanitize_json_duplicates(&json_str)))
            .unwrap_or_else(|_| {
                warn!("Panic in sanitize_json_duplicates, using raw JSON");
                json_str.clone()
            });

        let mut course: GeneratedCourse = serde_json::from_str(&sanitized).map_err(|e| {
            let snippet = &sanitized[..sanitized.len().min(500)];
            error!(
                error = %e,
                json_snippet = %snippet,
                "Failed to parse course JSON"
            );
            IntelloError::validation("generated_course", format!("Failed to parse course: {}", e))
        })?;

        sanitize_course_schemas(&mut course);

        // == Telemetry // ==
        let mut qcm_count = 0;
        let mut tf_count = 0;
        let mut flash_count = 0;
        let mut total_questions = 0;

        for module in &course.modules {
            for block in &module.blocks {
                match block {
                    ContentBlock::QcmSet { data } => {
                        qcm_count += 1;
                        total_questions += data.questions.len();
                    }
                    ContentBlock::TrueFalseSet { data } => {
                        tf_count += 1;
                        total_questions += data.statements.len();
                    }
                    ContentBlock::FlashcardSet { data } => {
                        flash_count += 1;
                        total_questions += data.cards.len();
                    }
                    _ => {}
                }
            }
        }

        info!("\n============================================================");
        info!("COURSE GENERATION COMPLETE");
        info!("============================================================");
        info!("Course: {}", course.course_metadata.title);
        info!("Modules: {}", course.modules.len());
        for (i, module) in course.modules.iter().enumerate() {
            info!(
                "  Module {}: {} ({} blocks)",
                i + 1,
                module.title,
                module.blocks.len()
            );
        }
        info!("\nVerification Stats:");
        info!("  QCM Sets: {}", qcm_count);
        info!("  True/False Sets: {}", tf_count);
        info!("  Flashcard Sets: {}", flash_count);
        info!("  Total Questions/Cards: {}", total_questions);
        info!(
            "  Source Concepts Used: {}",
            knowledge_base.key_concepts.len()
        );
        info!("============================================================\n");

        if qcm_count == 0 || tf_count == 0 || flash_count == 0 {
            warn!("Partial generation: One or more game types missing!");
        }

        Ok(course)
    }
}



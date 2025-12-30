//! Block Protocol Course Generation with Two-Stage Pipeline
//!
//! Stage 1: Knowledge Extraction - Analyzes resources, extracts key concepts
//! Stage 2: Course Generation - Creates course using structured knowledge

use super::types_domain::OpenRouterService;
use super::utils_service::extract_json_from_response;
use crate::services::intello::error_domain::IntelloError;


use tracing::{debug, error, info, warn};

use crate::http_api::data_transfer_object::intello::course::*;
use crate::services::intello::prompt_builder_service::{get_game_format, GAME_FORMATS};
use super::models_domain::DEFAULT_MODEL;

const MODEL: &str = DEFAULT_MODEL;




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

// == STAGE 1.5 OUTPUT TYPES // ==

/// Detailed educational content for a course section
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentSection {
    pub title: String,
    pub content: String,           // 400-600 words of detailed explanation
    pub key_points: Vec<String>,   // 3-5 bullet points
    pub examples: Vec<String>,     // 2-3 real-world examples
}

/// Detailed educational content for the entire course (Stage 1.5 output)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DetailedCourseContent {
    pub title: String,
    pub introduction: String,      // 200+ words course introduction
    pub sections: Vec<ContentSection>,
    pub conclusion: String,        // 100+ words wrap-up
}

// == COURSE GENERATION IMPLEMENTATION // ==

impl OpenRouterService {
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
    ) -> Result<(DetailedCourseContent, Option<super::types_domain::Usage>), IntelloError> {
        info!("\n============================================================");
        info!("STAGE 1.5: EDUCATIONAL CONTENT GENERATION");
        info!("============================================================");

        let knowledge_json = serde_json::to_string_pretty(knowledge_base)
            .unwrap_or_else(|_| "Error serializing knowledge".to_string());

        let prompt = format!(
            r#"You are a university professor writing a comprehensive textbook chapter.

TOPIC: {}
AUDIENCE: {}
STYLE: {}

=== KNOWLEDGE BASE ===
{}

=== YOUR TASK ===

Create DETAILED educational content with:
1. An introduction (200+ words) that hooks the reader and explains why this topic matters
2. 5-8 comprehensive sections, each containing:
   - A clear title
   - 400-600 words of in-depth explanation
   - Real-world examples and analogies
   - 3-5 key bullet points summarizing the section
3. A conclusion (100+ words) that ties everything together

=== REQUIREMENTS ===

- Write as if teaching to someone who has never heard of this topic
- Use clear paragraphs with logical flow
- Include analogies to make complex concepts accessible
- Provide concrete examples for every abstract concept
- DO NOT include exercises, quizzes, or tests
- DO NOT include diagrams or code
- ONLY output educational TEXT content
- Each section MUST have 400-600 words of detailed explanation

=== OUTPUT FORMAT (JSON) ===

{{
  "title": "Course title",
  "introduction": "200+ word introduction that explains why this topic matters, what will be covered, and what the reader will learn...",
  "sections": [
    {{
      "title": "Section 1 Title",
      "content": "400-600 word detailed explanation with clear paragraphs, examples, and thorough coverage of the concept...",
      "key_points": ["Key point 1", "Key point 2", "Key point 3"],
      "examples": ["Real-world example 1", "Real-world example 2"]
    }}
  ],
  "conclusion": "100+ word conclusion that summarizes key learnings and next steps..."
}}

Output ONLY valid JSON. No markdown, no code blocks, no explanations."#,
            topic,
            demand.target_audience_profile,
            demand.pedagogical_style,
            knowledge_json
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

    // == JSON REPAIR (AI Recovery) // ==
    
    /// Attempts to repair malformed course JSON using AI
    async fn repair_course_json(
        &self,
        malformed_json: &str,
        error_message: &str,
    ) -> Result<(String, Option<super::types_domain::Usage>), IntelloError> {
        info!("Attempting AI-powered JSON repair...");
        
        let prompt = format!(
            r#"You are a JSON repair assistant. The following JSON course failed to parse.

## ERROR MESSAGE:
{error_message}

## MALFORMED JSON (may be truncated):
{malformed_json}

## YOUR TASK:
1. Identify the parsing issue from the error message
2. Fix the JSON structure to make it valid
3. Ensure all required fields are present:
   - course_metadata: {{ title, description, level }}
   - modules: array of modules with blocks
   - synthesis: {{ title, blocks }}
4. Return ONLY the fixed, complete JSON - no explanations

Common fixes:
- Missing closing brackets }} or ]]
- Trailing commas before }}
- Unescaped quotes in strings
- Invalid array elements
- Missing required fields

CRITICAL: Output ONLY valid JSON starting with {{"#
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;
        
        let repaired = extract_json_from_response(&response.content);
        info!(
            repaired_len = repaired.len(),
            "AI repair completed"
        );
        
        Ok((repaired, response.usage))
    }

    // == STAGE 2: COURSE GENERATION // ==

    pub async fn generate_course_unified(
        &self,
        topic: &str,
        keywords: &[String],
        instructions: &str,
        resources: &str,
    ) -> Result<(GeneratedCourse, super::types_domain::CourseGenerationUsage), IntelloError> {
        use super::types_domain::CourseGenerationUsage;
        
        let mut usage = CourseGenerationUsage::new();
        
        info!("\n============================================================");
        info!("COURSE GENERATION STARTED (4.5-STAGE PIPELINE)");
        info!("============================================================");

        // == STAGE 0: Decrypt Demand ==
        let (demand, decrypt_usage) = self
            .decrypt_user_demand(topic, keywords, instructions)
            .await?;
        usage.decrypt = decrypt_usage;

        // == STAGE 0.5: Expand Knowledge (Multi-Model) ==
        let (external_knowledge, expand_usages) = self.expand_knowledge_base(topic, &demand).await?;
        usage.expand = expand_usages;
        
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
        let (knowledge_base, extract_usage) = self
            .extract_core_knowledge(topic, &demand, &enhanced_resources)
            .await?;
        usage.extract = extract_usage;

        // == STAGE 1.5: Generate Educational Content ==
        let (educational_content, content_usage) = self
            .generate_educational_content(topic, &demand, &knowledge_base)
            .await?;
        usage.content = content_usage;

        // Serialize content for Stage 2
        let educational_content_json = serde_json::to_string_pretty(&educational_content)
            .unwrap_or_else(|_| "Error serializing educational content".to_string());

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
        info!(
            "Educational content size: {} chars",
            educational_content_json.len()
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

=== PRE-WRITTEN EDUCATIONAL CONTENT ===

The following detailed educational content has been written by an expert professor.
You MUST incorporate this content into your course modules.
Use the sections as the basis for your modules - each section should become a module.
The "content" field from each section should be used as the TEXT block content.
DO NOT shorten, summarize, or paraphrase - use this detailed content in full.
Your job is to:
1. Structure this content into course modules
2. ADD visual diagrams (Mermaid) to illustrate key concepts
3. ADD exercises (QCM, TrueFalse, Flashcards) that TEST this content

{}

=== CRITICAL: VERIFICATION TRIAD ===

Every module MUST end with these THREE verification blocks in this EXACT order:
1. `qcm_set` (as much questions as needed to test the knowledge gotten in that section) - Based on key_points from source material
2. `true_false_set` (as much questions as needed to test the knowledge gotten in that section) - Challenge misconceptions from source
3. `flashcard_set` (as much questions as needed to test the knowledge gotten in that section) - Use definitions from key_concepts

Every game should have as much questions as needed to test the knowledge gotten in that section.

=== CRITICAL: TEXTUAL CONTENT ===

Every module MUST contain substantial `text` blocks using the pre-written educational content.
- Use the detailed content from the PRE-WRITTEN EDUCATIONAL CONTENT section above
- Include ALL the content, examples, and explanations provided
- Add paragraph breaks and formatting for readability
- The standard flow for each module is:
  1. **Explain** (Text): Use the pre-written content for the corresponding section
  2. **Visualize** (Schema): Create a diagram to illustrate the concept
  3. **Verify** (Exercises): Test the knowledge with QCM, TrueFalse, Flashcards
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
        {{"type": "schema", "language": "mermaid", "content": "graph TD\\n  A-->B"}},
        {{"type": "qcm_set", "data": {{...}}}},
        {{"type": "true_false_set", "data": {{...}}}},
        {{"type": "flashcard_set", "data": {{...}}}}
      ]
    }}
  ],
  "synthesis": {{
    "title": "Course Synthesis",
    "blocks": [
      {{"type": "text", "content": "Comprehensive markdown summary of ALL course concepts..."}},
      {{"type": "qcm_set", "data": {{
        "name": "Final Assessment",
        "description": "Comprehensive quiz covering entire course",
        "level": "medium",
        "subjects": ["all topics"],
        "questions": [
          {{
            "question": "Question text?",
            "right_answer": "Correct answer",
            "wrong_answers": ["Wrong 1", "Wrong 2", "Wrong 3"],
            "explanation": "Why this is correct"
          }}
        ]
      }}}}
    ]
  }}
}}

=== CRITICAL: SYNTHESIS MODULE REQUIREMENTS ===
1. Synthesis is a MODULE (same structure as regular modules)
2. "title": "Course Synthesis" or similar
3. "blocks": [
     {{"type": "text", "content": "2-4 paragraph markdown summary"}},
     {{"type": "qcm_set", "data": {{...30 questions...}}}}
   ]
4. The QCM in synthesis MUST have EXACTLY 30 questions
5. Questions cover ALL modules proportionally
6. Each question MUST include "explanation" field

=== CRITICAL: JSON ARRAY VALIDATION ===
DO NOT output type hints or format strings. Output ACTUAL content values.

WRONG (will cause parsing failure):
  "wrong_answers": "[string, string]"
  "wrong_answers": "[bool, 10]"
  "key_takeaways": "[item1, item2]"

CORRECT (actual string values in arrays):
  "wrong_answers": ["Paris is the capital", "London is larger", "Madrid is older"]
  "key_takeaways": ["Rust provides memory safety", "Ownership prevents data races"]

Every array field MUST contain actual content strings, NOT type definitions.

=== RULES ===
1. Generate AS MANY modules as needed to cover ALL concepts IN DEPTH
2. Each module ends with: QCM (30 questions) → True/False (30) → Flashcards (30)
3. EVERY qcm_set must have EXACTLY 30 questions with explanations
4. EVERY question MUST include an "explanation" field
5. EVERY "wrong_answers" array MUST have 3 actual string answers
6. Be COMPREHENSIVE - cover every concept from source material
7. QCM ANSWER LENGTH: All 4 answers (1 right + 3 wrong) MUST have SIMILAR lengths - users should NOT guess by length!
8. Output ONLY JSON - start with {{ immediately
9. NO markdown, NO trailing commas, NO type hints in arrays
10. Include "synthesis" and "final_qcm" at the end

Generate the JSON course now:"#,
            topic,
            instructions,
            structured_context,
            educational_content_json,
            qcm_fmt.format_description,
            fc_fmt.format_description,
            qcm_fmt.json_schema,
            tf_fmt.json_schema,
            fc_fmt.json_schema
        );

        let response = self
            .send_chat_request_with_model(&prompt, Some(MODEL))
            .await?;
        usage.generate = response.usage;
        let json_str = extract_json_from_response(&response.content);

        debug!(
            raw_json_len = json_str.len(),
            "Extracted JSON from AI response"
        );

        // Use CourseParser for resilient parsing (with AI recovery on failure)
        let mut course = match crate::services::intello::course_parser_service::CourseParser::parse_course(&json_str) {
            Ok(c) => c,
            Err(parse_error) => {
                let error_msg = parse_error.to_string();
                warn!(
                    error = %error_msg,
                    "Initial parsing failed, attempting AI repair..."
                );
                
                // Attempt AI repair (limit JSON to avoid token overflow)
                let json_for_repair = if json_str.len() > 50_000 {
                    format!("{}... [TRUNCATED]", &json_str[..50_000])
                } else {
                    json_str.clone()
                };
                
                let (repaired_json, repair_usage) = self.repair_course_json(&json_for_repair, &error_msg).await?;
                usage.repair = repair_usage;
                
                // Second parsing attempt with repaired JSON
                crate::services::intello::course_parser_service::CourseParser::parse_course(&repaired_json)
                    .map_err(|e2| {
                        error!(
                            original_error = %error_msg,
                            repair_error = %e2,
                            "AI repair failed - course generation unsuccessful"
                        );
                        e2
                    })?
            }
        };

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

        Ok((course, usage))
    }
}



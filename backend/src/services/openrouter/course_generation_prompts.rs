//! Course Generation Prompts
//! 
//! This module contains all AI prompt builders for the course generation pipeline.
//! Separated from service logic for maintainability and testability.

/// Stage 0: Decrypt user demand to understand learning intent
pub fn build_demand_decryption_prompt(topic: &str, keywords_str: &str, instructions: &str) -> String {
    format!(
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
    )
}

/// Stage 0.5: Expand knowledge base with external research
pub fn build_knowledge_expansion_prompt(
    topic: &str, 
    target_audience: &str, 
    core_intent: &str
) -> String {
    format!(
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
        topic, target_audience, core_intent
    )
}

/// Stage 1: Extract core knowledge from resources
pub fn build_knowledge_extraction_prompt(
    topic: &str,
    audience_profile: &str,
    core_intent: &str,
    mandatory_topics: &str,
    constraints: &str,
    resources: &str,
) -> String {
    format!(
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
        topic, audience_profile, core_intent, mandatory_topics, constraints, resources
    )
}

/// Stage 1.5: Generate detailed educational content
pub fn build_educational_content_prompt(
    topic: &str,
    audience_profile: &str,
    pedagogical_style: &str,
    knowledge_json: &str,
    section_count: &str,
    word_count: &str,
) -> String {
    format!(
        r#"You are an expert educational writer.
TOPIC: {}
TARGET AUDIENCE: {}
STYLE: {}

=== KNOWLEDGE BASE ===
{}

=== YOUR TASK ===

Create DETAILED educational content with:
1. An introduction (200+ words) that hooks the reader and explains why this topic matters
2. {} comprehensive sections, each containing:
   - A clear title
   - {} words of in-depth explanation
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
- Each section MUST have {} words of detailed explanation

=== OUTPUT FORMAT (JSON) ===

{{
  "title": "Course title",
  "introduction": "200+ word introduction that explains why this topic matters, what will be covered, and what the reader will learn...",
  "sections": [
    {{
      "title": "Section 1 Title",
      "content": "{} word detailed explanation with clear paragraphs, examples, and thorough coverage of the concept...",
      "key_points": ["Key point 1", "Key point 2", "Key point 3"],
      "examples": ["Real-world example 1", "Real-world example 2"]
    }}
  ],
  "conclusion": "100+ word conclusion that summarizes key learnings and next steps..."
}}

Output ONLY valid JSON. No markdown, no code blocks, no explanations."#,
        topic, audience_profile, pedagogical_style,
        knowledge_json, section_count, word_count, word_count, word_count
    )
}

/// Module prompt input structure to avoid massive function signatures
pub struct ModulePromptInput<'a> {
    pub topic: &'a str,
    pub section_title: &'a str,
    pub section_content: &'a str,
    pub context_str: &'a str,
    pub qcm_count: u8,
    pub tf_count: u8,
    pub fc_count: u8,
    pub qcm_format_desc: &'a str,
    pub tf_format_desc: &'a str,
    pub fc_format_desc: &'a str,
    pub qcm_schema: &'a str,
    pub tf_schema: &'a str,
    pub fc_schema: &'a str,
}

/// Stage 2.1: Generate individual course modules
pub fn build_module_prompt(input: &ModulePromptInput) -> String {
    format!(
        r#"You are an expert educational architect. Create a SINGLE course module.
TOPIC: {}
MODULE TITLE: {}

=== SOURCE CONTEXT (For Reference) ===
{}

=== SECTION CONTENT (Source of Truth) ===
TITLE: {}
CONTENT:
{}

=== INSTRUCTIONS ===
1. Create a "CourseModule" JSON object for this section.
2. Use the provided CONTENT as the "text" block (DO NOT summarize, use it ALL).
3. Create 1 SCHEMA (Mermaid) to visualize the concept.
4. Create 3 GAME SETS to test this specific content:
   - QCM Set (EXACTLY {} questions)
   - True/False Set (EXACTLY {} statements)
   - Flashcard Set (EXACTLY {} cards)

=== CRITICAL: TEXTUAL CONTENT ===
Every module MUST contain substantial `text` blocks using the provided SECTION CONTENT.
- Include ALL the content, examples, and explanations provided - DO NOT TRUNCATE
- Add paragraph breaks and formatting for readability
- The text block content MUST be the COMPLETE section content, not summarized or shortened
- A module without detailed text explanations is a FAILURE.

=== CRITICAL: VISUAL DIAGRAMS (MERMAID) ===
Every module MUST include 1 `schema` block to VISUALIZE concepts:
- Use Mermaid diagrams: flowchart, graph TD, sequenceDiagram, classDiagram
- Place diagrams AFTER explaining a concept to reinforce understanding
- Diagrams should be DETAILED with multiple nodes and relationships

=== MERMAID.JS SYNTAX GUIDE ===
1. HEADER: Always start with flowchart/graph/sequenceDiagram
2. QUOTES: If a label has spaces/symbols, quote it: ID["Label Text"]
3. NO STYLES: DO NOT use `style`, `linkStyle`, `classDef`. We manage colors.
4. TYPES:
   * flowchart/graph: for processes
   * sequenceDiagram: for interactions
   * NO mindmap
   
=== GAME FORMATS ===

1. QCM SET (Multiple Choice):
{}
   - MUST have 1 right answer and 3 wrong answers.
   - All answers must be similar length.
   - "explanation" is required.

2. TRUE/FALSE SET:
{}
   - "explanation" is required.

3. FLASHCARD SET:
{}
   - Front: Concept/Question
   - Back: Definition/Answer

=== JSON OUTPUT STRUCTURE ===
{{
  "title": "{}",
  "blocks": [
    {{ "type": "text", "content": "[COMPLETE SECTION CONTENT HERE - DO NOT USE ... OR TRUNCATE - INCLUDE THE FULL TEXT]" }},
    {{ "type": "schema", "language": "mermaid", "content": "graph TD\n A-->B" }},
    {{ "type": "qcm_set", "data": {{
        "name": "QCM: {}",
        "description": "Multiple choice questions for this module",
        "level": "medium",
        "subjects": [],
        "questions": [...]
    }} }},
    {{ "type": "true_false_set", "data": {{
        "name": "True/False: {}",
        "description": "True or false statements for this module",
        "level": "medium",
        "subjects": [],
        "statements": [...]
    }} }},
    {{ "type": "flashcard_set", "data": {{
        "name": "Flashcards: {}",
        "description": "Flashcards for memorization",
        "level": "medium",
        "subjects": [],
        "cards": [...]
    }} }}
  ]
}}

=== CRITICAL: GAME DATA STRUCTURE ===
EVERY game block's "data" object MUST include these fields:
1. "name": String - Format: "GameType: Section Title"
2. "description": String - Brief description of the game
3. "level": "medium" - Always use "medium" as the level
4. "subjects": [] - Always use empty array
5. Then the content array (questions/statements/cards)

FAILURE TO INCLUDE name, description, level, subjects WILL CAUSE PARSING ERRORS.

=== JSON RULES ===
1. OUTPUT ONLY JSON.
2. NO markdown formatting.
3. Start immediately with {{.
4. ARRAYS: Use actual strings, not type hints. Example: "wrong_answers": ["A", "B", "C"]
5. ESCAPE: Properly escape quotes inside strings.

=== SCHEMAS ===

QCM SCHEMA:
{}

TRUE/FALSE SCHEMA:
{}

FLASHCARD SCHEMA:
{}"#,
        input.topic,
        input.section_title,
        input.context_str,
        input.section_title,
        input.section_content,
        input.qcm_count,
        input.tf_count,
        input.fc_count,
        input.qcm_format_desc,
        input.tf_format_desc,
        input.fc_format_desc,
        input.section_title,
        input.section_title,
        input.section_title,
        input.section_title,
        input.qcm_schema,
        input.tf_schema,
        input.fc_schema
    )
}

/// Stage 2.2: Generate synthesis module
pub fn build_synthesis_prompt(
    topic: &str,
    context_str: &str,
    educational_content_json: &str,
    qcm_format_desc: &str,
    qcm_schema: &str,
) -> String {
    format!(
        r#"You are an expert educational architect. Create the SYNTHESIS module.
TOPIC: {}

=== KNOWLEDGE BASE ===
{}

=== FULL COURSE CONTENT ===
{}

=== INSTRUCTIONS ===
1. Write a comprehensive Markdown SUMMARY (2-4 paragraphs) of the entire course.
2. Create a FINAL ASSESSMENT (QCM Set) with exactly 15 questions covering all topics.

=== GAME FORMAT: QCM SET ===
{}
   - MUST have 1 right answer and 3 wrong answers.
   - All answers must be similar length.
   - "explanation" is required.
   - QUESTIONS MUST COVER ALL COURSE MODULES PROPORTIONALLY.

=== OUTPUT FORMAT ===
{{
  "title": "Course Synthesis",
  "blocks": [
    {{ "type": "text", "content": "Comprehensive summary..." }},
    {{ "type": "qcm_set", "data": {{ 
        "name": "Final Assessment",
        "description": "Comprehensive quiz covering entire course",
        "level": "hard",
        "subjects": ["all"],
        "questions": [ ... 15 questions ... ]
     }} }}
  ]
}}

=== JSON RULES ===
1. OUTPUT ONLY JSON.
2. NO markdown formatting.
3. Start immediately with {{.
4. ARRAYS: Use actual strings, not type hints. Example: "wrong_answers": ["A", "B", "C"]
5. ESCAPE: Properly escape quotes inside strings.
6. END CLEANLY: Close with `}}` on its own line. NO trailing commas before `}}` or `]`.
7. NO TEXT AFTER JSON: Do NOT add comments, explanations, or any text after the closing brace.

=== QCM SCHEMA ===
{}"#,
        topic, context_str, educational_content_json, qcm_format_desc, qcm_schema
    )
}

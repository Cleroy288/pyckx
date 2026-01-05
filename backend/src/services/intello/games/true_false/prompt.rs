//! True/False prompt builder

use crate::services::intello::games::shared::prompt_helpers::{
    get_game_format, get_language_name, level_to_string, GAME_FORMATS,
};
use crate::services::intello::Level;

pub struct TrueOrFalsePromptInput {
    /// Name of the true/false set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated statements (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the statements
    pub subjects: Vec<String>,
    /// Number of statements to generate
    pub num_statements: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating true/false statements
// ** build_true_false_prompt **
// ==> Builds complete AI prompt for true/false statement generation
//
// @ input : TrueOrFalsePromptInput with generation parameters
// @ returns : Formatted prompt string for AI model
pub fn build_true_false_prompt(input: &TrueOrFalsePromptInput) -> String {
    let game_format = get_game_format("true_false").unwrap_or(&GAME_FORMATS[3]);

    let subjects_list = if input.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        input.subjects.join(", ")
    };

    let documents_content = input
        .documents
        .iter()
        .enumerate()
        .map(|(i, (filename, content))| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                filename,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL TRUE OR FALSE STATEMENT GENERATION

You are an expert educational content creator. Your task is to generate high-quality true or false statements that help students test their understanding of key concepts from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate statements that DIRECTLY test knowledge about {subjects}
- Every statement MUST be specifically about {subjects}
- Extract the most important facts and concepts about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}
- Create a mix of TRUE and FALSE statements (approximately 50/50)

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust statement complexity based on this level:
- Easy: Basic facts, simple definitions, fundamental concepts
- Medium: More nuanced facts, relationships between concepts, require understanding
- Hard: Complex concepts, subtle distinctions, require careful analysis

### 3. LANGUAGE
All content MUST be in {language}. This includes statements and explanations.

---

## Statement Set Details

**Name**: {name}
**Description**: {description}
**Number of Statements**: {num_statements} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract information ONLY about {subjects}:

{documents}

---

## Output Format

{game_name}
{format_description}

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Statement Generation Rules

1. **Subject Relevance**: Each statement MUST test knowledge about {subjects}
   - Focus on key facts, definitions, and concepts
   - Do NOT create generic statements unrelated to {subjects}

2. **Balance True and False**: Create roughly equal numbers of true and false statements
   - Do not make all statements one type
   - False statements should be plausible but incorrect

3. **Statement Quality**:
   - Statements should be clear and unambiguous
   - Avoid trick questions or misleading wording
   - Each statement should test ONE concept

4. **Explanations**:
   - Explain WHY the statement is true or false
   - Reference the source material when appropriate
   - Be educational and helpful

5. **Source Accuracy**: All facts must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_statements} statements are specifically about {subjects}
✓ All statements match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ There is a good mix of true and false statements
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_statements = input.num_statements,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

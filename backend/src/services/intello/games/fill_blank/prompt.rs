//! Fill Blank prompt builder

use crate::services::intello::games::shared::prompt_helpers::{
    get_game_format, get_language_name, level_to_string, GAME_FORMATS,
};
use crate::services::intello::Level;

pub struct FillBlankPromptInput {
    /// Name of the fill blank set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated content (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the questions
    pub subjects: Vec<String>,
    /// Number of questions to generate
    pub num_questions: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating fill in the blank questions
// ** build_fill_blank_prompt **
// ==> Builds complete AI prompt for fill-in-the-blank generation
//
// @ input : FillBlankPromptInput with generation parameters
// @ returns : Formatted prompt string for AI model
pub fn build_fill_blank_prompt(input: &FillBlankPromptInput) -> String {
    let game_format = get_game_format("fill_blank").unwrap_or(&GAME_FORMATS[6]);

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
        r#"# EDUCATIONAL FILL IN THE BLANK GENERATION

You are an expert educational content creator. Your task is to generate high-quality fill-in-the-blank exercises that help students test their knowledge by completing phrases with the correct answer from multiple options.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test knowledge about {subjects}
- Every phrase MUST be specifically about {subjects}
- Extract the most important concepts, facts, and terminology about {subjects}
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. Output Format: {game_name}

{format_description}

### 3. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust complexity based on this level:
- Easy: Simple vocabulary, basic concepts, obvious correct answers
- Medium: Technical terms, concepts requiring understanding, plausible distractors
- Hard: Advanced vocabulary, nuanced concepts, very plausible distractors

### 4. LANGUAGE
All content MUST be in {language}. This includes phrases, options, and explanations.

---

## Question Set Details

**Name**: {name}
**Description**: {description}
**Number of Questions**: {num_questions} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract content ONLY about {subjects}:

{documents}

---

## Output Format

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Question Generation Rules

1. **Subject Relevance**: Each phrase MUST test knowledge about {subjects}
   - Use key definitions, facts, concepts, and vocabulary
   - Do NOT create generic phrases unrelated to {subjects}

2. **Phrase Quality**:
   - Each phrase should have ONE clear blank (marked with ___)
   - The blank should be for an important term or concept
   - The phrase should be grammatically correct when completed

3. **Answer Options**:
   - Provide at least 4 options per question
   - EXACTLY ONE option must be correct (is_correct: true)
   - Wrong options should be plausible distractors from the same domain
   - Avoid obviously wrong answers

4. **Explanations**:
   - Explain WHY the correct answer is right
   - Reference the source material when appropriate
   - Be educational and helpful

5. **Source Accuracy**: All phrases and answers must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All phrases are from the source documents
✓ Each question has at least 4 options with EXACTLY ONE correct
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_questions = input.num_questions,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

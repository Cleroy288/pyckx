//! Order Phrase prompt builder

use crate::services::intello::games::shared::prompt_helpers::{
    get_game_format, get_language_name, level_to_string, GAME_FORMATS,
};
use crate::services::intello::Level;

pub struct OrderPhrasePromptInput {
    /// Name of the order phrase set
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

/// Build an AI prompt for generating order phrase questions
// ** build_order_phrase_prompt **
// ==> Builds complete AI prompt for phrase ordering game generation
//
// @ input : OrderPhrasePromptInput with generation parameters
// @ returns : Formatted prompt string for AI model
pub fn build_order_phrase_prompt(input: &OrderPhrasePromptInput) -> String {
    let game_format =
        get_game_format("order_phrase").unwrap_or(&GAME_FORMATS[5]);

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
        r#"# EDUCATIONAL ORDER PHRASE GENERATION

You are an expert educational content creator. Your task is to generate high-quality phrase ordering exercises that help students learn key concepts and improve language comprehension from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate phrases that DIRECTLY relate to {subjects}
- Every phrase MUST be specifically about {subjects}
- Extract meaningful sentences and key phrases about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. Output Format: {game_name}

{format_description}

### 3. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust phrase complexity based on this level:
- Easy: Short phrases (4-6 words), simple vocabulary, basic sentence structure
- Medium: Medium phrases (5-8 words), moderate vocabulary, standard sentences
- Hard: Longer phrases (7-10 words), complex vocabulary, sophisticated structures

### 4. LANGUAGE
All content MUST be in {language}. This includes phrases and hints.

---

## Phrase Set Details

**Name**: {name}
**Description**: {description}
**Number of Phrases**: {num_questions} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract phrases ONLY about {subjects}:

{documents}

---

## Output Format

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Phrase Generation Rules

1. **Subject Relevance**: Each phrase MUST be about {subjects}
   - Use key definitions, facts, and concepts from the source
   - Do NOT create generic phrases unrelated to {subjects}

2. **Phrase Quality**:
   - Phrases should be grammatically correct
   - Each phrase should be meaningful on its own
   - Avoid fragments or incomplete thoughts
   - Prefer impactful or memorable phrases from the source

3. **Word Splitting**:
   - Split the phrase into individual words
   - Each word gets a position (0-indexed)
   - Include punctuation attached to words (e.g., "world!" not "world" + "!")

4. **Hints**:
   - Provide a helpful hint that guides without giving away the order
   - Reference the topic or first word if helpful

5. **Source Accuracy**: All phrases must come from or be based on the provided documents

## Final Check

Before outputting, verify:
✓ All {num_questions} phrases are specifically about {subjects}
✓ All phrases match the difficulty level (word count, complexity)
✓ All content is in {language}
✓ All phrases are from the source documents
✓ Positions are correctly 0-indexed
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

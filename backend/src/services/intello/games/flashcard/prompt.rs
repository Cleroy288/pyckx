//! Flashcard prompt builder

use crate::services::intello::games::shared::prompt_helpers::{
    get_game_format, get_language_name, level_to_string, GAME_FORMATS,
};
use crate::services::intello::Level;

/// Input for flashcard prompt generation
pub struct FlashcardPromptInput {
    /// Name of the flashcard set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated flashcards (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the flashcards
    pub subjects: Vec<String>,
    /// Number of flashcards to generate
    pub num_cards: u32,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating flashcards
// ** build_flashcard_prompt **
// ==> Builds complete AI prompt for flashcard generation
//
// @ input : FlashcardPromptInput with generation parameters
// @ returns : Formatted prompt string for AI model
pub fn build_flashcard_prompt(input: &FlashcardPromptInput) -> String {
    let game_format = get_game_format("flashcard").unwrap_or(&GAME_FORMATS[2]);

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
        r#"# EDUCATIONAL FLASHCARD GENERATION

You are an expert educational content creator. Your task is to generate high-quality flashcards that help students memorize and recall key concepts from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate flashcards that DIRECTLY cover key facts, terms, and concepts about {subjects}
- Every flashcard MUST be specifically about {subjects}
- Extract the most important definitions, facts, and relationships about {subjects}
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust flashcard complexity based on this level:
- Easy: Basic definitions, simple facts, fundamental concepts
- Medium: More detailed explanations, relationships between concepts
- Hard: Complex concepts, nuanced distinctions, advanced terminology

### 3. LANGUAGE
All content MUST be in {language}. This includes both front and back of cards.

---

## Flashcard Set Details

**Name**: {name}
**Description**: {description}
**Number of Cards**: {num_cards} (generate EXACTLY this many)

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

## Flashcard Generation Rules

1. **Subject Relevance**: Each flashcard MUST cover {subjects}
   - Focus on key terms, definitions, facts, and concepts
   - Do NOT create generic flashcards unrelated to {subjects}

2. **Card Quality**:
   - Front: Clear, concise prompt (term, question, or concept)
   - Back: Accurate, complete answer or definition
   - One concept per card - don't overload

3. **Variety**: Cover different aspects of {subjects}
   - Definitions and terminology
   - Key facts and dates
   - Relationships and processes
   - Important figures or examples

4. **Source Accuracy**: All facts must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_cards} flashcards are specifically about {subjects}
✓ All cards match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_cards = input.num_cards,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

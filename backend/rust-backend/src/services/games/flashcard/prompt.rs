//! Flashcard prompt builder

use crate::services::games::shared::prompt_helpers::{
    format_documents, format_instructions,
    format_subjects, get_game_format_or_default,
    get_language_name, level_to_string, GamePromptInput,
};

/// Build an AI prompt for generating flashcards
pub fn build_flashcard_prompt(
    input: &GamePromptInput,
) -> String {
    let fmt = get_game_format_or_default("flashcard");
    let subjects = format_subjects(&input.subjects);
    let docs = format_documents(&input.documents);
    let lang = get_language_name(&input.language);
    let level = level_to_string(&input.level);
    let instr = format_instructions(&input.instructions);

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
**Number of Cards**: {num_items} (generate EXACTLY this many)

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
✓ All {num_items} flashcards are specifically about {subjects}
✓ All cards match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = fmt.game_name,
        format_description = fmt.format_description,
        subjects = subjects,
        level = level,
        language = lang,
        num_items = input.num_items,
        instructions = instr,
        documents = docs,
        json_schema = fmt.json_schema,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::Level;

    /// Build a minimal GamePromptInput for testing
    fn test_flashcard_input() -> GamePromptInput {
        GamePromptInput {
            name: "Test Cards".to_string(),
            description: "Test".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Easy,
            subjects: vec![
                "Chemistry".to_string(),
            ],
            num_items: 99,
            documents: vec![],
        }
    }

    #[test]
    fn test_build_flashcard_prompt_contains_subjects() {
        // arrange
        let input = test_flashcard_input();

        // act
        let result = build_flashcard_prompt(&input);

        // assert
        assert!(
            result.contains("Chemistry"),
            "prompt must contain the subject"
        );
    }

    #[test]
    fn test_build_flashcard_prompt_empty_subjects_uses_default()
    {
        // arrange
        let mut input = test_flashcard_input();
        input.subjects = vec![];

        // act
        let result = build_flashcard_prompt(&input);

        // assert
        assert!(
            result.contains("General topics"),
            "prompt must fall back to general topics"
        );
    }

    #[test]
    fn test_build_flashcard_prompt_contains_num_cards() {
        // arrange
        let input = test_flashcard_input();

        // act
        let result = build_flashcard_prompt(&input);

        // assert
        assert!(
            result.contains("99"),
            "prompt must contain num_items value"
        );
    }
}

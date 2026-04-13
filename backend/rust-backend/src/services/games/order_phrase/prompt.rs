//! Order Phrase prompt builder

use crate::services::games::shared::prompt_helpers::{
    format_documents, format_instructions,
    format_subjects, get_game_format_or_default,
    get_language_name, level_to_string, GamePromptInput,
};

/// Build an AI prompt for generating order phrase questions
pub fn build_order_phrase_prompt(
    input: &GamePromptInput,
) -> String {
    let fmt = get_game_format_or_default("order_phrase");
    let subjects = format_subjects(&input.subjects);
    let docs = format_documents(&input.documents);
    let lang = get_language_name(&input.language);
    let level = level_to_string(&input.level);
    let instr = format_instructions(&input.instructions);

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
**Number of Phrases**: {num_items} (generate EXACTLY this many)

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
✓ All {num_items} phrases are specifically about {subjects}
✓ All phrases match the difficulty level (word count, complexity)
✓ All content is in {language}
✓ All phrases are from the source documents
✓ Positions are correctly 0-indexed
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

    /// Factory for a default GamePromptInput
    fn test_order_phrase_input() -> GamePromptInput {
        GamePromptInput {
            name: "Test Order".to_string(),
            description: "Test".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Medium,
            subjects: vec![
                "History".to_string(),
            ],
            num_items: 5,
            documents: vec![],
        }
    }

    #[test]
    fn test_build_order_phrase_prompt_contains_subjects() {
        // arrange
        let input = test_order_phrase_input();

        // act
        let result =
            build_order_phrase_prompt(&input);

        // assert
        assert!(
            result.contains("History"),
            "Prompt must contain subject 'History'"
        );
    }

    #[test]
    fn test_build_order_phrase_prompt_empty_subjects_uses_default()
    {
        // arrange
        let mut input = test_order_phrase_input();
        input.subjects = vec![];

        // act
        let result =
            build_order_phrase_prompt(&input);

        // assert
        assert!(
            result.contains("General topics"),
            "Empty subjects should fall back to default"
        );
    }

    #[test]
    fn test_build_order_phrase_prompt_contains_doc_content() {
        // arrange
        let mut input = test_order_phrase_input();
        input.documents = vec![(
            "notes.txt".to_string(),
            "The French Revolution".to_string(),
        )];

        // act
        let result =
            build_order_phrase_prompt(&input);

        // assert
        assert!(
            result.contains("The French Revolution"),
            "Prompt must contain document content"
        );
    }
}

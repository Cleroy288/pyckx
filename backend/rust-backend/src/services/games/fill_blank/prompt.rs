//! Fill Blank prompt builder

use crate::services::games::shared::prompt_helpers::{
    format_documents, format_instructions,
    format_subjects, get_game_format_or_default,
    get_language_name, level_to_string, GamePromptInput,
};

/// Build an AI prompt for fill in the blank questions
pub fn build_fill_blank_prompt(
    input: &GamePromptInput,
) -> String {
    let fmt = get_game_format_or_default("fill_blank");
    let subjects = format_subjects(&input.subjects);
    let docs = format_documents(&input.documents);
    let lang = get_language_name(&input.language);
    let level = level_to_string(&input.level);
    let instr = format_instructions(&input.instructions);

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
**Number of Questions**: {num_items} (generate EXACTLY this many)

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
✓ All {num_items} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All phrases are from the source documents
✓ Each question has at least 4 options with EXACTLY ONE correct
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
    fn test_fill_blank_input() -> GamePromptInput {
        GamePromptInput {
            name: "Test Fill".to_string(),
            description: "Test".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Easy,
            subjects: vec!["Math".to_string()],
            num_items: 5,
            documents: vec![],
        }
    }

    #[test]
    fn test_build_fill_blank_prompt_contains_subjects() {
        // arrange
        let input = test_fill_blank_input();

        // act
        let result = build_fill_blank_prompt(&input);

        // assert
        assert!(
            result.contains("Math"),
            "Prompt must contain subject 'Math'"
        );
    }

    #[test]
    fn test_build_fill_blank_prompt_empty_subjects_uses_default()
    {
        // arrange
        let mut input = test_fill_blank_input();
        input.subjects = vec![];

        // act
        let result = build_fill_blank_prompt(&input);

        // assert
        assert!(
            result.contains("General topics"),
            "Empty subjects should fall back to default"
        );
    }

    #[test]
    fn test_build_fill_blank_prompt_contains_level() {
        // arrange
        let mut input = test_fill_blank_input();
        input.level = Level::Hard;

        // act
        let result = build_fill_blank_prompt(&input);

        // assert
        assert!(
            result.contains("HARD"),
            "Prompt must contain level 'HARD'"
        );
    }
}

//! True/False prompt builder

use crate::services::games::shared::prompt_helpers::{
    format_documents, format_instructions,
    format_subjects, get_game_format_or_default,
    get_language_name, level_to_string, GamePromptInput,
};

/// Build an AI prompt for generating true/false statements
pub fn build_true_false_prompt(
    input: &GamePromptInput,
) -> String {
    let fmt = get_game_format_or_default("true_false");
    let subjects = format_subjects(&input.subjects);
    let docs = format_documents(&input.documents);
    let lang = get_language_name(&input.language);
    let level = level_to_string(&input.level);
    let instr = format_instructions(&input.instructions);

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
**Number of Statements**: {num_items} (generate EXACTLY this many)

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
✓ All {num_items} statements are specifically about {subjects}
✓ All statements match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ There is a good mix of true and false statements
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
    fn test_true_false_input() -> GamePromptInput {
        GamePromptInput {
            name: "Test TF".to_string(),
            description: "Test".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Easy,
            subjects: vec![
                "Geography".to_string(),
            ],
            num_items: 42,
            documents: vec![],
        }
    }

    #[test]
    fn test_build_true_false_prompt_contains_subjects() {
        // arrange
        let input = test_true_false_input();

        // act
        let result = build_true_false_prompt(&input);

        // assert
        assert!(
            result.contains("Geography"),
            "Prompt must contain subject 'Geography'"
        );
    }

    #[test]
    fn test_build_true_false_prompt_empty_subjects_uses_default()
    {
        // arrange
        let mut input = test_true_false_input();
        input.subjects = vec![];

        // act
        let result = build_true_false_prompt(&input);

        // assert
        assert!(
            result.contains("General topics"),
            "Empty subjects should fall back to default"
        );
    }

    #[test]
    fn test_build_true_false_prompt_contains_num_statements() {
        // arrange
        let input = test_true_false_input();

        // act
        let result = build_true_false_prompt(&input);

        // assert
        assert!(
            result.contains("42"),
            "Prompt must contain num_items '42'"
        );
    }
}

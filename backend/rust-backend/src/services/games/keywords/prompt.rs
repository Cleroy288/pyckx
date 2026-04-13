//! Keywords prompt builder

use crate::services::games::shared::prompt_helpers::{
    format_documents, format_instructions,
    format_subjects, get_game_format_or_default,
    get_language_name, level_to_string, GamePromptInput,
};

/// Build an AI prompt for generating keywords questions
pub fn build_keywords_prompt(
    input: &GamePromptInput,
) -> String {
    let fmt = get_game_format_or_default("keywords");
    let subjects = format_subjects(&input.subjects);
    let docs = format_documents(&input.documents);
    let lang = get_language_name(&input.language);
    let level = level_to_string(&input.level);
    let instr = format_instructions(&input.instructions);

    format!(
        r#"# EDUCATIONAL KEYWORDS RECOGNITION GENERATION

You are an expert educational content creator. Your task is to generate high-quality keyword recognition exercises that help students identify key concepts and vocabulary from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test knowledge about {subjects}
- Every statement MUST be specifically about {subjects}
- Extract the most important concepts and vocabulary about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. Output Format: {game_name}

{format_description}

### 3. Key Parameters
- **Set Name**: {name}
- **Description**: {description}
- **Difficulty Level**: {level}
- **Language**: Generate ALL content in {language}
- **Number of Questions**: Generate exactly {num_items} keyword questions
- **Additional Instructions**: {instructions}

---

## SOURCE DOCUMENTS

{documents}

---

## OUTPUT FORMAT (JSON ONLY)

You MUST output VALID JSON matching this exact structure:

```json
{json_schema}
```

---

## Keyword Generation Rules

1. **Subject Relevance**: Each statement MUST test knowledge about {subjects}
   - Focus on key facts, definitions, and concepts
   - Do NOT create generic questions unrelated to {subjects}

2. **Keyword Balance**: Each question should have 6-8 keywords
   - About 60% correct (related to the statement)
   - About 40% incorrect (plausible but unrelated distractors)

3. **Keyword Quality**:
   - Correct keywords should be directly related to the statement's topic
   - Incorrect keywords should be plausible terms from the same domain but unrelated to this specific statement
   - Avoid obviously wrong keywords that are easy to eliminate

4. **Explanations**:
   - Explain WHY the correct keywords relate to the statement
   - Reference the source material when appropriate
   - Be educational and helpful

5. **Source Accuracy**: All statements and keywords must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_items} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ Each question has 6-8 keywords with a good mix of correct and incorrect
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
    fn test_keywords_input() -> GamePromptInput {
        GamePromptInput {
            name: "Test Keywords".to_string(),
            description: "Test".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Medium,
            subjects: vec![
                "Physics".to_string(),
            ],
            num_items: 5,
            documents: vec![],
        }
    }

    #[test]
    fn test_build_keywords_prompt_contains_subjects() {
        // arrange
        let input = test_keywords_input();

        // act
        let result = build_keywords_prompt(&input);

        // assert
        assert!(
            result.contains("Physics"),
            "prompt must contain the subject"
        );
    }

    #[test]
    fn test_build_keywords_prompt_empty_instructions_uses_default()
    {
        // arrange
        let input = test_keywords_input();

        // act
        let result = build_keywords_prompt(&input);

        // assert
        assert!(
            result.contains(
                "No additional instructions"
            ),
            "prompt must use default instructions"
        );
    }

    #[test]
    fn test_build_keywords_prompt_contains_language() {
        // arrange
        let mut input = test_keywords_input();
        input.language = "fr".to_string();

        // act
        let result = build_keywords_prompt(&input);

        // assert
        assert!(
            result.contains("French"),
            "prompt must contain language name"
        );
    }
}

//! Open Question prompt builder

use crate::services::games::shared::prompt_helpers::{
    format_documents, format_instructions,
    format_subjects, get_game_format_or_default,
    get_language_name, level_to_string, GamePromptInput,
};

/// Build an AI prompt for generating open-ended questions
pub fn build_open_question_prompt(
    input: &GamePromptInput,
) -> String {
    let fmt = get_game_format_or_default("open_question");
    let subjects = format_subjects(&input.subjects);
    let docs = format_documents(&input.documents);
    let lang = get_language_name(&input.language);
    let level = level_to_string(&input.level);
    let instr = format_instructions(&input.instructions);

    format!(
        r#"# EDUCATIONAL OPEN-ENDED QUESTION GENERATION

You are an expert educational content creator. Your task is to generate high-quality open-ended questions that help students deeply understand specific subjects from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test deep understanding of {subjects}
- Every question MUST be specifically about {subjects} - no generic or off-topic questions
- Extract the most important concepts, theories, and details about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}
- Questions should require the student to demonstrate real understanding of {subjects}

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

You MUST strictly follow these difficulty guidelines:
- Question complexity must match this level
- Expected answer depth must match this level
- Vocabulary and terminology must match this level

### 3. LANGUAGE
All content MUST be in {language}. This includes questions, expected answers, and hints.

---

## Question Set Details

**Name**: {name}
**Description**: {description}
**Number of Questions**: {num_items} (generate EXACTLY this many)

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

## Question Generation Rules

1. **Subject Relevance**: Each question MUST directly test understanding of {subjects}
   - Ask about key concepts, theories, processes, and relationships within {subjects}
   - Questions should require explaining, analyzing, or applying knowledge of {subjects}
   - Do NOT create generic questions that could apply to any topic
   - Do NOT create questions about topics outside {subjects}

2. **Question Types for Open-Ended Format**:
   - "Explain..." - Ask students to explain concepts in their own words
   - "Compare and contrast..." - Ask about similarities and differences
   - "Why does..." - Ask about causes and reasoning
   - "How would you..." - Ask about application of knowledge
   - "What is the significance of..." - Ask about importance and implications
   - "Describe the process of..." - Ask about procedures and sequences

3. **Difficulty Consistency**: Every question must match the difficulty level
   - Easy: Simple explanations, basic definitions, straightforward facts
   - Medium: Connections between concepts, cause-effect relationships, applications
   - Hard: Analysis, synthesis, evaluation, complex reasoning

4. **Expected Answers**:
   - Must be accurate based on the source documents
   - Length and depth should match the difficulty level
   - Should demonstrate the understanding you're testing for

5. **Hints**:
   - Should guide thinking without giving away the answer
   - Reference relevant concepts or sections from the source
   - Help students recall what they learned about {subjects}

6. **Coverage**: Distribute questions across different aspects of {subjects}
   - Cover various concepts, not just one narrow topic
   - Ensure comprehensive testing of the subject matter

## Final Check

Before outputting, verify:
✓ All {num_items} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ Expected answers are accurate and appropriately detailed
✓ Hints are helpful but don't reveal answers
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
    fn test_open_question_input() -> GamePromptInput {
        GamePromptInput {
            name: "Test OQ".to_string(),
            description: "Test".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Hard,
            subjects: vec![
                "Literature".to_string(),
            ],
            num_items: 5,
            documents: vec![],
        }
    }

    #[test]
    fn test_build_open_question_prompt_contains_subjects() {
        // arrange
        let input = test_open_question_input();

        // act
        let result =
            build_open_question_prompt(&input);

        // assert
        assert!(
            result.contains("Literature"),
            "Prompt must contain subject 'Literature'"
        );
    }

    #[test]
    fn test_build_open_question_prompt_empty_subjects_uses_default()
    {
        // arrange
        let mut input = test_open_question_input();
        input.subjects = vec![];

        // act
        let result =
            build_open_question_prompt(&input);

        // assert
        assert!(
            result.contains("General topics"),
            "Empty subjects should fall back to default"
        );
    }

    #[test]
    fn test_build_open_question_prompt_contains_language() {
        // arrange
        let mut input = test_open_question_input();
        input.language = "es".to_string();

        // act
        let result =
            build_open_question_prompt(&input);

        // assert
        assert!(
            result.contains("Spanish"),
            "Prompt must contain language name 'Spanish'"
        );
    }
}

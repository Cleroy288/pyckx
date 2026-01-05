//! Keywords prompt builder

use crate::services::intello::games::shared::prompt_helpers::{
    get_game_format, get_language_name, level_to_string, GAME_FORMATS,
};
use crate::services::intello::Level;

pub struct KeywordsPromptInput {
    /// Name of the keywords set
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

/// Build an AI prompt for generating keywords questions
// ** build_keywords_prompt **
// ==> Builds complete AI prompt for keywords game generation
//
// @ input : KeywordsPromptInput with generation parameters
// @ returns : Formatted prompt string for AI model
pub fn build_keywords_prompt(input: &KeywordsPromptInput) -> String {
    let game_format = get_game_format("keywords").unwrap_or(&GAME_FORMATS[4]);

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
- **Number of Questions**: Generate exactly {num_questions} keyword questions
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
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ Each question has 6-8 keywords with a good mix of correct and incorrect
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

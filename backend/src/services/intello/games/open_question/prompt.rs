//! Open Question prompt builder

use crate::services::intello::games::shared::prompt_helpers::{
    get_game_format, get_language_name, level_to_string, GAME_FORMATS,
};
use crate::services::intello::Level;

pub struct OpenQuestionPromptInput {
    /// Name of the open question set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated questions (e.g., "en", "fr")
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

/// Build an AI prompt for generating open-ended questions
// ** build_open_question_prompt **
// ==> Builds complete AI prompt for open question generation
//
// @ input : OpenQuestionPromptInput with generation parameters
// @ returns : Formatted prompt string for AI model
pub fn build_open_question_prompt(input: &OpenQuestionPromptInput) -> String {
    let game_format =
        get_game_format("open_question").unwrap_or(&GAME_FORMATS[1]);

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
**Number of Questions**: {num_questions} (generate EXACTLY this many)

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
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ Expected answers are accurate and appropriately detailed
✓ Hints are helpful but don't reveal answers
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

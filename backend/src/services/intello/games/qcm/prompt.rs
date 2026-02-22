//! QCM prompt builder

use crate::services::intello::custom_question_domain::CustomQuestion;
use crate::services::intello::games::shared::prompt_helpers::{
    get_game_format, get_language_name, level_to_string, GAME_FORMATS,
};

// ** build_qcm_prompt **
// ==> Builds complete AI prompt for QCM question generation
//
// @ custom_question : CustomQuestion with generation parameters
// @ returns : Formatted prompt string for AI model
pub fn build_qcm_prompt(custom_question: &CustomQuestion) -> String {
    let game_format = get_game_format(&custom_question.output_game)
        .unwrap_or(&GAME_FORMATS[0]); // Default to QCM if not found

    let subjects_list = if custom_question.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        custom_question.subjects.join(", ")
    };

    // Build the documents content section
    let documents_content = custom_question
        .documents
        .iter()
        .enumerate()
        .map(|(i, doc)| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                doc.filename,
                doc.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL QUIZ GENERATION

You are an expert educational content creator. Your task is to generate high-quality quiz questions that help students learn specific subjects from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test knowledge about {subjects}
- Every question MUST be specifically about {subjects} - no generic or off-topic questions
- Extract the most important concepts, facts, and details about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

You MUST strictly follow these difficulty guidelines. Every question must match this level.

### 3. LANGUAGE
All content MUST be in {language}. This includes questions, answers, and explanations.

---

## Quiz Details

**Name**: {name}
**Description**: {description}
**Number of Questions**: {num_questions}

⚠️ STRICT CONSTRAINT: You MUST generate EXACTLY {num_questions} questions. Not more, not fewer. If you generate any other number the output will be rejected.

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

1. **Subject Relevance**: Each question MUST directly test knowledge about {subjects}
   - Ask about key concepts, definitions, facts, and relationships within {subjects}
   - Do NOT create generic questions that could apply to any topic
   - Do NOT create questions about topics outside {subjects}

2. **Difficulty Consistency**: Every question must match the {level} level
   - Vocabulary, complexity, and reasoning must all align with this level
   - Do not mix easy and hard questions

3. **Source Accuracy**: All facts must come from the provided documents
   - Do not invent information not present in the source material
   - Quote or paraphrase directly from the documents

4. **Answer Quality**:
   - Correct answer must be unambiguously right based on the source
   - Wrong answers must be plausible but clearly incorrect
   - Explanations should teach why the answer is correct

5. **Coverage**: Distribute questions across different aspects of {subjects}
   - Cover various concepts, not just one narrow topic
   - Ensure comprehensive testing of the subject matter

## Final Check

Before outputting, verify:
✓ You have EXACTLY {num_questions} questions — count them
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the {level} difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = custom_question.name,
        description = custom_question.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&custom_question.level),
        language = get_language_name(&custom_question.language),
        num_questions = custom_question.num_questions,
        instructions = if custom_question.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            custom_question.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

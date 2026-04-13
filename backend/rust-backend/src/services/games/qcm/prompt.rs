//! QCM prompt builder

use crate::services::custom_question_domain::CustomQuestion;
use crate::services::games::shared::prompt_helpers::{
    format_documents, format_instructions,
    format_subjects, get_game_format_or_default,
    get_language_name, level_to_string,
};

/// Build an AI prompt for QCM question generation
pub fn build_qcm_prompt(
    cq: &CustomQuestion,
) -> String {
    let fmt = get_game_format_or_default(&cq.output_game);
    let subjects = format_subjects(&cq.subjects);
    let level = level_to_string(&cq.level);
    let lang = get_language_name(&cq.language);
    let instr = format_instructions(&cq.instructions);
    let docs = format_qcm_documents(cq);

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
        name = cq.name,
        description = cq.description,
        game_name = fmt.game_name,
        format_description = fmt.format_description,
        subjects = subjects,
        level = level,
        language = lang,
        num_questions = cq.num_questions,
        instructions = instr,
        documents = docs,
        json_schema = fmt.json_schema,
    )
}

/// Format CustomQuestion documents into markdown
fn format_qcm_documents(cq: &CustomQuestion) -> String {
    let pairs: Vec<(String, String)> = cq
        .documents
        .iter()
        .map(|d| {
            (d.filename.clone(), d.content.clone())
        })
        .collect();
    format_documents(&pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::custom_question_domain::{
        CustomQuestion, CustomQuestionDocument,
        DocumentType,
    };
    use crate::services::Level;

    /// Build a minimal CustomQuestion for testing
    fn test_custom_question() -> CustomQuestion {
        CustomQuestion {
            id: "test-id".to_string(),
            user_id: "user-1".to_string(),
            name: "Test Quiz".to_string(),
            description: "A test quiz".to_string(),
            instructions: String::new(),
            language: "en".to_string(),
            level: Level::Easy,
            output_game: "qcm".to_string(),
            subjects: vec!["Biology".to_string()],
            num_questions: 5,
            documents: vec![],
            total_token_count: 0,
        }
    }

    #[test]
    fn test_build_qcm_prompt_contains_subjects() {
        // arrange
        let input = test_custom_question();

        // act
        let result = build_qcm_prompt(&input);

        // assert
        assert!(
            result.contains("Biology"),
            "prompt must contain the subject"
        );
    }

    #[test]
    fn test_build_qcm_prompt_empty_subjects_uses_default() {
        // arrange
        let mut input = test_custom_question();
        input.subjects = vec![];

        // act
        let result = build_qcm_prompt(&input);

        // assert
        assert!(
            result.contains("General topics"),
            "prompt must fall back to general topics"
        );
    }

    #[test]
    fn test_build_qcm_prompt_contains_document_content() {
        // arrange
        let mut input = test_custom_question();
        input.documents = vec![CustomQuestionDocument {
            filename: "bio.txt".to_string(),
            doc_type: DocumentType::Text,
            content: "Mitochondria facts".to_string(),
            token_count: 10,
        }];

        // act
        let result = build_qcm_prompt(&input);

        // assert
        assert!(
            result.contains("Mitochondria facts"),
            "prompt must embed document content"
        );
    }
}

//! Verification prompt builder

use crate::services::games::shared::prompt_helpers::get_language_name;

/// A single answer to be graded
pub struct AnswerToGrade {
    /// Question ID
    pub question_id: String,
    /// The question text
    pub question: String,
    /// The hint that was provided
    pub hint: String,
    /// The expected/correct answer
    pub expected_answer: String,
    /// The user's submitted answer
    pub user_answer: String,
}

/// Input for building a verification prompt
pub struct VerificationPromptInput {
    /// Language for the feedback
    pub language: String,
    /// The source document content (for context)
    pub source_content: String,
    /// List of answers to grade
    pub answers: Vec<AnswerToGrade>,
}

/// Build an AI prompt for grading open question answers
pub fn build_verification_prompt(
    input: &VerificationPromptInput,
) -> String {
    let answers_section = input
        .answers
        .iter()
        .enumerate()
        .map(|(i, a)| {
            format!(
                r#"### Answer {} (ID: {})
**Question:** {}
**Hint provided:** {}
**Expected answer:** {}
**User's answer:** {}"#,
                i + 1,
                a.question_id,
                a.question,
                a.hint,
                a.expected_answer,
                a.user_answer
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"**Situation**
You are an educational assessment AI tasked with grading student answers to open-ended questions. You have access to the source material the questions were based on, and you must evaluate each answer fairly and constructively.

**Task**
Grade each user answer by comparing it to the expected answer and the source material. Provide a grade and constructive feedback for each answer.

**Grading Scale**
- **right**: The answer is correct or substantially correct. The user demonstrates clear understanding of the concept.
- **medium**: The answer is partially correct. The user shows some understanding but is missing key elements or has minor errors.
- **error**: The answer is incorrect or shows fundamental misunderstanding of the concept.

---

## Source Material

The following is the source content the questions were based on. Use this to verify factual accuracy:

```
{source_content}
```

---

## Answers to Grade

{answers_section}

---

## Output Format

You MUST respond with valid JSON in the following exact format:

```json
{{
  "grades": [
    {{
      "question_id": "the-question-id",
      "grade": "right" | "medium" | "error",
      "feedback": "Constructive feedback explaining why this grade was given. Be encouraging but honest."
    }}
  ]
}}
```

---

## Grading Guidelines

1. **Be fair and consistent** - Apply the same standards to all answers
2. **Consider partial credit** - Use "medium" for answers that show understanding but are incomplete
3. **Focus on understanding** - Grade based on conceptual understanding, not exact wording
4. **Be constructive** - Feedback should help the student learn, not just criticize
5. **Reference the source** - Base your grading on the provided source material
6. **Use {language}** - All feedback must be written in {language}

## Quality Standards for Feedback

- Keep feedback concise but helpful (2-3 sentences)
- Explain what was correct (if anything)
- Explain what was missing or incorrect
- Suggest how to improve (for medium/error grades)
- Be encouraging and educational

**IMPORTANT**: Respond ONLY with the JSON output. Do not include any additional text, explanations, or markdown formatting outside the JSON structure."#,
        source_content = input.source_content,
        answers_section = answers_section,
        language = get_language_name(&input.language),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Factory for a default VerificationPromptInput
    fn test_verification_input() -> VerificationPromptInput {
        VerificationPromptInput {
            language: "en".to_string(),
            source_content: "Test source content"
                .to_string(),
            answers: vec![AnswerToGrade {
                question_id: "q1".to_string(),
                question: "What is X?".to_string(),
                hint: "Think about Y".to_string(),
                expected_answer: "X is Y".to_string(),
                user_answer: "X is Y".to_string(),
            }],
        }
    }

    #[test]
    fn test_build_verification_prompt_contains_source() {
        // arrange
        let mut input = test_verification_input();
        input.source_content =
            "Cell biology notes".to_string();

        // act
        let result = build_verification_prompt(&input);

        // assert
        assert!(
            result.contains("Cell biology notes")
        );
    }

    #[test]
    fn test_build_verification_prompt_contains_answer() {
        // arrange
        let mut input = test_verification_input();
        input.answers[0].user_answer =
            "Mitochondria".to_string();

        // act
        let result = build_verification_prompt(&input);

        // assert
        assert!(result.contains("Mitochondria"));
    }

    #[test]
    fn test_build_verification_prompt_contains_lang() {
        // arrange
        let mut input = test_verification_input();
        input.language = "fr".to_string();

        // act
        let result = build_verification_prompt(&input);

        // assert
        assert!(result.contains("French"));
    }
}

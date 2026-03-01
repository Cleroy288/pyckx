use crate::services::intello::course::domain::SectionPlan;

// ** build_section_prompt **
// ==> Builds AI prompt for generating detailed section content
//
// @ section_plan : SectionPlan from CoursePlan
// @ context : Summary of previous sections (for continuity)
// @ resources : Source documents for this section
// @ returns : Formatted prompt string for AI
pub fn build_section_prompt(
    section_plan: &SectionPlan,
    context: Option<&str>,
    resources: Option<&str>,
) -> String {
    // Step 1: Build context section if provided
    let context_section = if let Some(ctx) = context {
        format!("\n\n## Previous Sections Context\n\n{}", ctx)
    } else {
        String::new()
    };

    // Step 2: Build resources section if provided
    let resources_section = if let Some(res) = resources {
        format!("\n\n## Source Materials\n\n{}", res)
    } else {
        String::new()
    };

    // Step 3: Format key concepts list
    let key_concepts_str = section_plan.key_concepts.join("\n- ");

    // Step 4: Build complete prompt
    format!(
        r#"# SECTION CONTENT GENERATION

You are an expert educational content creator. Generate detailed, structured content for one course section.

## SECTION DETAILS

**Title**: {title}
**Section Order**: {order} (this tells you how foundational vs advanced to be)
**Key Concepts to Cover**:
- {key_concepts}

**QCM Questions to Generate**: {qcm_count}{context_section}{resources_section}

---

## YOUR TASK

Create comprehensive educational content for this section including:
1. **Content Blocks**: Text, schemas (mermaid diagrams), subtitles
2. **QCM Assessment**: {qcm_count} questions testing key concepts

## OUTPUT FORMAT

Respond with ONLY valid JSON:

```json
{{
  "order": {order},
  "title": "{title}",
  "content_blocks": [
    {{
      "type": "subtitle",
      "content": "Introduction to [Topic]"
    }},
    {{
      "type": "text",
      "content": "300-500 words explaining the concept..."
    }},
    {{
      "type": "schema",
      "language": "mermaid",
      "content": "graph TD\\n    A[Start] --> B[Process]"
    }},
    {{
      "type": "text",
      "content": "More explanation..."
    }}
  ],
  "qcm_set": {{
    "name": "{title} Assessment",
    "description": "Test your understanding of {title}",
    "level": "intermediate",
    "subjects": ["concept1", "concept2"],
    "questions": [
      {{
        "question": "Clear, specific question testing concept?",
        "right_answer": "The correct answer to this question",
        "wrong_answers": ["First wrong answer", "Second wrong answer", "Third wrong answer"],
        "explanation": "This is correct because... The first wrong answer is incorrect because... The second wrong answer fails because... The third option is wrong due to... This teaches us that understanding X is key to Y."
      }},
      {{
        "question": "Another question about a different concept?",
        "right_answer": "Another correct answer",
        "wrong_answers": ["Wrong option A", "Wrong option B", "Wrong option C"],
        "explanation": "Detailed 100+ word explanation that teaches WHY this answer is correct and WHY each wrong answer is incorrect. Include real-world examples and reinforce the learning."
      }}
    ]
  }}
}}
```

⚠️ CRITICAL JSON REQUIREMENTS - FAILURE TO FOLLOW = INVALID OUTPUT:

1. **wrong_answers MUST have EXACTLY 3 items** - not 2, not 4, EXACTLY 3
2. **Every question MUST have all fields**: question, right_answer, wrong_answers, explanation
3. **explanation MUST be 100+ words** - teach WHY correct, WHY others wrong
4. **qcm_set is MANDATORY** - every section MUST have it


**CONTENT REQUIREMENTS:**
- Use 3-5 content blocks (mix text, subtitles, schemas)
- Text blocks: 300-500 words each
- Schemas: Use mermaid syntax for diagrams
- Cover ALL key concepts listed above
- Build on previous sections if context provided

**QCM REQUIREMENTS (MANDATORY - EVERY SECTION MUST HAVE THIS):**
- The qcm_set field is REQUIRED, not optional
- Generate EXACTLY {qcm_count} questions
- Each question must test a specific key concept

**EXPLANATION REQUIREMENTS (CRITICAL):**
- Each explanation MUST be 100+ words (3-5 sentences minimum)
- Explain WHY the correct answer is right
- Explain WHY each wrong answer is incorrect  
- Include a teaching moment or real-world example
- Help the learner understand deeply, not just memorize

**OUTPUT ONLY THE JSON. No other text.**"#,
        title = section_plan.title,
        order = section_plan.order,
        key_concepts = key_concepts_str,
        qcm_count = section_plan.qcm_count,
        context_section = context_section,
        resources_section = resources_section
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a test SectionPlan
    fn test_section_plan() -> SectionPlan {
        SectionPlan {
            order: 1,
            title: "Ownership".into(),
            key_concepts: vec!["move".into(), "borrow".into()],
            qcm_count: 3,
        }
    }

    #[test]
    fn test_build_section_prompt_contains_title() {
        // arrange
        let plan = test_section_plan();

        // act
        let result = build_section_prompt(&plan, None, None);

        // assert
        assert!(result.contains("Ownership"));
    }

    #[test]
    fn test_build_section_prompt_without_context() {
        // arrange
        let plan = test_section_plan();

        // act
        let result = build_section_prompt(&plan, None, None);

        // assert
        assert!(!result.contains("Previous Sections Context"));
    }

    #[test]
    fn test_build_section_prompt_with_context() {
        // arrange
        let plan = test_section_plan();
        let ctx = "Previous section covered basics";

        // act
        let result =
            build_section_prompt(&plan, Some(ctx), None);

        // assert
        assert!(result.contains("Previous Sections Context"));
        assert!(result.contains("Previous section covered"));
    }

    #[test]
    fn test_build_section_prompt_with_resources() {
        // arrange
        let plan = test_section_plan();
        let res = "Chapter 2 content";

        // act
        let result =
            build_section_prompt(&plan, None, Some(res));

        // assert
        assert!(result.contains("Source Materials"));
        assert!(result.contains("Chapter 2 content"));
    }
}

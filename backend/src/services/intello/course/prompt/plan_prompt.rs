use crate::services::intello::course::domain::ExtractedIdeas;

// ** build_course_plan_prompt **
// ==> Builds AI prompt for generating course structure from ideas
//
// @ ideas : ExtractedIdeas from Stage 1
// @ section_count : Desired number of sections
// @ returns : Formatted prompt string for AI
pub fn build_course_plan_prompt(ideas: &ExtractedIdeas, section_count: u8) -> String {
    // Step 1: Format mandatory topics
    let mandatory_topics_str = ideas.mandatory_topics.join(", ");

    // Step 2: Format suggested topics
    let suggested_topics_str = ideas.suggested_topics.join(", ");

    // Step 3: Build complete prompt
    format!(
        r#"# COURSE STRUCTURE PLANNING

You are an expert course designer. Create a logical, well-structured course outline.

## LEARNING OBJECTIVES

**Core Intent**: {core_intent}
**Target Level**: {target_level}
**Must Cover**: {mandatory_topics}
**Recommended**: {suggested_topics}

## YOUR TASK

Design a course with {section_count} sections that:
1. Builds concepts progressively (each section builds on previous)
2. Covers all mandatory topics
3. Includes relevant suggested topics where appropriate
4. Has 3-5 QCM questions per section for assessment

## OUTPUT FORMAT

Respond with ONLY valid JSON:

```json
{{
  "title": "Course Title (concise, descriptive)",
  "subtitle": "One-sentence description of what students will learn",
  "sections": [
    {{
      "order": 1,
      "title": "Section 1 Title",
      "key_concepts": ["concept1", "concept2", "concept3"],
      "qcm_count": 5
    }},
    {{
      "order": 2,
      "title": "Section 2 Title",
      "key_concepts": ["concept4", "concept5", "concept6"],
      "qcm_count": 5
    }}
  ]
}}
```

**REQUIREMENTS:**
- Each section must have 3-5 key_concepts
- qcm_count must be between 3-5 per section
- Sections must build logically (prerequisites → advanced)
- Cover all mandatory topics across sections

**OUTPUT ONLY THE JSON. No other text.**"#,
        core_intent = ideas.core_intent,
        target_level = ideas.target_level,
        mandatory_topics = mandatory_topics_str,
        suggested_topics = suggested_topics_str,
        section_count = section_count
    )
}

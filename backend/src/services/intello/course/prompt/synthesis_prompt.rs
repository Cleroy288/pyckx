use crate::services::intello::course::domain::{CoursePlan, ParsedSection};

// ** build_synthesis_prompt **
// ==> Builds AI prompt for generating course synthesis with final QCM
//
// @ course_plan : Original CoursePlan
// @ parsed_sections : All generated sections
// @ returns : Formatted prompt string for AI
pub fn build_synthesis_prompt(course_plan: &CoursePlan, parsed_sections: &[ParsedSection]) -> String {
    // Step 1: Build sections summary
    let sections_summary = parsed_sections
        .iter()
        .map(|s| format!("**Section {}**: {} - Key concepts: {}", 
            s.order, 
            s.title,
            s.content_blocks
                .iter()
                .filter_map(|block| {
                    if let crate::http_api::data_transfer_object::intello::course::ContentBlock::Subtitle { content } = block {
                        Some(content.as_str())
                    } else {
                        None
                    }
                })
                .take(3)
                .collect::<Vec<_>>()
                .join(", ")
        ))
        .collect::<Vec<_>>()
        .join("\n");

    // Step 2: Extract all topics covered
    let all_topics: Vec<String> = parsed_sections
        .iter()
        .flat_map(|s| s.qcm_set.subjects.clone())
        .collect();
    let topics_str = all_topics.join(", ");

    // Step 3: Build complete prompt
    format!(
        r#"# COURSE SYNTHESIS GENERATION

You are an expert educational content creator. Generate a comprehensive synthesis for the entire course.

## COURSE OVERVIEW

**Course Title**: {title}
**Description**: {subtitle}

## SECTIONS COVERED

{sections_summary}

**All Topics**: {topics}

---

## YOUR TASK

Create a comprehensive course synthesis including:
1. **Summary Text**: 500-700 words reviewing the entire course
2. **Key Takeaways**: 5-8 bullet points of main learnings
3. **Final QCM**: 10-15 questions covering the entire course

## OUTPUT FORMAT

Respond with ONLY valid JSON:

```json
{{
  "summary_text": "Comprehensive 500-700 word summary reviewing all sections, how they connect, and the overall learning journey. Emphasize key insights and how concepts build on each other.",
  "key_takeaways": [
    "First major learning point",
    "Second major learning point",
    "Third major learning point",
    "Fourth major learning point",
    "Fifth major learning point"
  ],
  "final_qcm": {{
    "name": "Final Course Assessment",
    "description": "Comprehensive assessment covering all course concepts",
    "level": "intermediate",
    "subjects": ["topic1", "topic2", "topic3"],
    "questions": [
      {{
        "question": "Question testing concept from Section 1?",
        "right_answer": "Correct answer",
        "wrong_answers": ["Wrong 1", "Wrong 2", "Wrong 3"],
        "explanation": "Detailed explanation connecting to course concepts"
      }}
    ]
  }}
}}
```

**SYNTHESIS REQUIREMENTS:**
- Summary must review ALL sections in logical order
- Highlight how concepts connect and build on each other
- Emphasize practical applications and key insights
- Write in an engaging, reflective tone

**KEY TAKEAWAYS REQUIREMENTS:**
- 5-8 bullet points
- Each should be a complete, meaningful statement
- Cover different sections/topics
- Focus on actionable knowledge

**FINAL QCM REQUIREMENTS:**
- 10-15 questions covering the ENTIRE course
- Mix difficulty (some integration questions combining concepts)
- Ensure all major topics from all sections are tested
- Explanations should reinforce learning from the entire course

**OUTPUT ONLY THE JSON. No other text.**"#,
        title = course_plan.title,
        subtitle = course_plan.subtitle,
        sections_summary = sections_summary,
        topics = topics_str
    )
}


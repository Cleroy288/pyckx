
// ** build_ideas_extraction_prompt **
// ==> Builds AI prompt for extracting user demand ideas
//
// @ user_topic : User's raw topic/request
// @ resources_content : Optional source documents
// @ returns : Formatted prompt string for AI
pub fn build_ideas_extraction_prompt(
    user_topic: &str,
    resources_content: Option<&str>,
) -> String {
    // Step 1: Build resources section if provided
    let resources_section = if let Some(resources) = resources_content {
        format!("\n\n## Source Materials\n\n{}", resources)
    } else {
        String::new()
    };

    // Step 2: Build complete prompt
    format!(
        r#"# EDUCATIONAL DEMAND ANALYSIS

You are an expert educational analyst. Your task is to analyze a user's learning request and extract the core learning objectives.

## USER REQUEST

{user_topic}{resources_section}

---

## YOUR TASK

Analyze the user's request and extract:
1. **Core Intent**: What is the main learning objective?
2. **Target Level**: Beginner, Intermediate, or Advanced?
3. **Mandatory Topics**: What specific topics did the user explicitly mention?
4. **Suggested Topics**: What related topics would enhance learning?
5. **Constraints**: Any time limits, prerequisites, or special requirements mentioned?

## OUTPUT FORMAT

Respond with ONLY valid JSON:

```json
{{
  "core_intent": "Main learning objective in one sentence",
  "target_level": "beginner|intermediate|advanced",
  "mandatory_topics": ["topic1",  "topic2", "topic3"],
  "suggested_topics": ["related_topic1", "related_topic2"],
  "constraints": ["constraint1", "constraint2"]
}}
```

**IMPORTANT:**
- Be specific with topics (not too broad, not too narrow)
- Suggest 2-4 related topics that logically connect
- Identify any implicit constraints (e.g., "quick overview" → time constraint)

**OUTPUT ONLY THE JSON. No other text.**"#,
        user_topic = user_topic,
        resources_section = resources_section
    )
}


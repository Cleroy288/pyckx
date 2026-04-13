//! Shared prompt helpers for all game types

use crate::services::Level;

// Re-export game_formats for backward compatibility
pub use super::game_formats::{
    get_game_format, get_game_format_or_default,
    GameOutputFormat, GAME_FORMATS,
};

/// Default text when no instructions are provided
pub const DEFAULT_INSTRUCTIONS: &str =
    "No additional instructions provided.";

/// Default text when no subjects are provided
pub const DEFAULT_SUBJECTS: &str =
    "General topics from the provided content";

/// Shared input for all game prompt builders
pub struct GamePromptInput {
    /// Name of the game/quiz set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language code (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics to focus on
    pub subjects: Vec<String>,
    /// Number of items to generate
    pub num_items: u32,
    /// Document contents (filename, content)
    pub documents: Vec<(String, String)>,
}

/// Format subjects list with fallback to default
pub fn format_subjects(subjects: &[String]) -> String {
    if subjects.is_empty() {
        DEFAULT_SUBJECTS.to_string()
    } else {
        subjects.join(", ")
    }
}

/// Format documents into markdown sections
pub fn format_documents(
    docs: &[(String, String)],
) -> String {
    docs.iter()
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
        .join("\n\n")
}

/// Return instructions or the default fallback
pub fn format_instructions(instructions: &str) -> &str {
    if instructions.is_empty() {
        DEFAULT_INSTRUCTIONS
    } else {
        instructions
    }
}

/// Convert Level enum to detailed guideline string
pub fn level_to_string(level: &Level) -> &'static str {
    match level {
        Level::Easy => {
            r#"EASY - Beginner Level
   - Use simple, everyday vocabulary (avoid jargon and technical terms)
   - Ask about basic facts, definitions, and simple concepts
   - Questions should be straightforward with obvious correct answers
   - Focus on "what", "who", "when" type questions
   - Answers should be short and direct
   - Wrong answers should be clearly distinguishable from correct ones"#
        }
        Level::Medium => {
            r#"MEDIUM - Intermediate Level
   - Use appropriate technical vocabulary with context
   - Ask about relationships, causes, effects, and applications
   - Questions require understanding, not just memorization
   - Include "why", "how", and "explain" type questions
   - Answers may require connecting multiple concepts
   - Wrong answers should be plausible but distinguishable"#
        }
        Level::Hard => {
            r#"HARD - Advanced Level
   - Use precise technical and domain-specific terminology
   - Ask about complex relationships, analysis, and synthesis
   - Questions require deep understanding and critical thinking
   - Include scenario-based, analytical, and evaluation questions
   - Answers require integrating multiple concepts and reasoning
   - Wrong answers should be sophisticated and require careful analysis to eliminate"#
        }
    }
}

/// Map a language code to its full name
pub fn get_language_name(code: &str) -> &'static str {
    match code.to_lowercase().as_str() {
        "en" => "English",
        "fr" => "French",
        "es" => "Spanish",
        "de" => "German",
        "it" => "Italian",
        "pt" => "Portuguese",
        "nl" => "Dutch",
        "pl" => "Polish",
        "ru" => "Russian",
        "ja" => "Japanese",
        "zh" => "Chinese (Simplified)",
        _ => "English",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::Level;

    #[test]
    fn test_format_subjects_with_values() {
        // arrange
        let subjects =
            vec!["Math".to_string(), "Physics".to_string()];

        // act
        let result = format_subjects(&subjects);

        // assert
        assert_eq!(result, "Math, Physics");
    }

    #[test]
    fn test_format_subjects_empty_returns_default() {
        // arrange / act
        let result = format_subjects(&[]);

        // assert
        assert_eq!(result, DEFAULT_SUBJECTS);
    }

    #[test]
    fn test_format_documents_single_doc() {
        // arrange
        let docs = vec![(
            "notes.txt".to_string(),
            "Content here".to_string(),
        )];

        // act
        let result = format_documents(&docs);

        // assert
        assert!(result.contains("Document 1 - notes.txt"));
        assert!(result.contains("Content here"));
    }

    #[test]
    fn test_format_documents_empty_returns_empty() {
        // arrange / act
        let result = format_documents(&[]);

        // assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_format_instructions_empty_returns_default() {
        // arrange / act
        let result = format_instructions("");

        // assert
        assert_eq!(result, DEFAULT_INSTRUCTIONS);
    }

    #[test]
    fn test_format_instructions_nonempty_returns_value() {
        // arrange / act
        let result = format_instructions("Do X");

        // assert
        assert_eq!(result, "Do X");
    }

    #[test]
    fn test_level_to_string_easy_contains_easy() {
        // arrange / act
        let result = level_to_string(&Level::Easy);

        // assert
        assert!(result.contains("EASY"));
    }

    #[test]
    fn test_level_to_string_medium_contains_medium() {
        // arrange / act
        let result = level_to_string(&Level::Medium);

        // assert
        assert!(result.contains("MEDIUM"));
    }

    #[test]
    fn test_level_to_string_hard_contains_hard() {
        // arrange / act
        let result = level_to_string(&Level::Hard);

        // assert
        assert!(result.contains("HARD"));
    }

    #[test]
    fn test_get_language_name_known_codes() {
        // arrange
        let cases = vec![
            ("en", "English"),
            ("fr", "French"),
            ("es", "Spanish"),
            ("de", "German"),
            ("it", "Italian"),
            ("pt", "Portuguese"),
            ("nl", "Dutch"),
            ("pl", "Polish"),
            ("ru", "Russian"),
            ("ja", "Japanese"),
            ("zh", "Chinese (Simplified)"),
        ];

        for (code, expected) in cases {
            // act
            let result = get_language_name(code);

            // assert
            assert_eq!(
                result, expected,
                "get_language_name('{}') failed",
                code
            );
        }
    }

    #[test]
    fn test_get_language_name_unknown_defaults_english() {
        // arrange / act
        let result = get_language_name("xx");

        // assert
        assert_eq!(result, "English");
    }

    #[test]
    fn test_get_language_name_case_insensitive() {
        // arrange / act
        let result = get_language_name("FR");

        // assert
        assert_eq!(result, "French");
    }
}

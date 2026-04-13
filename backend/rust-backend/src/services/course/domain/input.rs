/// Input for generating a full AI course
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GenerateCourseInput {
    pub topic: String,
    pub keywords: Vec<String>,
    pub instructions: String,
    pub resources: String,
    pub session_id: Option<String>,
    pub text_length: Option<String>,
    pub exercise_depth: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ** helper to build a test input with all fields **
    fn test_input() -> GenerateCourseInput {
        GenerateCourseInput {
            topic: "Rust".to_string(),
            keywords: vec!["ownership".to_string()],
            instructions: "Focus on safety".to_string(),
            resources: "The Rust Book".to_string(),
            session_id: Some("sess-1".to_string()),
            text_length: Some("long".to_string()),
            exercise_depth: Some("deep".to_string()),
        }
    }

    #[test]
    fn test_generate_course_input_clone() {
        // arrange
        let input = test_input();

        // act
        let cloned = input.clone();

        // assert
        assert_eq!(cloned.topic, input.topic);
        assert_eq!(cloned.keywords, input.keywords);
        assert_eq!(cloned.instructions, input.instructions);
        assert_eq!(cloned.resources, input.resources);
        assert_eq!(cloned.session_id, input.session_id);
        assert_eq!(cloned.text_length, input.text_length);
        assert_eq!(
            cloned.exercise_depth,
            input.exercise_depth
        );
    }

    #[test]
    fn test_generate_course_input_debug() {
        // arrange
        let input = test_input();

        // act
        let debug_str = format!("{:?}", input);

        // assert
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_generate_course_input_optional_fields_none() {
        // arrange / act
        let input = GenerateCourseInput {
            topic: "Math".to_string(),
            keywords: vec![],
            instructions: String::new(),
            resources: String::new(),
            session_id: None,
            text_length: None,
            exercise_depth: None,
        };

        // assert
        assert!(input.session_id.is_none());
        assert!(input.text_length.is_none());
        assert!(input.exercise_depth.is_none());
    }
}

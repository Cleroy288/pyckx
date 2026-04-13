// impl pour un coding game
//
use crate::infra::openrouter::OpenRouterClient;
use crate::infra::openrouter::extract_json_from_response;
use crate::services::error_domain::StudyError;

use serde::Deserialize;

/// Format JSON attendu de l'IA
#[derive(Debug, Deserialize)]
pub struct CodingAiResponse {
    pub exercise: CodingAiExercise,
}

#[derive(Debug, Deserialize)]
pub struct CodingAiExercise {
    pub subject: String,
    pub code_snippet: String,
}

pub struct CodingGame {
    openrouter_client: OpenRouterClient,
}

impl CodingGame {
    pub fn new(
        openrouter_client: OpenRouterClient,
    ) -> Self {
        Self { openrouter_client }
    }

    pub fn check(expected: &str, user_code: &str) -> bool {
        let normalize = |s: &str| -> String {
            s.chars().filter(|c| !c.is_whitespace()).collect()
        };
        normalize(expected) == normalize(user_code)
    }

    pub async fn generate(
        &self,
        language: &str,
        subject: &str,
        level: &str,
        langue: &str,
    ) -> Result<CodingAiExercise, StudyError> {
        let prompt = Self::build_prompt(language, subject, level, langue);
        let ai_result = self.send_chat_request(&prompt).await?;
        Self::parse_coding_response(&ai_result)
    }

    fn build_prompt(
        language: &str,
        subject: &str,
        level: &str,
        langue: &str,
    ) -> String {
        let topic = if subject.is_empty() {
            format!("general {language} programming")
        } else {
            format!("{subject} in {language}")
        };

        let output_lang = match langue {
            "fr" => "French",
            "nl" => "Dutch",
            _ => "English",
        };

        format!(
            r#"Generate a {level} difficulty code snippet about {topic}.
The programming language is {language}.
Write the subject title in {output_lang}.

The student will re-type this code to learn syntax.
The code must be correct, clean, no comments.
Difficulty controls length: higher == more code.

Respond with ONLY valid JSON, no other text:

```json
{{
  "exercise": {{
    "subject": "<short title in {output_lang}, max 3 words>",
    "code_snippet": "<valid {language} code, no comments>"
  }}
}}
```"#
        )
    }

    /// Send chat request to the AI
    async fn send_chat_request(
        &self,
        prompt: &str,
    ) -> Result<String, StudyError> {
        let ai_result = self
            .openrouter_client
            .send_chat_request(prompt, None)
            .await?;
        Ok(ai_result.content)
    }

    /// Parse la réponse JSON de l'IA
    fn parse_coding_response(
        raw: &str,
    ) -> Result<CodingAiExercise, StudyError> {
        let json = extract_json_from_response(raw);

        let parsed: CodingAiResponse =
            serde_json::from_str(&json).map_err(|e| {
                StudyError::validation(
                    "ai_response",
                    format!(
                        "Failed to parse coding exercise: {}",
                        e
                    ),
                )
            })?;

        Ok(parsed.exercise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::openrouter::OpenRouterClient;

    #[test]
    fn test_build_prompt_contains_language_and_subject() {
        let prompt = CodingGame::build_prompt(
            "rust", "ownership", "medium", "eng",
        );

        assert!(prompt.contains("ownership"));
        assert!(prompt.contains("rust"));
        assert!(prompt.contains("medium"));
        assert!(prompt.contains("English"));
    }

    #[test]
    fn test_build_prompt_empty_subject_uses_general() {
        let prompt = CodingGame::build_prompt(
            "python", "", "easy", "fr",
        );

        assert!(prompt.contains("general python"));
        assert!(prompt.contains("French"));
    }

    #[test]
    fn test_parse_coding_response_valid_json_returns_exercise() {
        // arrange
        let json = r#"{
            "exercise": {
                "subject": "Rust",
                "code_snippet": "fn main() {}"
            }
        }"#;

        // act
        let result =
            CodingGame::parse_coding_response(json)
                .unwrap();

        // assert
        assert_eq!(result.subject, "Rust");
        assert_eq!(result.code_snippet, "fn main() {}");
    }

    #[test]
    fn test_parse_coding_response_invalid_json_returns_error() {
        // act
        let result =
            CodingGame::parse_coding_response("bad json");

        // assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_openrouter_generate_coding_game() {
        let api_key = std::env::var("OPENROUTER_API_KEY")
            .expect("OPENROUTER_API_KEY must be set");
        let client = OpenRouterClient::new(api_key);
        let game = CodingGame::new(client);

        let result = game
            .generate("rust", "loops", "easy", "eng")
            .await;

        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let exercise = result.unwrap();
        assert!(!exercise.code_snippet.is_empty());
    }
}

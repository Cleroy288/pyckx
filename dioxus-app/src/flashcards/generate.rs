//! Flashcard generation page — wires the generic
//! `GameGeneratePage` to the AI flashcards endpoint.

use crate::api::form_data::build_generate_form;
use crate::components::game_shared::generate_form::{
    GenerateFormData as LegacyFormData,
};
use crate::flashcards::api;
use crate::game_engine::{
    ApiCall, GameGeneratePage, GenerateFormData,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Hero title for the generation page.
const TITLE: &str = "Generate Flashcard";
/// Toast shown after a successful generation.
const SUCCESS_MSG: &str = "Generated!";
/// Path navigated to after success.
const REDIRECT: &str = "/flashcards";

/// Flashcard generation page.
pub fn FlashcardGeneratePage() -> Element {
    let api_call = generate_call();
    rsx! {
        GameGeneratePage {
            title: TITLE,
            success_msg: SUCCESS_MSG,
            redirect: REDIRECT,
            api_call: api_call,
        }
    }
}

/// Build the boxed API call passed to `GameGeneratePage`.
fn generate_call() -> ApiCall {
    ApiCall(Arc::new(|data| {
        Box::pin(async move {
            let legacy = to_legacy_form(data);
            let form = build_generate_form(&legacy);
            api::create_flashcards(&form)
                .await
                .map(|_| ())
        })
    }))
}

/// Convert the engine form data to the legacy struct
/// expected by `build_generate_form`. Both structs share
/// the same field shape — this is a 1:1 copy.
fn to_legacy_form(data: GenerateFormData) -> LegacyFormData {
    LegacyFormData {
        name: data.name,
        description: data.description,
        instructions: data.instructions,
        language: data.language,
        level: data.level,
        subjects: data.subjects,
        num_questions: data.num_questions,
        files: data.files,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> GenerateFormData {
        GenerateFormData {
            name: "n".into(),
            description: "d".into(),
            instructions: "i".into(),
            language: "en".into(),
            level: "easy".into(),
            subjects: vec!["math".into()],
            num_questions: 5,
            files: vec!["a.pdf".into()],
        }
    }

    #[test]
    fn test_to_legacy_form_copies_name() {
        let out = to_legacy_form(sample());
        assert_eq!(out.name, "n");
    }

    #[test]
    fn test_to_legacy_form_copies_subjects() {
        let out = to_legacy_form(sample());
        assert_eq!(out.subjects, vec!["math".to_string()]);
    }

    #[test]
    fn test_to_legacy_form_copies_files() {
        let out = to_legacy_form(sample());
        assert_eq!(out.files, vec!["a.pdf".to_string()]);
    }

    #[test]
    fn test_to_legacy_form_copies_num_questions() {
        let out = to_legacy_form(sample());
        assert_eq!(out.num_questions, 5);
    }
}

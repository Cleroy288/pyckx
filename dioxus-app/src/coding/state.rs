//! Coding state — the in-flight exercise shared between
//! the generate page and the play page.

use dioxus::prelude::*;

use crate::coding::api::ExerciseResponse;

/// Domain model for a coding exercise loaded in the UI.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CodingExercise {
    pub subject: String,
    pub code_snippet: String,
    pub language: String,
}

impl CodingExercise {
    /// Build a domain exercise from the API response and chosen language.
    pub fn from_response(
        resp: ExerciseResponse,
        language: String,
    ) -> Self {
        Self {
            subject: resp.subject,
            code_snippet: resp.code_snippet,
            language,
        }
    }
}

/// Reactive context type for the current exercise (None = empty).
pub type ExerciseSignal = Signal<Option<CodingExercise>>;

/// Provides the empty exercise context to descendants.
#[component]
pub fn CodingExerciseProvider(children: Element) -> Element {
    use_context_provider(|| {
        Signal::new(None::<CodingExercise>)
    });
    rsx! { {children} }
}

/// Hook to read/write the current coding exercise.
pub fn use_exercise() -> ExerciseSignal {
    use_context::<ExerciseSignal>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_response_copies_fields() {
        let resp = ExerciseResponse {
            subject: "loops".into(),
            code_snippet: "fn x() {}".into(),
        };
        let ex = CodingExercise::from_response(
            resp,
            "rust".into(),
        );
        assert_eq!(ex.subject, "loops");
    }

    #[test]
    fn test_from_response_keeps_language() {
        let resp = ExerciseResponse {
            subject: "s".into(),
            code_snippet: "c".into(),
        };
        let ex = CodingExercise::from_response(
            resp,
            "python".into(),
        );
        assert_eq!(ex.language, "python");
    }
}

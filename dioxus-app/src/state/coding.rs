use dioxus::prelude::*;

use crate::api::coding::CodingExerciseResponse;

#[derive(Clone, Debug, Default)]
pub struct CodingExercise {
    pub subject: String,
    pub code_snippet: String,
    pub language: String,
}

impl From<(CodingExerciseResponse, String)> for CodingExercise {
    fn from((resp, language): (CodingExerciseResponse, String)) -> Self {
        Self {
            subject: resp.subject,
            code_snippet: resp.code_snippet,
            language,
        }
    }
}

#[component]
pub fn CodingExerciseProvider(
    children: Element,
) -> Element {
    use_context_provider(|| {
        Signal::new(None::<CodingExercise>)
    });
    rsx! { {children} }
}

//! Coding generate page — assembles the form, submits it,
//! stores the resulting exercise, then routes to play.

use dioxus::prelude::*;
use dioxus::router::Navigator;

use crate::auth::use_auth_guard;
use crate::coding::api::{generate, GenerateRequest};
use crate::coding::generate_form::{
    opt_if_not, opt_trim, use_form, use_status, FormState,
    FormView, StatusState, LANGUE_DEFAULT,
};
use crate::coding::state::{use_exercise, CodingExercise};
use crate::home::HomeTopBar;
use crate::routes::Route;
use crate::ui::{HeroBanner, PageLayout};

/// Page: form to request a new coding exercise.
pub fn CodingGeneratePage() -> Element {
    let _auth = use_auth_guard();
    let form = use_form();
    let status = use_status();
    let exercise = use_exercise();
    let nav = navigator();

    let on_submit = move |_| {
        submit(form, status, exercise, nav);
    };

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner { title: "Coding Game", span {} }
                FormView {
                    form: form,
                    status: status,
                    on_submit: on_submit,
                }
            }
        }
    }
}

/// Build the API request from current form values.
fn to_request(form: &FormState) -> GenerateRequest {
    GenerateRequest {
        language: (form.language)(),
        subject: opt_trim((form.subject)()),
        level: (form.level)(),
        langue: opt_if_not((form.langue)(), LANGUE_DEFAULT),
    }
}

/// Fire the async generate request and update state on result.
fn submit(
    form: FormState,
    mut status: StatusState,
    mut exercise: Signal<Option<CodingExercise>>,
    nav: Navigator,
) {
    status.loading.set(true);
    status.error.set(None);
    let req = to_request(&form);
    let lang = (form.language)();

    spawn(async move {
        let result = generate(&req).await;
        apply_result(result, lang, &mut exercise, &mut status, nav);
    });
}

/// Dispatch the API outcome: route on success, surface error otherwise.
fn apply_result(
    result: Result<crate::coding::api::ExerciseResponse, String>,
    lang: String,
    exercise: &mut Signal<Option<CodingExercise>>,
    status: &mut StatusState,
    nav: Navigator,
) {
    match result {
        Ok(resp) => {
            exercise.set(Some(
                CodingExercise::from_response(resp, lang),
            ));
            nav.push(Route::CodingPlay {});
        }
        Err(e) => status.error.set(Some(e)),
    }
    status.loading.set(false);
}

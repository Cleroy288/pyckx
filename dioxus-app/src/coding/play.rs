//! Coding play page — shows the snippet, hosts the editor,
//! submits the user's solution, displays the verdict.

use dioxus::prelude::*;

use crate::auth::use_auth_guard;
use crate::coding::api::{check, CheckRequest};
use crate::coding::play_view::{
    EmptyExercise, ExerciseBody, StatusState,
};
use crate::coding::state::{use_exercise, CodingExercise};
use crate::home::HomeTopBar;
use crate::ui::{
    get_editor_value, HeroBanner, PageLayout,
};

const EDITOR_ID: &str = "coding-play-editor";

/// Page: solve the loaded exercise and submit it for checking.
pub fn CodingPlayPage() -> Element {
    let _auth = use_auth_guard();
    let Some(exercise) = (use_exercise())() else {
        return rsx! { EmptyExercise {} };
    };

    let status = StatusState::new();
    let on_submit = make_submit(exercise.clone(), status);

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner { title: "{exercise.subject}", span {} }
                ExerciseBody {
                    exercise: exercise,
                    editor_id: EDITOR_ID.to_string(),
                    status: status,
                    on_submit: on_submit,
                }
            }
        }
    }
}

/// Build the click handler that runs the check call.
fn make_submit(
    exercise: CodingExercise,
    status: StatusState,
) -> impl FnMut(MouseEvent) + 'static {
    move |_| {
        let expected = exercise.code_snippet.clone();
        run_check(expected, status);
    }
}

/// Spawn the async check workflow and update status reactively.
fn run_check(expected: String, mut status: StatusState) {
    status.start();
    spawn(async move {
        let user_code = get_editor_value(EDITOR_ID).await;
        let req = CheckRequest { expected, user_code };
        match check(&req).await {
            Ok(resp) => status.finish(Some(resp.is_correct)),
            Err(e) => status.fail(e),
        }
    });
}

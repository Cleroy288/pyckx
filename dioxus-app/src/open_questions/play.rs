//! Open-question play page — loads a set, runs the player.
//!
//! Top-level page wires the generic `GamePlayPage` to the
//! open-question API and to a wrapper that owns the AI
//! grading state shared with `OpenQuestionPlayer`.

use super::api;
use super::player::OpenQuestionPlayer;
use super::types::{
    CheckAnswersRequest, CheckAnswersResponse,
    OpenQuestionSet,
};
use crate::game_engine::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

const BACK_PATH: &str = "/open-questions";

/// Play page: receives the set ID from the route.
pub fn OpenQuestionPlayPage(id: String) -> Element {
    rsx! {
        GamePlayPage {
            id: id,
            back_path: BACK_PATH,
            fetch_set: build_fetch(),
            render_player: build_renderer(),
        }
    }
}

/// Build the async loader passed to the generic play page.
fn build_fetch() -> FetchSetById<OpenQuestionSet> {
    FetchSetById(Arc::new(|id: String| {
        Box::pin(api::find_set_by_id(id))
    }))
}

/// Build the renderer that mounts the wrapper component.
fn build_renderer() -> PlayerRenderer<OpenQuestionSet> {
    PlayerRenderer(Arc::new(|set, on_back| {
        rsx! {
            PlayerWithGrading { set: set, on_back: on_back }
        }
    }))
}

/// Wrapper component owning the AI grading state.
///
/// Kept here (not in `player.rs`) because the API call and
/// the navigation lifecycle belong to the page layer.
#[component]
fn PlayerWithGrading(
    set: OpenQuestionSet,
    on_back: EventHandler<()>,
) -> Element {
    let grading = use_signal(|| false);
    let grades: Signal<Option<CheckAnswersResponse>> =
        use_signal(|| None);
    let error: Signal<Option<String>> = use_signal(|| None);
    let on_check =
        check_handler(grading, grades, error);

    rsx! {
        OpenQuestionPlayer {
            set: set,
            on_back: on_back,
            on_check: on_check,
            grades: grades,
            grading: grading,
            error: error,
        }
    }
}

/// Build the on-check callback: spawns the grading request,
/// updates `grades` on success or `error` on failure, and
/// flips the `grading` flag at start/end.
fn check_handler(
    mut grading: Signal<bool>,
    mut grades: Signal<Option<CheckAnswersResponse>>,
    mut error: Signal<Option<String>>,
) -> impl FnMut(CheckAnswersRequest) {
    move |req: CheckAnswersRequest| {
        grading.set(true);
        error.set(None);
        spawn(async move {
            match api::grade_answers(&req).await {
                Ok(r) => grades.set(Some(r)),
                Err(e) => error.set(Some(e)),
            }
            grading.set(false);
        });
    }
}

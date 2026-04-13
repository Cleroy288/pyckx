//! Open question play page

use crate::api;
use crate::components::open_question::open_question_player::OpenQuestionPlayer;
use crate::components::game_shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use crate::domain::open_question_types::*;
use dioxus::prelude::*;
use std::sync::Arc;

/// Open question play page — receives id from route
pub fn OpenQuestionPlayPage(id: String) -> Element {
    let fetch: FetchSetById<_> =
        FetchSetById(Arc::new(|id: String| {
            Box::pin(async move {
                api::open_questions::get_open_question_sets()
                    .await
                    .ok()
                    .and_then(|v| {
                        v.into_iter()
                            .find(|s| s.id == id)
                    })
            })
        }));

    let render: PlayerRenderer<_> =
        PlayerRenderer(Arc::new(|s, on_back| {
            rsx! {
                OqPlayerWrapper {
                    set: s,
                    on_back: on_back,
                }
            }
        }));

    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/open-questions",
            fetch_set: fetch,
            render_player: render,
        }
    }
}

/// Wrapper that owns API grading state
#[component]
fn OqPlayerWrapper(
    set: OpenQuestionSet,
    on_back: EventHandler<()>,
) -> Element {
    let mut grading = use_signal(|| false);
    let mut grades: Signal<
        Option<CheckAnswersResponse>,
    > = use_signal(|| None);
    let mut error: Signal<Option<String>> =
        use_signal(|| None);

    let on_check =
        move |req: CheckAnswersRequest| {
            grading.set(true);
            error.set(None);
            spawn(async move {
                match api::open_questions::check_answers(
                    &req,
                )
                .await
                {
                    Ok(r) => grades.set(Some(r)),
                    Err(e) => error.set(Some(e)),
                }
                grading.set(false);
            });
        };

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

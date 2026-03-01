use crate::api;
use crate::components::intello::open_question::open_question_player::OpenQuestionPlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Open question play page
#[component]
pub fn OpenQuestionPlayPage() -> impl IntoView {
    let fetch: FetchSetById<_> =
        Arc::new(|id: String| {
            Box::pin(async move {
                api::open_questions::get_open_question_sets()
                    .await
                    .ok()
                    .and_then(|v| {
                        v.into_iter()
                            .find(|s| s.id == id)
                    })
            }) as _
        });

    let render: PlayerRenderer<_> =
        Arc::new(|s, on_back| {
            view! {
                <OpenQuestionPlayer
                    set=s on_back=on_back
                />
            }
            .into_any()
        });

    view! {
        <GamePlayPage
            back_path="/intello/open-questions"
            fetch_set=fetch
            render_player=render
        />
    }
}

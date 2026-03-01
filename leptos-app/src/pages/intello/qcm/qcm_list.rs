use crate::api;
use crate::components::intello::shared::game_list_page::{
    DeleteFn, FetchSets, GameListPage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// QCM list page — thin wrapper over GameListPage
#[component]
pub fn QcmListPage() -> impl IntoView {
    let fetch: FetchSets<_> = Arc::new(|| {
        Box::pin(api::intello::get_qcm_sets())
    });
    let delete: DeleteFn = Arc::new(|id| {
        Box::pin(
            api::intello::delete_qcm_set_owned(id),
        )
    });

    view! {
        <GameListPage
            title="QCM Sets"
            game_label="QCM"
            generate_path="/intello/qcm/generate"
            play_path_prefix="/intello/qcm/play"
            fetch_sets=fetch
            create_path="/intello/qcm/create"
            delete_fn=delete
            quick_path="/intello/qcm/quick"
        />
    }
}

//! QCM list page — thin wrapper over GameListPage

use crate::api;
use crate::components::intello::shared::game_list_page::{
    DeleteFn, FetchSets, GameListPage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// QCM list page
pub fn QcmListPage() -> Element {
    let fetch: FetchSets<_> =
        FetchSets(Arc::new(|| {
            Box::pin(api::intello::get_qcm_sets())
        }));
    let delete: DeleteFn =
        DeleteFn(Arc::new(|id| {
            Box::pin(
                api::intello::delete_qcm_set_owned(
                    id,
                ),
            )
        }));

    rsx! {
        GameListPage {
            title: "QCM Sets",
            game_label: "QCM",
            generate_path: "/intello/qcm/generate",
            play_path_prefix: "/intello/qcm/play",
            fetch_sets: fetch,
            create_path: "/intello/qcm/create",
            delete_fn: delete,
            quick_path: "/intello/qcm/quick",
        }
    }
}

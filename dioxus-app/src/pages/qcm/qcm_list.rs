//! QCM list page — thin wrapper over GameListPage

use crate::api;
use crate::components::game_shared::game_list_page::{
    DeleteFn, FetchSets, GameListPage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// QCM list page
pub fn QcmListPage() -> Element {
    let fetch: FetchSets<_> =
        FetchSets(Arc::new(|| {
            Box::pin(api::study::get_qcm_sets())
        }));
    let delete: DeleteFn =
        DeleteFn(Arc::new(|id| {
            Box::pin(
                api::study::delete_qcm_set_owned(
                    id,
                ),
            )
        }));

    rsx! {
        GameListPage {
            title: "QCM Sets",
            game_label: "QCM",
            generate_path: "/qcm/generate",
            play_path_prefix: "/qcm/play",
            fetch_sets: fetch,
            create_path: "/qcm/create",
            delete_fn: delete,
            quick_path: "/qcm/quick",
        }
    }
}

//! QCM list page — thin wrapper over `GameListPage`.

use std::sync::Arc;

use dioxus::prelude::*;

use crate::game_engine::{
    DeleteFn, FetchSets, GameListPage,
};
use crate::qcm::api;
use crate::qcm::types::QcmSet;

const TITLE: &str = "QCM Sets";
const LABEL: &str = "QCM";
const PATH_PLAY: &str = "/qcm/play";
const PATH_CREATE: &str = "/qcm/create";
const PATH_GENERATE: &str = "/qcm/generate";
const PATH_QUICK: &str = "/qcm/quick";

/// QCM list page — shows every saved set.
pub fn QcmListPage() -> Element {
    let fetch = make_fetch();
    let delete = make_delete();
    rsx! {
        GameListPage::<QcmSet> {
            title: TITLE,
            game_label: LABEL,
            generate_path: PATH_GENERATE,
            play_path_prefix: PATH_PLAY,
            create_path: PATH_CREATE,
            quick_path: PATH_QUICK,
            fetch_sets: fetch,
            delete_fn: delete,
        }
    }
}

/// Build the async fetch closure for the list page.
fn make_fetch() -> FetchSets<QcmSet> {
    FetchSets(Arc::new(|| {
        Box::pin(api::get_qcm_sets())
    }))
}

/// Build the async delete-by-id closure.
fn make_delete() -> DeleteFn {
    DeleteFn(Arc::new(|id| {
        Box::pin(api::delete_qcm_set_owned(id))
    }))
}

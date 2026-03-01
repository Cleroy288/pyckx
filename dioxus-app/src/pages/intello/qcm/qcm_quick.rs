//! Quick QCM: upload PDF, play immediately

use super::quick_form::QuickQcmForm;
use crate::components::intello::qcm::qcm_player::QcmPlayer;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{
    Button, ButtonVariant,
};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::domain::qcm_types::QcmSet;
use crate::state::auth::use_auth_guard;
use dioxus::prelude::*;

/// Quick QCM page — form then player
pub fn QcmQuickPage() -> Element {
    let _auth = use_auth_guard();
    let nav = navigator();
    let mut qcm_set: Signal<Option<QcmSet>> =
        use_signal(|| None);

    let on_back = move |_| {
        qcm_set.set(None);
    };
    let on_result = move |set: QcmSet| {
        qcm_set.set(Some(set));
    };

    let is_playing = (qcm_set)().is_some();

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "Quick QCM",
                Button {
                    text: "Back".to_string(),
                    variant: ButtonVariant::Outline,
                    on_click: move |_| {
                        nav.push("/intello/qcm");
                    },
                }
            }
            if let Some(set) = (qcm_set)() {
                QcmPlayer {
                    set: set,
                    on_back: on_back,
                }
            }
            div {
                style: if is_playing { "display:none" }
                    else { "display:block" },
                QuickQcmForm {
                    on_result: on_result,
                }
            }
        }
    }
}

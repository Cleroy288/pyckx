//! QCM manual creation page

use crate::components::qcm::ManualQcmForm;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use dioxus::prelude::*;

/// QCM manual creation page
pub fn QcmCreatePage() -> Element {
    let nav = navigator();
    let on_created = move |_: ()| {
        nav.push("/qcm");
    };

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "Create QCM Set",
                span {}
            }
            ManualQcmForm {
                on_back: on_created,
            }
        }
    }
}

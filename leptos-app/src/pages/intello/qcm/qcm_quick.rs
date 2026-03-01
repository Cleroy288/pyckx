// ** qcm_quick.rs **
// ==> Quick QCM: upload PDF, play immediately

use super::quick_form::QuickQcmForm;
use crate::components::intello::qcm::qcm_player::QcmPlayer;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{
    nav_click, Button, ButtonVariant,
};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::domain::qcm_types::QcmSet;
use crate::state::auth::use_auth_guard;
use leptos::prelude::*;

/// Quick QCM page — form then player
#[component]
pub fn QcmQuickPage() -> impl IntoView {
    let _auth = use_auth_guard();
    let qcm_set: RwSignal<Option<QcmSet>> =
        RwSignal::new(None);

    let on_back = Callback::new(move |_| {
        qcm_set.set(None);
    });
    let on_result = Callback::new(move |set: QcmSet| {
        qcm_set.set(Some(set));
    });

    let is_playing =
        move || qcm_set.get().is_some();

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="Quick QCM">
                <Button
                    text="Back"
                    variant=ButtonVariant::Outline
                    on_click=nav_click("/intello/qcm")
                />
            </HeroBanner>
            {move || qcm_set.get().map(|set| view! {
                <QcmPlayer set=set on_back=on_back />
            })}
            <div style:display=move || {
                if is_playing() { "none" } else { "block" }
            }>
                <QuickQcmForm on_result=on_result />
            </div>
        </PageLayout>
    }
}

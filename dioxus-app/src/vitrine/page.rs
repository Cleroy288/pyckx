//! `VitrinePage` — public landing page composition.
//!
//! One glance: nav, hero, animated stage, apps, pricing.
//! No business logic lives here — only layout.

use dioxus::prelude::*;

use super::apps::AppsSection;
use super::hero::HeroSection;
use super::nav::VitrineNav;
use super::pricing::PricingSection;
use super::stage::AnimationStage;
use crate::state::hooks::use_redirect_if_authenticated;

/// Public landing page. Redirects signed-in users home.
pub fn VitrinePage() -> Element {
    use_redirect_if_authenticated();
    rsx! {
        div { class: "vitrine-page",
            VitrineNav {}
            HeroSection {}
            AnimationStage {}
            AppsSection {}
            PricingSection {}
        }
    }
}

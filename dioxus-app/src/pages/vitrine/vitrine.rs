//! VitrinePage — public landing page composition

use dioxus::prelude::*;

use super::apps::AppsSection;
use super::hero::HeroSection;
use super::nav::VitrineNav;
use super::pricing::PricingSection;
use super::stage::AnimationStage;
use crate::state::hooks::use_redirect_if_authenticated;

/// Vitrine page — public-facing landing
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

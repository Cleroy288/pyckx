use leptos::prelude::*;
use leptos_router::components::A;

use super::data::{APP_CARDS, PRICING_TIERS};
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::hero_banner::HeroBanner;
use crate::state::hooks::use_redirect_if_authenticated;

stylance::import_crate_style!(vitrine_style, "src/pages/vitrine/vitrine.module.css");

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <div class=vitrine_style::hero_section>
            <HeroBanner title="Pyckx">
                <p class=vitrine_style::hero_tagline>
                    "All-in-one platform. One subscription. All features."
                </p>
                <div class=vitrine_style::hero_cta>
                    <A href="/login">
                        <Button text="Get Started".to_string() />
                    </A>
                </div>
            </HeroBanner>
        </div>
    }
}

#[component]
fn TaglineSection() -> impl IntoView {
    view! {
        <section class=vitrine_style::tagline>
            <div class=vitrine_style::tagline_text>
                <p class=vitrine_style::tagline_line>
                    "Multiple apps in one"
                </p>
                <p class=vitrine_style::tagline_line>
                    "One abonnement"
                </p>
            </div>
        </section>
    }
}

#[component]
fn AppsSection() -> impl IntoView {
    view! {
        <section id="services" class=vitrine_style::apps>
            <HeroBanner title="Our Apps">
                <p class=vitrine_style::hero_tagline>
                    "Everything you need, all in one place"
                </p>
            </HeroBanner>
            <div class=vitrine_style::apps_grid>
                {APP_CARDS.iter().map(|app| {
                    let class = format!(
                        "{} {}",
                        vitrine_style::vitrine_card,
                        vitrine_style::app_card,
                    );
                    view! {
                        <div class=class>
                            <div class=vitrine_style::app_card_icon>
                                <crate::components::ui::icon::Icon icon_name=app.icon.to_string() />
                            </div>
                            <h3 class=vitrine_style::card_title>{app.title}</h3>
                            <p class=vitrine_style::card_content>{app.description}</p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
fn PricingSection() -> impl IntoView {
    view! {
        <section id="pricing" class=vitrine_style::pricing>
            <HeroBanner title="Pricing">
                <p class=vitrine_style::hero_tagline>
                    "Simple, transparent pricing. One subscription for everything."
                </p>
            </HeroBanner>
            <div class=vitrine_style::pricing_grid>
                {PRICING_TIERS.iter().map(|tier| {
                    view! {
                        <div class=vitrine_style::vitrine_card>
                            <div class=vitrine_style::pricing_card_content>
                                <h3 class=vitrine_style::pricing_name>{tier.name}</h3>
                                <div class=vitrine_style::pricing_price>
                                    <span class=vitrine_style::pricing_amount>{tier.price} "€"</span>
                                    <Show when=move || !tier.period.is_empty() fallback=|| ()>
                                        <span class=vitrine_style::pricing_period>"/" {tier.period}</span>
                                    </Show>
                                </div>
                                <ul class=vitrine_style::pricing_features>
                                    {tier.features.iter().map(|&feature| {
                                        view! {
                                            <li>{feature}</li>
                                        }
                                    }).collect_view()}
                                </ul>
                                <div class=vitrine_style::pricing_button>
                                    <A href="/login">
                                        <Button text="Get Started".to_string() />
                                    </A>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
fn StartNowSection() -> impl IntoView {
    view! {
        <section id="start-now" class=vitrine_style::start_now>
            <HeroBanner title="Ready to Get Started?">
                <p class=vitrine_style::hero_tagline>
                    "Join thousands of users who trust Pyckx for their daily needs.
                    Create your account in seconds and start using all our features today."
                </p>
                <div class=vitrine_style::start_now_cta>
                    <A href="/login">
                        <Button text="Create Account".to_string() />
                    </A>
                    <p class=vitrine_style::start_now_note>
                        "By signing up, you agree to our "<a href="/terms">"Terms of Service"</a>" and "<a href="/privacy">"Privacy Policy"</a>"."
                    </p>
                </div>
            </HeroBanner>
        </section>
    }
}

#[component]
pub fn Vitrine() -> impl IntoView {
    use_redirect_if_authenticated();

    view! {
        <>
            <VitrineTopBar />
            <main class=vitrine_style::vitrine>
                <div class=vitrine_style::content>
                    <HeroSection />
                    <TaglineSection />
                    <AppsSection />
                    <PricingSection />
                    <StartNowSection />
                </div>
            </main>
        </>
    }
}

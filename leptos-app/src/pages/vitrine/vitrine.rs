use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use super::components::SectionHeader;
use super::data::{APP_CARDS, PLATFORM_CARDS, PRICING_TIERS};
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardVariant};
use crate::state::use_auth;

stylance::import_crate_style!(vitrine_style, "src/pages/vitrine/vitrine.module.css");

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section id="hero" class=vitrine_style::hero>
            <div class=vitrine_style::hero_gradient></div>
            <div class=vitrine_style::hero_content>
                <h1 class=vitrine_style::hero_title>
                    <span class=vitrine_style::title_highlight>"Pyckx"</span>
                </h1>
                <p class=vitrine_style::hero_tagline>
                    "All-in-one platform. One subscription. All features."
                </p>
                <div class=vitrine_style::hero_cta>
                    <A href="/login">
                        <Button text="Get Started".to_string() icon="ArrowRight" />
                    </A>
                </div>
            </div>
        </section>
    }
}

#[component]
fn PlatformSection() -> impl IntoView {
    view! {
        <section class=vitrine_style::platform>
            <div class=vitrine_style::platform_content>
                {PLATFORM_CARDS.iter().map(|card| {
                    view! {
                        <Card variant=CardVariant::Glass>
                            <div class=vitrine_style::card_inner>
                                <div class=vitrine_style::card_icon>
                                    <crate::components::ui::icon::Icon icon_name=card.icon.to_string() />
                                </div>
                                <h3 class=vitrine_style::card_title>{card.title}</h3>
                                <p class=vitrine_style::card_content>{card.description}</p>
                            </div>
                        </Card>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
fn AppsSection() -> impl IntoView {
    view! {
        <section id="services" class=vitrine_style::apps>
            <SectionHeader
                title="Our Apps".to_string()
                subtitle="Everything you need, all in one place".to_string()
            />
            <div class=vitrine_style::apps_grid>
                {APP_CARDS.iter().map(|app| {
                    view! {
                        <Card variant=CardVariant::Glass class=vitrine_style::app_card.to_string()>
                            <div class=vitrine_style::card_inner>
                                <div class=vitrine_style::app_card_icon>
                                    <crate::components::ui::icon::Icon icon_name=app.icon.to_string() />
                                </div>
                                <h3 class=vitrine_style::card_title>{app.title}</h3>
                                <p class=vitrine_style::card_content>{app.description}</p>
                            </div>
                        </Card>
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
            <SectionHeader
                title="Pricing".to_string()
                subtitle="Simple, transparent pricing. One subscription for everything.".to_string()
            />
            <div class=vitrine_style::pricing_grid>
                {PRICING_TIERS.iter().map(|tier| {
                    let variant = if tier.popular { CardVariant::Popular } else { CardVariant::Glass };
                    view! {
                        <Card variant=variant>
                            <div class=vitrine_style::pricing_card_content>
                                <Show when=move || tier.popular fallback=|| view! {}>
                                    <div class=vitrine_style::pricing_badge>"Popular"</div>
                                </Show>
                                <h3 class=vitrine_style::pricing_name>{tier.name}</h3>
                                <div class=vitrine_style::pricing_price>
                                    <span class=vitrine_style::pricing_amount>{tier.price} "€"</span>
                                    <Show when=move || !tier.period.is_empty() fallback=|| view! {}>
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
                                        <Button text="Get Started".to_string() icon="ArrowRight" />
                                    </A>
                                </div>
                            </div>
                        </Card>
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
            <div class=vitrine_style::start_now_content>
                <h2 class=vitrine_style::start_now_title>"Ready to Get Started?"</h2>
                <p class=vitrine_style::start_now_text>
                    "Join thousands of users who trust Pyckx for their daily needs.
                    Create your account in seconds and start using all our features today."
                </p>
                <div class=vitrine_style::start_now_form>
                    <A href="/login">
                        <Button text="Create Account".to_string() icon="ArrowRight" />
                    </A>
                </div>
                <p class=vitrine_style::start_now_note>
                    "By signing up, you agree to our "<a href="/terms">"Terms of Service"</a>" and "<a href="/privacy">"Privacy Policy"</a>"."
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn Vitrine() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    
    // Redirect authenticated users to home page
    // Wait until session check is complete before redirecting
    Effect::new(move |_| {
        let is_checking = auth.is_checking_session.get();
        let has_user = auth.user.get().is_some();
        
        // Only redirect after session check is complete and user is authenticated
        if !is_checking && has_user {
            navigate("/home", Default::default());
        }
    });

    view! {
        <>
            <VitrineTopBar />
            <main class=vitrine_style::vitrine>
                <HeroSection />
                <PlatformSection />
                <AppsSection />
                <PricingSection />
                <StartNowSection />
            </main>
        </>
    }
}

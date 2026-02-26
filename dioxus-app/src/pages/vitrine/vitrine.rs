//! Vitrine (landing) page — public-facing sections

use dioxus::prelude::*;

use super::data::{APP_CARDS, PRICING_TIERS};
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::icon::Icon;
use crate::state::hooks::use_redirect_if_authenticated;

/// Hero section with CTA
fn HeroSection() -> Element {
    rsx! {
        div {
            class: "w-full animate-[fadeInUp_0.6s_ease_both]",
            HeroBanner {
                title: "Pyckx",
                p {
                    class: "text-[clamp(1.25rem,2.5vw,1.75rem)] \
                        text-[var(--color-text-secondary)] \
                        mb-12 leading-relaxed font-normal \
                        tracking-tight",
                    "All-in-one platform. One subscription. \
                        All features."
                }
                div {
                    class: "flex gap-4 justify-center \
                        flex-wrap items-center",
                    Link {
                        to: "/login",
                        Button {
                            text: "Get Started".to_string(),
                        }
                    }
                }
            }
        }
    }
}

/// Tagline section with border-left accent
fn TaglineSection() -> Element {
    rsx! {
        section {
            class: "w-full py-12 pl-[15%] mb-8 \
                animate-[fadeInUp_0.6s_ease_both] \
                [animation-delay:0.15s]",
            div {
                class: "border-l-2 \
                    border-[var(--color-primary)] \
                    pl-6",
                p {
                    class: "text-[clamp(1.1rem,2vw,1.4rem)] \
                        text-[var(--color-text-secondary)] \
                        m-0 leading-[1.8] font-normal",
                    "Multiple apps in one"
                }
                p {
                    class: "text-[clamp(1.1rem,2vw,1.4rem)] \
                        text-[var(--color-text-secondary)] \
                        m-0 leading-[1.8] font-normal",
                    "One abonnement"
                }
            }
        }
    }
}

/// Apps section — grid of app cards
fn AppsSection() -> Element {
    rsx! {
        section {
            id: "services",
            class: "w-full mb-16 \
                animate-[fadeInUp_0.6s_ease_both] \
                [animation-delay:0.25s]",
            HeroBanner {
                title: "Our Apps",
                p {
                    class: "text-[clamp(1.25rem,2.5vw,1.75rem)] \
                        text-[var(--color-text-secondary)] \
                        mb-12 leading-relaxed",
                    "Everything you need, all in one place"
                }
            }
            div {
                class: "grid grid-cols-2 \
                    max-md:grid-cols-1 gap-0",
                for app in APP_CARDS.iter() {
                    div {
                        class: "border-spin-card \
                            relative bg-[var(--glass-bg)] \
                            backdrop-blur-[20px] \
                            border border-[var(--color-border)] \
                            p-10 overflow-hidden \
                            flex flex-col items-center \
                            text-center",
                        div {
                            class: "w-14 h-14 flex items-center \
                                justify-center \
                                bg-[color-mix(in_srgb,var(--primary)_8%,transparent)] \
                                border border-[color-mix(in_srgb,var(--primary)_15%,transparent)] \
                                text-[var(--color-primary)] \
                                mb-6 transition-colors \
                                duration-[var(--transition-fast)]",
                            Icon {
                                icon_name: app.icon.to_string(),
                            }
                        }
                        h3 {
                            class: "text-2xl font-bold \
                                text-[var(--color-text-primary)] \
                                mb-4 tracking-tight",
                            "{app.title}"
                        }
                        p {
                            class: "text-base \
                                text-[var(--color-text-secondary)] \
                                leading-relaxed m-0",
                            "{app.description}"
                        }
                    }
                }
            }
        }
    }
}

/// Pricing section — grid of pricing tiers
fn PricingSection() -> Element {
    rsx! {
        section {
            id: "pricing",
            class: "w-full mb-8 \
                animate-[fadeInUp_0.6s_ease_both] \
                [animation-delay:0.35s]",
            HeroBanner {
                title: "Pricing",
                p {
                    class: "text-[clamp(1.25rem,2.5vw,1.75rem)] \
                        text-[var(--color-text-secondary)] \
                        mb-12 leading-relaxed",
                    "Simple, transparent pricing. \
                        One subscription for everything."
                }
            }
            div {
                class: "grid grid-cols-2 \
                    max-md:grid-cols-1 gap-0 \
                    items-stretch",
                for tier in PRICING_TIERS.iter() {
                    div {
                        class: "border-spin-card \
                            relative bg-[var(--glass-bg)] \
                            backdrop-blur-[20px] \
                            border border-[var(--color-border)] \
                            p-10 overflow-hidden",
                        div {
                            class: "flex flex-col text-center \
                                min-h-[350px]",
                            h3 {
                                class: "text-2xl font-bold \
                                    text-[var(--color-text-primary)] \
                                    mb-4 mt-2",
                                "{tier.name}"
                            }
                            div {
                                class: "mb-8 flex items-baseline \
                                    justify-center gap-1 \
                                    flex-wrap",
                                span {
                                    class: "text-[2.5rem] font-extrabold \
                                        text-[var(--color-primary)] \
                                        leading-none",
                                    "{tier.price}"
                                    "EUR"
                                }
                                if !tier.period.is_empty() {
                                    span {
                                        class: "text-base \
                                            text-[var(--color-text-secondary)]",
                                        "/{tier.period}"
                                    }
                                }
                            }
                            ul {
                                class: "list-none m-0 mb-8 p-0 \
                                    text-left flex-1",
                                for feature in tier.features.iter() {
                                    li {
                                        class: "py-3 \
                                            text-[var(--color-text-secondary)] \
                                            border-b \
                                            border-[color-mix(in_srgb,var(--primary)_5%,transparent)] \
                                            last:border-b-0 \
                                            text-[0.95rem] \
                                            leading-snug",
                                        "{feature}"
                                    }
                                }
                            }
                            div {
                                class: "mt-auto w-full",
                                Link {
                                    to: "/login",
                                    Button {
                                        text: "Get Started".to_string(),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Start Now section with CTA
fn StartNowSection() -> Element {
    rsx! {
        section {
            id: "start-now",
            class: "border-spin-always \
                w-full mt-8",
            HeroBanner {
                title: "Ready to Get Started?",
                p {
                    class: "text-[clamp(1.25rem,2.5vw,1.75rem)] \
                        text-[var(--color-text-secondary)] \
                        mb-12 leading-relaxed",
                    "Join thousands of users who trust \
                        Pyckx for their daily needs. \
                        Create your account in seconds \
                        and start using all our features \
                        today."
                }
                div {
                    class: "flex flex-col items-center \
                        gap-4 py-8",
                    Link {
                        to: "/login",
                        Button {
                            text: "Create Account".to_string(),
                        }
                    }
                    p {
                        class: "text-sm \
                            text-[var(--color-text-secondary)] \
                            m-0",
                        "By signing up, you agree to our "
                        a { href: "/terms", "Terms of Service" }
                        " and "
                        a { href: "/privacy", "Privacy Policy" }
                        "."
                    }
                }
            }
        }
    }
}

/// Vitrine page — public landing
pub fn VitrinePage() -> Element {
    use_redirect_if_authenticated();

    rsx! {
        VitrineTopBar {}
        main {
            class: "page-layout \
                relative w-full min-h-screen \
                flex flex-col items-center \
                pt-20 px-8 pb-16 z-[1] \
                bg-[var(--color-background)]",
            div {
                class: "relative flex-1 w-full \
                    max-w-[1200px] mx-auto",
                HeroSection {}
                TaglineSection {}
                AppsSection {}
                PricingSection {}
                StartNowSection {}
            }
        }
    }
}

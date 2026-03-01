//! App Card Component - Clickable card for dashboard apps

use crate::components::ui::card::{Card, CardVariant};
use crate::components::ui::icon::Icon;
use leptos::prelude::*;
use leptos_router::components::A;

stylance::import_crate_style!(style, "src/components/dashboard/dashboard.module.css");

#[component]
pub fn AppCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] icon: String,
    #[prop(into)] href: String,
) -> impl IntoView {
    view! {
        <A href=href attr:class=style::app_link>
            <Card variant=CardVariant::Glass class=style::app_card.to_string()>
                <div class=style::app_icon>
                    <Icon icon_name=icon />
                </div>
                <h3 class=style::app_title>{title}</h3>
                <p class=style::app_description>{description}</p>
            </Card>
        </A>
    }
}

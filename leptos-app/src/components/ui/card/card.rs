// ** card.rs **
// ==> Reusable Card Component with variants

use leptos::prelude::*;

// Import scoped styles
stylance::import_crate_style!(style, "src/components/ui/card/card.module.css");

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CardVariant {
    #[default]
    Glass,
    Solid,
    Popular,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(optional, default = CardVariant::Glass)] variant: CardVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant_class = match variant {
        CardVariant::Glass => style::glass,
        CardVariant::Solid => style::solid,
        CardVariant::Popular => style::popular,
    };

    view! {
        <div class=format!("{} {} {}", style::card, variant_class, class.unwrap_or_default())>
            {children()}
        </div>
    }
}

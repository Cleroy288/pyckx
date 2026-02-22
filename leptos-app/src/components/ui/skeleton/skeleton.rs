// ** skeleton.rs **
// ==> Pulsing placeholder for loading states

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/skeleton/skeleton.module.css"
);

/// Generic pulsing skeleton placeholder
#[component]
pub fn Skeleton(
    #[prop(optional, into)]
    width: Option<String>,
    #[prop(optional, into)]
    height: Option<String>,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let w = width.unwrap_or_else(|| "100%".to_string());
    let h = height.unwrap_or_else(|| "1rem".to_string());

    view! {
        <div
            class=format!(
                "{} {}",
                style::skeleton,
                class.unwrap_or_default()
            )
            style=format!("width:{};height:{}", w, h)
        />
    }
}

/// Card-shaped skeleton placeholder
#[component]
pub fn SkeletonCard(
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=format!(
            "{} {}",
            style::skeleton_card,
            class.unwrap_or_default()
        )>
            <Skeleton height="1.25rem".to_string() width="60%".to_string() />
            <Skeleton height="0.875rem".to_string() />
            <Skeleton height="0.875rem".to_string() width="80%".to_string() />
        </div>
    }
}

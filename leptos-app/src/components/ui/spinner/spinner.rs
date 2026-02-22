// ** spinner.rs **
// ==> CSS-animated loading spinner, 3 sizes

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/spinner/spinner.module.css"
);

/// Spinner size variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Loading spinner component
#[component]
pub fn Spinner(
    #[prop(optional, default = SpinnerSize::Medium)]
    size: SpinnerSize,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let size_class = match size {
        SpinnerSize::Small => style::small,
        SpinnerSize::Medium => style::medium,
        SpinnerSize::Large => style::large,
    };

    view! {
        <div class=format!(
            "{} {} {}",
            style::spinner,
            size_class,
            class.unwrap_or_default()
        )>
            <div class=style::ring></div>
        </div>
    }
}

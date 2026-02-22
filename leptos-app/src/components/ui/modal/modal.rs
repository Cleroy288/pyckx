// ** modal.rs **
// ==> Overlay dialog component with backdrop

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/modal/modal.module.css"
);

/// Helper to create a modal close callback
pub fn on_close<F>(f: F) -> Callback<()>
where
    F: Fn() + Send + Sync + 'static,
{
    Callback::new(move |_| f())
}

/// Overlay dialog component
#[component]
pub fn Modal(
    /// Whether the modal is visible
    #[prop(into)]
    open: Signal<bool>,
    /// Called when backdrop or X is clicked
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Optional title text
    #[prop(optional, into)]
    title: Option<String>,
    /// Modal body content
    children: Children,
) -> impl IntoView {
    let title_text = title;

    view! {
        <div
            class=style::backdrop
            style:display=move || {
                if open.get() { "flex" } else { "none" }
            }
            on:click=move |_| {
                if let Some(cb) = &on_close {
                    cb.run(());
                }
            }
        >
            <div
                class=style::modal
                on:click=|ev| ev.stop_propagation()
            >
                <div class=style::header>
                    {title_text.map(|t| {
                        view! {
                            <h2 class=style::title>
                                {t}
                            </h2>
                        }
                    })}
                    <button
                        class=style::close_btn
                        on:click=move |_| {
                            if let Some(cb) =
                                &on_close
                            {
                                cb.run(());
                            }
                        }
                    >
                        <crate::components::ui
                            ::icon::Icon
                            icon_name="X"
                                .to_string()
                        />
                    </button>
                </div>
                <div class=style::body>
                    {children()}
                </div>
            </div>
        </div>
    }
}

// ** resource_list.rs **
// ==> Displays course resources with upload button

use crate::components::ui::button::{
    btn_click, Button, ButtonSize,
};
use crate::components::ui::empty_state::EmptyState;
use crate::domain::course_types::ResourceData;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/course/resource_list.module.css"
);

/// Resource list with upload action
#[component]
pub fn ResourceList(
    /// List of resources
    #[prop(into)]
    resources: Signal<Vec<ResourceData>>,
    /// Called when upload button clicked
    #[prop(into)]
    on_upload: Callback<()>,
) -> impl IntoView {
    let empty = Signal::derive(move || {
        resources.get().is_empty()
    });

    view! {
        <div class=style::container>
            <div class=style::header>
                <h3 class=style::title>"Resources"</h3>
                <Button
                    text="Upload"
                    size=ButtonSize::Small
                    on_click=btn_click(move |_| {
                        on_upload.run(())
                    })
                />
            </div>
            <Show
                when=move || !empty.get()
                fallback=|| view! {
                    <EmptyState
                        icon="Book"
                        message="No resources yet"
                    />
                }
            >
                <div class=style::list>
                    <For
                        each=move || resources.get()
                        key=|r| r.id.clone()
                        let:res
                    >
                        <div class=style::item>
                            <div class=style::item_info>
                                <span class=style::item_name>
                                    {res.name.clone()}
                                </span>
                                <span class=style::item_type>
                                    {res.content_type.clone()}
                                </span>
                            </div>
                        </div>
                    </For>
                </div>
            </Show>
        </div>
    }
}

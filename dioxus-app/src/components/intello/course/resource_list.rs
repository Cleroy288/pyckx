// ** resource_list.rs **
// ==> Displays course resources with upload button

use crate::components::ui::button::{
    Button, ButtonSize,
};
use crate::components::ui::empty_state::EmptyState;
use crate::domain::course_types::ResourceData;
use dioxus::prelude::*;

/// Resource list with upload action
#[component]
pub fn ResourceList(
    /// List of resources
    resources: Signal<Vec<ResourceData>>,
    /// Called when upload button clicked
    on_upload: EventHandler<()>,
) -> Element {
    let empty = use_memo(move || {
        (resources)().is_empty()
    });

    rsx! {
        div { class: "flex flex-col gap-4",
            div {
                class: "flex justify-between \
                    items-center",
                h3 {
                    class: "text-base font-semibold \
                        text-[var(--color-text-primary)] \
                        m-0",
                    "Resources"
                }
                Button {
                    text: "Upload",
                    size: ButtonSize::Small,
                    on_click: move |_| {
                        on_upload.call(())
                    },
                }
            }
            if (empty)() {
                EmptyState {
                    icon: "Book",
                    message: "No resources yet",
                }
            } else {
                div { class: "flex flex-col gap-2",
                    for res in (resources)().iter() {
                        div {
                            class: "flex \
                                justify-between \
                                items-center \
                                py-3 px-4 \
                                border \
                                border-[var(--color-border)] \
                                bg-[var(--glass-bg)] \
                                backdrop-blur-[20px]",
                            div {
                                class: "flex flex-col \
                                    gap-1",
                                span {
                                    class: "text-[0.9rem] \
                                        text-[var(--color-text-primary)]",
                                    "{res.name}"
                                }
                                span {
                                    class: "text-xs \
                                        text-[var(--color-text-secondary)]",
                                    "{res.content_type}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

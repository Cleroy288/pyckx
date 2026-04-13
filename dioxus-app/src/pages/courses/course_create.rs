//! Create a new course form

use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::input::Input;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::components::ui::textarea::Textarea;
use crate::domain::course_types::CreateCourseRequest;
use crate::state::hooks::use_async_navigate;
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Create course page
pub fn CourseCreatePage() -> Element {
    let _auth = use_auth_guard();
    let mut nav_to = use_async_navigate();

    let mut name = use_signal(String::new);
    let mut desc = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let valid =
        use_memo(move || !(name)().trim().is_empty());

    let handle = move |_| {
        loading.set(true);
        spawn(async move {
            let req = CreateCourseRequest {
                name: (name)(),
                description: (desc)(),
            };
            match api::courses::create_course(&req)
                .await
            {
                Ok(course) => {
                    nav_to.set(Some(format!(
                        "/courses/{}",
                        course.id
                    )));
                }
                Err(e) => {
                    crate::api::log::log_error(
                        &format!(
                            "Create course failed: {e}"
                        ),
                        "course_create",
                    );
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "New Course",
                span {}
            }
            div {
                class: "flex flex-col gap-4 max-w-lg",
                Input {
                    input_type: "text".to_string(),
                    id: "course-name".to_string(),
                    label: "Name".to_string(),
                    placeholder: "Course name".to_string(),
                    value: name,
                    on_input: move |v: String| {
                        name.set(v);
                    },
                    required: true,
                }
                Textarea {
                    id: "course-desc".to_string(),
                    label: "Description".to_string(),
                    placeholder: "Course description..."
                        .to_string(),
                    value: desc,
                    on_input: move |v: String| {
                        desc.set(v);
                    },
                }
                if (loading)() {
                    Spinner {}
                } else {
                    Button {
                        text: "Create".to_string(),
                        disabled: Signal::new(
                            !(valid)()
                        ),
                        on_click: handle,
                    }
                }
            }
        }
    }
}

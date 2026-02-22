// ** course_create.rs **
// ==> Create a new course form

use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{btn_click, Button};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::input::Input;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::components::ui::textarea::Textarea;
use crate::components::ui::toast::use_toast;
use crate::domain::course_types::CreateCourseRequest;
use crate::state::hooks::use_async_navigate;
use crate::state::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;

stylance::import_crate_style!(
    style,
    "src/pages/intello/courses/course_create.module.css"
);

/// Create course page
#[component]
pub fn CourseCreatePage() -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();
    let nav_to = use_async_navigate();

    let name = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let loading = RwSignal::new(false);

    let valid = Signal::derive(move || {
        !name.get().trim().is_empty()
    });

    let handle = move |_| {
        loading.set(true);
        spawn_local(async move {
            let req = CreateCourseRequest {
                name: name.get(),
                description: desc.get(),
            };
            match api::courses::create_course(&req)
                .await
            {
                Ok(course) => {
                    toast.success(
                        "Course created".into(),
                    );
                    nav_to.set(Some(format!(
                        "/intello/courses/{}",
                        course.id
                    )));
                }
                Err(e) => {
                    toast.error(e);
                    loading.set(false);
                }
            }
        });
    };

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="New Course">
                <span />
            </HeroBanner>
            <div class=style::form>
                <Input
                    type_="text"
                    id="course-name"
                    label="Name".to_string()
                    placeholder="Course name"
                        .to_string()
                    value=name
                    on_input=Callback::new(
                        move |v| name.set(v),
                    )
                    required=true
                />
                <Textarea
                    id="course-desc"
                    label="Description".to_string()
                    placeholder="Course description..."
                        .to_string()
                    value=desc
                    on_input=Callback::new(
                        move |v| desc.set(v),
                    )
                />
                <Show
                    when=move || loading.get()
                    fallback=move || {
                        view! {
                            <Button
                                text="Create"
                                disabled=Signal::derive(
                                    move || !valid.get(),
                                )
                                on_click=btn_click(
                                    handle,
                                )
                            />
                        }
                    }
                >
                    <Spinner />
                </Show>
            </div>
        </PageLayout>
    }
}

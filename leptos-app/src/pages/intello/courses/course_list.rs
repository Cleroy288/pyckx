// ** course_list.rs **
// ==> Lists all courses with create/delete actions

use crate::api;
use crate::components::intello::course::CourseCard;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{
    nav_click, Button,
};
use crate::components::ui::empty_state::EmptyState;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::loading_boundary::LoadingBoundary;
use crate::components::ui::toast::use_toast;
use crate::domain::course_types::CourseData;
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::state::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;

stylance::import_crate_style!(
    style,
    "src/pages/intello/courses/course_list.module.css"
);

/// Course list page
#[component]
pub fn CourseListPage() -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();

    let courses: RwSignal<Vec<CourseData>> =
        RwSignal::new(Vec::new());
    let loading = RwSignal::new(true);

    let fetch: FetchFn<Vec<CourseData>> =
        Box::new(|| {
            Box::pin(api::courses::list_courses())
        });
    use_fetch_on_mount(
        courses, loading, toast, fetch,
    );

    let on_delete = move |id: String| {
        handle_course_delete(id, courses, toast);
    };

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="Courses">
                <div class=style::actions>
                    <Button
                        text="New Course"
                        on_click=nav_click(
                            "/intello/courses/create",
                        )
                    />
                </div>
            </HeroBanner>
            <LoadingBoundary loading=loading>
                <Show
                    when=move || {
                        courses
                            .with(|v| !v.is_empty())
                    }
                    fallback=|| {
                        view! {
                            <EmptyState
                                icon="Book"
                                message="No courses yet"
                            />
                        }
                    }
                >
                    <div class=style::grid>
                        <For
                            each=move || {
                                courses
                                    .with(Vec::clone)
                            }
                            key=|c| c.id.clone()
                            let:course
                        >
                            {
                                let id =
                                    course.id.clone();
                                let del_id =
                                    id.clone();
                                let path = format!(
                                    "/intello/courses/{}",
                                    id,
                                );
                                view! {
                                    <CourseCard
                                        name=course
                                            .name
                                            .clone()
                                        description=course
                                            .description
                                            .clone()
                                        created_at=course
                                            .created_at
                                            .clone()
                                        on_view=crate::components::ui::button::nav_callback(path)
                                        on_delete=Callback::new({
                                            let del_id = del_id.clone();
                                            move |_| on_delete(del_id.clone())
                                        })
                                    />
                                }
                            }
                        </For>
                    </div>
                </Show>
            </LoadingBoundary>
        </PageLayout>
    }
}

/// Execute course deletion and update state
fn handle_course_delete(
    id: String,
    courses: RwSignal<Vec<CourseData>>,
    toast: crate::components::ui::toast::ToastState,
) {
    spawn_local(async move {
        let result =
            api::courses::delete_course(&id).await;
        match result {
            Ok(true) => {
                courses.update(
                    |v| v.retain(|c| c.id != id),
                );
                toast.success(
                    "Course deleted".into(),
                );
            }
            Ok(false) => toast.error(
                "Course not found".into(),
            ),
            Err(e) => toast.error(e),
        }
    });
}

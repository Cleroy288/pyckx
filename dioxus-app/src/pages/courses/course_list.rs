//! Lists all courses with create/delete actions

use crate::api;
use crate::components::course::CourseCard;
use crate::home::HomeTopBar;
use crate::ui::Button;
use crate::ui::CardGrid;
use crate::ui::EmptyState;
use crate::ui::HeroBanner;
use crate::ui::LoadingBoundary;
use crate::ui::PageLayout;
use crate::domain::course_types::CourseData;
use crate::state::hooks::{
    use_fetch_on_mount, FetchFn,
};
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Course list page
pub fn CourseListPage() -> Element {
    let _auth = use_auth_guard();
    let nav = navigator();

    let mut courses: Signal<Vec<CourseData>> =
        use_signal(Vec::new);
    let loading = use_signal(|| true);

    let fetch: FetchFn<Vec<CourseData>> =
        Box::new(|| {
            Box::pin(api::courses::list_courses())
        });
    use_fetch_on_mount(courses, loading, fetch);

    let on_delete = move |id: String| {
        spawn(async move {
            if let Ok(true) =
                api::courses::delete_course(&id).await
            {
                courses
                    .write()
                    .retain(|c| c.id != id);
            }
        });
    };

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner {
                    title: "Courses",
                    div {
                        class: "absolute bottom-3 \
                            left-4 z-[1] flex gap-2",
                        Button {
                            text: "New Course",
                            on_click: move |_| {
                                nav.push(
                                    "/courses/create",
                                );
                            },
                        }
                    }
                }
                LoadingBoundary {
                    loading: loading,
                    if (courses)().is_empty() {
                        EmptyState {
                            icon: "Book",
                            message: "No courses yet",
                        }
                    } else {
                        CardGrid {
                            for course in
                                (courses)().iter()
                            {
                                {
                                    let id =
                                        course.id.clone();
                                    let del_id =
                                        id.clone();
                                    let path = format!(
                                        "/courses/{id}",
                                    );
                                    rsx! {
                                        CourseCard {
                                            key: "{id}",
                                            name: course
                                                .name
                                                .clone(),
                                            description:
                                                course
                                                .description
                                                .clone(),
                                            created_at:
                                                course
                                                .created_at
                                                .clone(),
                                            on_view:
                                                move |_| {
                                                nav.push(
                                                    &*path
                                                );
                                            },
                                            on_delete: {
                                                let d =
                                                    del_id
                                                    .clone();
                                                move |_| {
                                                    on_delete(
                                                        d.clone()
                                                    )
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

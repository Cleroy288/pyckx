//! Create a study session with topic and resources

use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::input::Input;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::components::ui::textarea::Textarea;
use crate::domain::course_types::CreateSessionRequest;
use crate::state::hooks::use_async_navigate;
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Create session page — receives course_id from route
pub fn SessionCreatePage(
    course_id: String,
) -> Element {
    let _auth = use_auth_guard();
    let mut nav_to = use_async_navigate();

    let mut topic = use_signal(String::new);
    let mut keywords = use_signal(String::new);
    let mut instr = use_signal(String::new);
    let mut lang = use_signal(|| "en".to_string());
    let mut loading = use_signal(|| false);

    let valid = use_memo(move || {
        !(topic)().trim().is_empty()
    });

    let cid = course_id.clone();
    let handle = move |_| {
        loading.set(true);
        let cid = cid.clone();
        spawn(async move {
            let kw: Vec<String> = (keywords)()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let req = CreateSessionRequest {
                topic: (topic)(),
                keywords: kw,
                instructions: (instr)(),
                language: (lang)(),
                resource_ids: Vec::new(),
            };
            match api::courses::create_session(
                &cid, &req,
            )
            .await
            {
                Ok(sess) => {
                    nav_to.set(Some(format!(
                        "/courses/{}/session/{}",
                        cid, sess.id
                    )));
                }
                Err(e) => {
                    crate::api::log::log_error(
                        &format!(
                            "Create session failed: {e}"
                        ),
                        "session_create",
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
                title: "New Session",
                span {}
            }
            div {
                class: "flex flex-col gap-4 max-w-lg",
                Input {
                    input_type: "text".to_string(),
                    id: "sess-topic".to_string(),
                    label: "Topic".to_string(),
                    placeholder: "Session topic".to_string(),
                    value: topic,
                    on_input: move |v: String| {
                        topic.set(v);
                    },
                    required: true,
                }
                Input {
                    input_type: "text".to_string(),
                    id: "sess-kw".to_string(),
                    label: "Keywords (comma separated)"
                        .to_string(),
                    placeholder: "AI, machine learning"
                        .to_string(),
                    value: keywords,
                    on_input: move |v: String| {
                        keywords.set(v);
                    },
                }
                Textarea {
                    id: "sess-instr".to_string(),
                    label: "Instructions".to_string(),
                    placeholder:
                        "Additional instructions..."
                            .to_string(),
                    value: instr,
                    on_input: move |v: String| {
                        instr.set(v);
                    },
                }
                Input {
                    input_type: "text".to_string(),
                    id: "sess-lang".to_string(),
                    label: "Language".to_string(),
                    value: lang,
                    on_input: move |v: String| {
                        lang.set(v);
                    },
                }
                if (loading)() {
                    Spinner {}
                } else {
                    Button {
                        text: "Create Session".to_string(),
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

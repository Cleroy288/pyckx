//! Intello Page - Learning games dashboard

use crate::components::intello::ManualQcmForm;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{btn_click, Button, ButtonSize, ButtonVariant};
use crate::components::ui::icon::Icon;
use crate::state::use_auth;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

stylance::import_crate_style!(style, "src/pages/intello/intello.module.css");

/// View state for the Intello app
#[derive(Clone, Copy, PartialEq, Default)]
enum IntelloView {
    #[default]
    Home,
    CreateQcm,
}

#[component]
pub fn IntelloPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    // Current view state
    let current_view = RwSignal::new(IntelloView::Home);

    // Redirect to login if not authenticated
    Effect::new(move |_| {
        let is_checking = auth.is_checking_session.get();
        let has_user = auth.user.get().is_some();

        if !is_checking && !has_user {
            navigate("/login", Default::default());
        }
    });

    // Navigation callbacks
    let go_home = Callback::new(move |_: ()| current_view.set(IntelloView::Home));

    // Handle successful QCM creation
    let on_qcm_created = Callback::new(move |set_id: String| {
        web_sys::console::log_1(&format!("Created QCM set: {}", set_id).into());
        current_view.set(IntelloView::Home);
    });

    view! {
        <>
            <HomeTopBar />
            <div class=style::page>
                // Show loading while checking session
                <Show
                    when=move || !auth.is_checking_session.get()
                    fallback=|| view! { <div class=style::loading>"Checking session..."</div> }
                >
                    // Show content only when authenticated
                    <Show
                        when=move || auth.user.get().is_some()
                        fallback=|| view! { <div class=style::loading>"Redirecting to login..."</div> }
                    >
                        // Dynamic view based on state
                        {move || match current_view.get() {
                            IntelloView::Home => view! {
                                <IntelloHome current_view=current_view />
                            }.into_any(),
                            IntelloView::CreateQcm => view! {
                                <div class=style::create_view>
                                    <div class=style::view_header>
                                        <button
                                            class=style::back_btn
                                            on:click=move |_| current_view.set(IntelloView::Home)
                                        >
                                            <Icon icon_name="ArrowLeft".to_string() />
                                            " Back"
                                        </button>
                                        <h2 class=style::view_title>"Create QCM Set"</h2>
                                    </div>
                                    <ManualQcmForm
                                        on_back=go_home
                                        on_success=on_qcm_created
                                    />
                                </div>
                            }.into_any(),
                        }}
                    </Show>
                </Show>
            </div>
        </>
    }
}

/// Intello Home View - Game selection
#[component]
fn IntelloHome(current_view: RwSignal<IntelloView>) -> impl IntoView {
    view! {
        <div class=style::home>
            <header class=style::header>
                <h1 class=style::title>"Intello"</h1>
                <p class=style::subtitle>"Your AI-powered learning companion"</p>
            </header>

            <section class=style::games_section>
                <h2 class=style::section_title>"Learning Games"</h2>
                <div class=style::games_grid>
                    // QCM Card
                    <div class=style::game_card>
                        <div class=style::game_icon>
                            <Icon icon_name="HelpCircle".to_string() />
                        </div>
                        <h3 class=style::game_title>"QCM"</h3>
                        <p class=style::game_description>
                            "Multiple choice questions to test your knowledge"
                        </p>
                        <div class=style::game_actions>
                            <Button
                                text="Create Set".to_string()
                                icon="Plus"
                                size=ButtonSize::Small
                                on_click=btn_click(move |_| current_view.set(IntelloView::CreateQcm))
                            />
                            <Button
                                text="My Sets".to_string()
                                icon="List"
                                variant=ButtonVariant::Outline
                                size=ButtonSize::Small
                            />
                        </div>
                    </div>

                    // Flashcards Card (Coming soon)
                    <div class=format!("{} {}", style::game_card, style::coming_soon)>
                        <div class=style::game_icon>
                            <Icon icon_name="Layers".to_string() />
                        </div>
                        <h3 class=style::game_title>"Flashcards"</h3>
                        <p class=style::game_description>
                            "Memorize with spaced repetition flashcards"
                        </p>
                        <span class=style::badge>"Coming Soon"</span>
                    </div>

                    // Open Questions Card (Coming soon)
                    <div class=format!("{} {}", style::game_card, style::coming_soon)>
                        <div class=style::game_icon>
                            <Icon icon_name="MessageSquare".to_string() />
                        </div>
                        <h3 class=style::game_title>"Open Questions"</h3>
                        <p class=style::game_description>
                            "Practice with AI-graded open-ended questions"
                        </p>
                        <span class=style::badge>"Coming Soon"</span>
                    </div>

                    // True/False Card (Coming soon)
                    <div class=format!("{} {}", style::game_card, style::coming_soon)>
                        <div class=style::game_icon>
                            <Icon icon_name="CheckCircle".to_string() />
                        </div>
                        <h3 class=style::game_title>"True or False"</h3>
                        <p class=style::game_description>
                            "Quick true/false questions for rapid learning"
                        </p>
                        <span class=style::badge>"Coming Soon"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}

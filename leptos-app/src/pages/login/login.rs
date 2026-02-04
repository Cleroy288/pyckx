
use crate::api;
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardVariant};
use crate::components::ui::input::Input;
use crate::state::use_auth;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

stylance::import_crate_style!(login_style, "src/pages/login/login.module.css");

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    // Form state
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    // Signal to trigger navigation after successful login
    let navigate_to = RwSignal::new(Option::<String>::None);

    // Effect to handle navigation (must be in sync context)
    // Also redirect to home if user is already authenticated
    Effect::new(move |_| {
        let is_checking = auth.is_checking_session.get();
        let has_user = auth.user.get().is_some();

        // If we're done checking and user is authenticated, redirect to home
        if !is_checking && has_user {
            navigate("/home", Default::default());
            return;
        }

        // Handle navigation triggered by login success
        if let Some(path) = navigate_to.get() {
            navigate(&path, Default::default());
            navigate_to.set(None);
        }
    });

    // Handle form submission
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let email_val = email.get();
        let password_val = password.get();

        // Clear previous errors
        auth.clear_error();
        auth.is_loading.set(true);

        spawn_local(async move {
            match api::login(&email_val, &password_val).await {
                Ok(user) => {
                    auth.set_user(user);
                    auth.is_loading.set(false);
                    // Trigger navigation via signal
                    navigate_to.set(Some("/home".to_string()));
                }
                Err(error) => {
                    auth.set_error(error);
                    auth.is_loading.set(false);
                }
            }
        });
    };

    view! {
        <>
            <VitrineTopBar />
            <div class="page-container">
                <Card variant=CardVariant::Solid class=login_style::login_card.to_string()>
                    <h1 class=login_style::login_title>"Login"</h1>

                    // Error message
                    <Show when=move || auth.error.get().is_some() fallback=|| ()>
                        <div class=login_style::error_message>
                            {move || auth.error.get().unwrap_or_default()}
                        </div>
                    </Show>

                    <form class=login_style::login_form on:submit=on_submit>
                        <Input
                            type_="email".to_string()
                            id="email".to_string()
                            label="Email".to_string()
                            placeholder="Enter your email".to_string()
                            required=true
                            value=email
                            on_input=Callback::new(move |val: String| email.set(val))
                        />

                        <Input
                            type_="password".to_string()
                            id="password".to_string()
                            label="Password".to_string()
                            placeholder="Enter your password".to_string()
                            required=true
                            minlength=6u32
                            value=password
                            on_input=Callback::new(move |val: String| password.set(val))
                        />

                        <Button
                            type_="submit".to_string()
                            text="Login".to_string()
                            disabled=auth.is_loading
                            class=login_style::submit_button.to_string()
                        />
                    </form>

                    <div class=login_style::back_link>
                        <a href="/">"← Back to Home"</a>
                    </div>
                </Card>
            </div>
        </>
    }
}


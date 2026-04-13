//! Login page — email + password form.

use crate::auth::api;
use crate::auth::shared::{AuthCard, AuthField, AuthSubmit};
use crate::auth::state::{
    use_auth, use_redirect_when_authenticated, AuthState,
};
use dioxus::prelude::*;

/// Login route component.
#[component]
pub fn LoginPage() -> Element {
    let auth = use_auth();
    use_redirect_when_authenticated();

    let email = use_signal(String::new);
    let password = use_signal(String::new);
    let error = use_memo(move || (auth.error)());

    let on_submit = submit_handler(auth, email, password);

    rsx! {
        AuthCard {
            title: "Login",
            subtitle: "Sign in to your account",
            error: error,
            form { class: "flex flex-col gap-3",
                onsubmit: on_submit,
                AuthField {
                    input_type: "email",
                    placeholder: "Email",
                    value: email,
                }
                AuthField {
                    input_type: "password",
                    placeholder: "Password",
                    value: password,
                }
                div { class: "mt-2",
                    AuthSubmit {
                        label: "Sign in",
                        disabled: (auth.is_loading)(),
                    }
                }
            }
            div { class: "text-center mt-6 text-sm",
                Link {
                    to: "/register",
                    class: "underline",
                    "Create an account"
                }
            }
        }
    }
}

/// Build the form `onsubmit` handler.
fn submit_handler(
    mut auth: AuthState,
    email: Signal<String>,
    password: Signal<String>,
) -> impl FnMut(Event<FormData>) + 'static {
    move |event: Event<FormData>| {
        event.prevent_default();
        auth.clear_error();
        auth.is_loading.set(true);
        let typed_email = (email)();
        let typed_password = (password)();
        spawn(async move {
            run_login(auth, typed_email, typed_password).await;
        });
    }
}

/// Call the API and update the auth state.
async fn run_login(
    mut auth: AuthState,
    email: String,
    password: String,
) {
    match api::login(&email, &password).await {
        Ok(user) => {
            auth.set_user(user);
            auth.is_loading.set(false);
        }
        Err(message) => {
            auth.set_error(message);
            auth.is_loading.set(false);
        }
    }
}

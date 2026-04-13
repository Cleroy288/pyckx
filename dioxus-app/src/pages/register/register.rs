//! Registration page with form validation

use crate::api;
use crate::components::auth::{
    AuthInput, AuthLayout, AuthSubmitBtn,
};
use crate::domain::auth_types::RegisterRequest;
use crate::domain::validation::{
    is_valid_email, passwords_match,
};
use crate::state::hooks::use_redirect_if_authenticated;
use crate::state::use_auth;
use dioxus::prelude::*;

/// Registration page
pub fn RegisterPage() -> Element {
    let mut auth = use_auth();
    use_redirect_if_authenticated();

    let email = use_signal(String::new);
    let password = use_signal(String::new);
    let confirm = use_signal(String::new);
    let username = use_signal(String::new);
    let mut error: Signal<Option<String>> =
        use_signal(|| None);
    let mut loading = use_signal(|| false);

    let valid = use_memo(move || {
        is_valid_email(&(email)())
            && passwords_match(
                &(password)(),
                &(confirm)(),
            )
            && !(username)().trim().is_empty()
    });

    let on_submit = move |ev: Event<FormData>| {
        ev.prevent_default();
        error.set(None);
        loading.set(true);
        spawn(async move {
            let req = RegisterRequest {
                email: (email)(),
                password: (password)(),
                username: (username)(),
                phone_country_code: None,
                phone_number: None,
            };
            match api::auth::register(&req).await {
                Ok(user) => {
                    auth.set_user(user);
                    // Redirect handled by
                    // use_redirect_if_authenticated
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    let btn_disabled =
        !(valid)() || (loading)();

    #[allow(clippy::redundant_closure)]
    let error_ro: Memo<Option<String>> =
        use_memo(move || error());

    rsx! {
        AuthLayout {
            title: "Register",
            subtitle: "Create your account",
            error: error_ro,
            form { onsubmit: on_submit,
                div { class: "auth-fields",
                    AuthInput {
                        input_type: "text",
                        placeholder: "Username",
                        value: username,
                    }
                    AuthInput {
                        input_type: "email",
                        placeholder: "Email",
                        value: email,
                    }
                    AuthInput {
                        input_type: "password",
                        placeholder: "Password",
                        value: password,
                    }
                    AuthInput {
                        input_type: "password",
                        placeholder: "Confirm password",
                        value: confirm,
                    }
                }
                AuthSubmitBtn {
                    label: "Create Account",
                    disabled: btn_disabled,
                }
            }
            div {
                class: "text-center mt-6",
                Link {
                    to: "/login",
                    class: "auth-link",
                    "Already have an account? Login"
                }
            }
        }
    }
}

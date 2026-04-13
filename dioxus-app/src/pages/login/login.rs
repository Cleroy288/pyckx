//! Login page with email/password form

use crate::api;
use crate::components::auth::{
    AuthInput, AuthLayout, AuthSubmitBtn,
};
use crate::state::hooks::use_redirect_if_authenticated;
use crate::state::use_auth;
use dioxus::prelude::*;

/// Login page
pub fn LoginPage() -> Element {
    let mut auth = use_auth();
    use_redirect_if_authenticated();

    let email = use_signal(String::new);
    let password = use_signal(String::new);

    let on_submit = move |ev: Event<FormData>| {
        ev.prevent_default();
        auth.clear_error();
        auth.is_loading.set(true);
        let e = (email)();
        let p = (password)();
        spawn(async move {
            match api::login(&e, &p).await {
                Ok(user) => {
                    auth.set_user(user);
                    auth.is_loading.set(false);
                    // Redirect handled by
                    // use_redirect_if_authenticated
                }
                Err(error) => {
                    auth.set_error(error);
                    auth.is_loading.set(false);
                }
            }
        });
    };

    let error_sig = use_memo(move || {
        (auth.error)()
    });

    rsx! {
        AuthLayout {
            title: "Login",
            subtitle: "Sign in to your account",
            error: error_sig,
            form { onsubmit: on_submit,
                div { class: "auth-fields",
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
                }
                AuthSubmitBtn {
                    label: "Login",
                    disabled: (auth.is_loading)(),
                }
            }
            div {
                class: "text-center mt-6",
                Link {
                    to: "/",
                    class: "auth-link",
                    "\u{2190} Back to Home"
                }
            }
        }
    }
}

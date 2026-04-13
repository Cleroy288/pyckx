//! Register page — username, email, password + confirm.

use crate::auth::api;
use crate::auth::shared::{AuthCard, AuthField, AuthSubmit};
use crate::auth::state::{
    use_auth, use_redirect_when_authenticated, AuthState,
};
use crate::domain::auth_types::RegisterRequest;
use crate::domain::validation::{
    is_valid_email, passwords_match,
};
use dioxus::prelude::*;

/// Register route component.
#[component]
pub fn RegisterPage() -> Element {
    let auth = use_auth();
    use_redirect_when_authenticated();

    let username = use_signal(String::new);
    let email = use_signal(String::new);
    let password = use_signal(String::new);
    let confirm = use_signal(String::new);

    let valid = use_memo(move || {
        is_form_valid(&username(), &email(), &password(), &confirm())
    });
    let error = use_memo(move || (auth.error)());
    let disabled = !valid() || (auth.is_loading)();

    let on_submit = submit_handler(
        auth, username, email, password,
    );

    rsx! {
        AuthCard {
            title: "Register",
            subtitle: "Create your account",
            error: error,
            form { class: "flex flex-col gap-3",
                onsubmit: on_submit,
                AuthField {
                    input_type: "text",
                    placeholder: "Username",
                    value: username,
                }
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
                AuthField {
                    input_type: "password",
                    placeholder: "Confirm password",
                    value: confirm,
                }
                div { class: "mt-2",
                    AuthSubmit {
                        label: "Create account",
                        disabled: disabled,
                    }
                }
            }
            div { class: "text-center mt-6 text-sm",
                Link {
                    to: "/login",
                    class: "underline",
                    "Already have an account? Sign in"
                }
            }
        }
    }
}

/// Pure form validation: all fields filled, email valid,
/// passwords match and non-empty.
pub fn is_form_valid(
    username: &str,
    email: &str,
    password: &str,
    confirm: &str,
) -> bool {
    !username.trim().is_empty()
        && is_valid_email(email)
        && passwords_match(password, confirm)
}

/// Build the form `onsubmit` handler.
fn submit_handler(
    mut auth: AuthState,
    username: Signal<String>,
    email: Signal<String>,
    password: Signal<String>,
) -> impl FnMut(Event<FormData>) + 'static {
    move |event: Event<FormData>| {
        event.prevent_default();
        auth.clear_error();
        auth.is_loading.set(true);
        let request = build_request(username, email, password);
        spawn(async move {
            run_register(auth, request).await;
        });
    }
}

/// Snapshot the signals into a `RegisterRequest`.
fn build_request(
    username: Signal<String>,
    email: Signal<String>,
    password: Signal<String>,
) -> RegisterRequest {
    RegisterRequest {
        username: (username)(),
        email: (email)(),
        password: (password)(),
        phone_country_code: None,
        phone_number: None,
    }
}

/// Call the API and update auth state.
async fn run_register(
    mut auth: AuthState,
    request: RegisterRequest,
) {
    match api::register(&request).await {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_form_valid_all_fields_valid_returns_true() {
        let ok = is_form_valid(
            "alice", "a@b.com", "pw12", "pw12",
        );
        assert!(ok);
    }

    #[test]
    fn test_is_form_valid_empty_username_returns_false() {
        let ok = is_form_valid(
            "  ", "a@b.com", "pw12", "pw12",
        );
        assert!(!ok);
    }

    #[test]
    fn test_is_form_valid_invalid_email_returns_false() {
        let ok = is_form_valid(
            "alice", "nope", "pw12", "pw12",
        );
        assert!(!ok);
    }

    #[test]
    fn test_is_form_valid_mismatched_passwords_returns_false() {
        let ok = is_form_valid(
            "alice", "a@b.com", "pw12", "other",
        );
        assert!(!ok);
    }

    #[test]
    fn test_is_form_valid_empty_password_returns_false() {
        let ok = is_form_valid(
            "alice", "a@b.com", "", "",
        );
        assert!(!ok);
    }
}

//! Registration page with form validation

use crate::api;
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::domain::auth_types::RegisterRequest;
use crate::state::hooks::{
    use_async_navigate,
    use_redirect_if_authenticated,
};
use crate::state::use_auth;
use dioxus::prelude::*;

/// Validates password match
fn passwords_match(a: &str, b: &str) -> bool {
    !a.is_empty() && a == b
}

/// Validates email has @ and domain
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> =
        email.split('@').collect();
    parts.len() == 2
        && !parts[0].is_empty()
        && parts[1].contains('.')
}

/// Registration page
pub fn RegisterPage() -> Element {
    let mut auth = use_auth();
    use_redirect_if_authenticated();
    let mut nav_to = use_async_navigate();

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut username = use_signal(String::new);
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
                    nav_to.set(Some("/home".into()));
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        VitrineTopBar {}
        div {
            class: "relative flex items-center \
                justify-center min-h-screen \
                bg-[var(--color-background)] \
                pt-20 pb-8 z-[1]",
            div {
                class: "relative flex flex-col w-full \
                    max-w-[420px] min-h-[580px] p-8 \
                    bg-[var(--glass-bg)] \
                    backdrop-blur-[20px] \
                    border border-[var(--color-border)] \
                    animate-[fadeInUp_0.6s_ease-out]",
                // Title bar
                div {
                    class: "relative overflow-hidden \
                        -mx-8 -mt-8 mb-8 px-8 py-6 \
                        text-center bg-[var(--glass-bg)] \
                        border-b border-[color-mix(in_srgb,var(--primary)_15%,transparent)]",
                    h1 {
                        class: "relative z-[1] m-0 text-[2rem] \
                            font-bold tracking-tight \
                            bg-gradient-to-br \
                            from-[var(--color-primary)] \
                            to-[color-mix(in_srgb,var(--primary)_70%,black)] \
                            bg-clip-text text-transparent",
                        "Register"
                    }
                }
                // Error message
                if let Some(err) = (error)() {
                    div {
                        class: "bg-[color-mix(in_srgb,var(--destructive)_10%,transparent)] \
                            border border-[color-mix(in_srgb,var(--destructive)_30%,transparent)] \
                            text-[var(--color-error)] \
                            px-4 py-3 mb-6 text-sm text-center",
                        "{err}"
                    }
                }
                // Form
                form {
                    class: "flex flex-col flex-1 -mx-8",
                    onsubmit: on_submit,
                    div {
                        class: "relative flex flex-col \
                            flex-1 justify-center gap-10",
                        Input {
                            input_type: "text".to_string(),
                            id: "reg-username".to_string(),
                            placeholder: "Username".to_string(),
                            required: true,
                            value: username,
                            on_input: move |v: String| {
                                username.set(v);
                            },
                        }
                        Input {
                            input_type: "email".to_string(),
                            id: "reg-email".to_string(),
                            placeholder: "Email".to_string(),
                            required: true,
                            value: email,
                            on_input: move |v: String| {
                                email.set(v);
                            },
                        }
                        Input {
                            input_type: "password".to_string(),
                            id: "reg-pw".to_string(),
                            placeholder: "Password".to_string(),
                            required: true,
                            value: password,
                            on_input: move |v: String| {
                                password.set(v);
                            },
                        }
                        Input {
                            input_type: "password".to_string(),
                            id: "reg-confirm".to_string(),
                            placeholder: "Confirm password"
                                .to_string(),
                            required: true,
                            value: confirm,
                            on_input: move |v: String| {
                                confirm.set(v);
                            },
                        }
                    }
                    Button {
                        type_: "submit".to_string(),
                        text: "Create Account".to_string(),
                        disabled: Signal::new(
                            !(valid)() || (loading)()
                        ),
                        class: "mt-6 w-full".to_string(),
                    }
                }
                // Back link
                div {
                    class: "text-center mt-6",
                    Link {
                        to: "/login",
                        class: "text-[var(--color-text-secondary)] \
                            no-underline text-sm \
                            hover:text-[var(--color-primary)] \
                            transition-colors \
                            duration-[var(--transition-fast)]",
                        "Already have an account? Login"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passwords_match_identical_strings() {
        assert!(passwords_match("abc123", "abc123"));
    }

    #[test]
    fn test_passwords_match_different_strings() {
        assert!(!passwords_match("abc", "xyz"));
    }

    #[test]
    fn test_passwords_match_empty_returns_false() {
        assert!(!passwords_match("", ""));
    }

    #[test]
    fn test_passwords_match_one_empty() {
        assert!(!passwords_match("abc", ""));
    }

    #[test]
    fn test_is_valid_email_standard_format() {
        assert!(is_valid_email("user@example.com"));
    }

    #[test]
    fn test_is_valid_email_subdomain() {
        assert!(is_valid_email("a@sub.example.com"));
    }

    #[test]
    fn test_is_valid_email_missing_at() {
        assert!(!is_valid_email("userexample.com"));
    }

    #[test]
    fn test_is_valid_email_missing_domain_dot() {
        assert!(!is_valid_email("user@example"));
    }

    #[test]
    fn test_is_valid_email_empty_string() {
        assert!(!is_valid_email(""));
    }

    #[test]
    fn test_is_valid_email_empty_local_part() {
        assert!(!is_valid_email("@example.com"));
    }

    #[test]
    fn test_is_valid_email_double_at() {
        assert!(!is_valid_email("a@b@c.com"));
    }
}

// ** register.rs **
// ==> Registration page with form validation

use crate::api;
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::components::ui::toast::use_toast;
use crate::domain::auth_types::RegisterRequest;
use crate::state::hooks::{
    use_async_navigate, use_redirect_if_authenticated,
};
use crate::state::use_auth;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

stylance::import_crate_style!(
    style,
    "src/pages/register/register.module.css"
);

/// Validates password match
fn passwords_match(a: &str, b: &str) -> bool {
    !a.is_empty() && a == b
}

/// Validates email has @ and domain
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    parts.len() == 2
        && !parts[0].is_empty()
        && parts[1].contains('.')
}

/// Registration page
#[component]
pub fn RegisterPage() -> impl IntoView {
    let auth = use_auth();
    let toast = use_toast();
    use_redirect_if_authenticated();
    let nav_to = use_async_navigate();

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm = RwSignal::new(String::new());
    let username = RwSignal::new(String::new());
    let error: RwSignal<Option<String>> =
        RwSignal::new(None);
    let loading = RwSignal::new(false);

    let valid = Signal::derive(move || {
        is_valid_email(&email.get())
            && passwords_match(
                &password.get(),
                &confirm.get(),
            )
            && !username.get().trim().is_empty()
    });

    let on_submit =
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            error.set(None);
            loading.set(true);
            spawn_local(async move {
                let req = RegisterRequest {
                    email: email.get(),
                    password: password.get(),
                    username: username.get(),
                    phone_country_code: None,
                    phone_number: None,
                };
                match api::auth::register(&req).await
                {
                    Ok(user) => {
                        auth.user.set(Some(user));
                        toast.success(
                            "Account created!"
                                .into(),
                        );
                        nav_to.set(Some(
                            "/home".into(),
                        ));
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
        };

    view! {
        <>
            <VitrineTopBar />
            <div class=style::register_page>
                <div class=style::register_card>
                    <div class=style::register_title_bar>
                        <h1 class=style::register_title>
                            "Register"
                        </h1>
                    </div>

                    // Error message
                    <Show
                        when=move || {
                            error.get().is_some()
                        }
                        fallback=|| ()
                    >
                        <div class=style::error_message>
                            {move || {
                                error
                                    .get()
                                    .unwrap_or_default()
                            }}
                        </div>
                    </Show>

                    <form
                        class=style::register_form
                        on:submit=on_submit
                    >
                        <div
                            class=style::register_inputs
                        >
                            <Input
                                type_="text"
                                    .to_string()
                                id="reg-username"
                                    .to_string()
                                placeholder="Username"
                                    .to_string()
                                required=true
                                value=username
                                on_input=Callback::new(
                                    move |v: String| {
                                        username.set(v)
                                    },
                                )
                            />
                            <Input
                                type_="email"
                                    .to_string()
                                id="reg-email"
                                    .to_string()
                                placeholder="Email"
                                    .to_string()
                                required=true
                                value=email
                                on_input=Callback::new(
                                    move |v: String| {
                                        email.set(v)
                                    },
                                )
                            />
                            <Input
                                type_="password"
                                    .to_string()
                                id="reg-pw"
                                    .to_string()
                                placeholder="Password"
                                    .to_string()
                                required=true
                                value=password
                                on_input=Callback::new(
                                    move |v: String| {
                                        password.set(v)
                                    },
                                )
                            />
                            <Input
                                type_="password"
                                    .to_string()
                                id="reg-confirm"
                                    .to_string()
                                placeholder="Confirm password"
                                    .to_string()
                                required=true
                                value=confirm
                                on_input=Callback::new(
                                    move |v: String| {
                                        confirm.set(v)
                                    },
                                )
                            />
                        </div>

                        <Button
                            type_="submit".to_string()
                            text="Create Account"
                                .to_string()
                            disabled=Signal::derive(
                                move || {
                                    !valid.get()
                                        || loading.get()
                                },
                            )
                            class=style::submit_button
                                .to_string()
                        />
                    </form>

                    <div class=style::back_link>
                        <A href="/login">
                            "Already have an account? Login"
                        </A>
                    </div>
                </div>
            </div>
        </>
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
    fn test_passwords_match_one_empty_returns_false()
    {
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

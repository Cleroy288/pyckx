//! Login page with email/password form

use crate::api;
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::state::hooks::{
    use_async_navigate,
    use_redirect_if_authenticated,
};
use crate::state::use_auth;
use dioxus::prelude::*;

/// Login page
pub fn LoginPage() -> Element {
    let mut auth = use_auth();
    use_redirect_if_authenticated();
    let mut nav_to = use_async_navigate();

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

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
                    nav_to.set(Some("/home".into()));
                }
                Err(error) => {
                    auth.set_error(error);
                    auth.is_loading.set(false);
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
                    max-w-[420px] min-h-[520px] p-8 \
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
                        "Login"
                    }
                }
                // Error message
                if let Some(err) = (auth.error)() {
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
                            flex-1 justify-center gap-16",
                        Input {
                            input_type: "email".to_string(),
                            id: "email".to_string(),
                            placeholder: "Enter your email"
                                .to_string(),
                            required: true,
                            value: email,
                            on_input: move |v: String| {
                                email.set(v);
                            },
                        }
                        Input {
                            input_type: "password".to_string(),
                            id: "password".to_string(),
                            placeholder: "Enter your password"
                                .to_string(),
                            required: true,
                            value: password,
                            on_input: move |v: String| {
                                password.set(v);
                            },
                        }
                    }
                    Button {
                        type_: "submit".to_string(),
                        text: "Login".to_string(),
                        disabled: auth.is_loading,
                        class: "mt-6 w-full".to_string(),
                    }
                }
                // Back link
                div {
                    class: "text-center mt-6",
                    Link {
                        to: "/",
                        class: "text-[var(--color-text-secondary)] \
                            no-underline text-sm \
                            hover:text-[var(--color-primary)] \
                            transition-colors \
                            duration-[var(--transition-fast)]",
                        "< Back to Home"
                    }
                }
            }
        }
    }
}

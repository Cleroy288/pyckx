// ** login.rs **
// ==> Login page with email/password form

use crate::api;
use crate::components::top_bar::VitrineTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::state::hooks::{
    use_async_navigate, use_redirect_if_authenticated,
};
use crate::state::use_auth;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

stylance::import_crate_style!(
    style,
    "src/pages/login/login.module.css"
);

/// Login page
#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    use_redirect_if_authenticated();
    let nav_to = use_async_navigate();

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let on_submit =
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            auth.clear_error();
            auth.is_loading.set(true);
            let e = email.get();
            let p = password.get();
            spawn_local(async move {
                match api::login(&e, &p).await {
                    Ok(user) => {
                        auth.set_user(user);
                        auth.is_loading.set(false);
                        nav_to.set(Some(
                            "/home".into(),
                        ));
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
            <div class=style::login_page>
                <div class=style::login_card>
                    <div class=style::login_title_bar>
                        <h1 class=style::login_title>
                            "Login"
                        </h1>
                    </div>
                    <Show
                        when=move || {
                            auth.error.get().is_some()
                        }
                        fallback=|| ()
                    >
                        <div class=style::error_message>
                            {move || {
                                auth.error
                                    .get()
                                    .unwrap_or_default()
                            }}
                        </div>
                    </Show>
                    <form
                        class=style::login_form
                        on:submit=on_submit
                    >
                        <div class=style::login_inputs>
                            <Input
                                type_="email"
                                    .to_string()
                                id="email".to_string()
                                placeholder="Enter your email"
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
                                id="password"
                                    .to_string()
                                placeholder="Enter your password"
                                    .to_string()
                                required=true
                                minlength=6u32
                                value=password
                                on_input=Callback::new(
                                    move |v: String| {
                                        password.set(v)
                                    },
                                )
                            />
                        </div>
                        <Button
                            type_="submit"
                                .to_string()
                            text="Login".to_string()
                            disabled=auth.is_loading
                            class=style::submit_button
                                .to_string()
                        />
                    </form>
                    <div class=style::back_link>
                        <A href="/">
                            "← Back to Home"
                        </A>
                    </div>
                </div>
            </div>
        </>
    }
}

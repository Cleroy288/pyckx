use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod api;
mod components;
mod pages;
mod state;

use components::logo::Logo;
use pages::{HomePage, LoginPage, Vitrine};
use state::AuthProvider;

// Import the background CSS module
stylance::import_crate_style!(background_style, "styles/background.module.css");

// ** Background **
// ==> Displays the background with grid lines and contains child components
// @children: Child components to render inside the background
#[component]
fn Background(children: Children) -> impl IntoView {
    view! {
        <div class=background_style::background>
            <div class=background_style::lines></div>
            <Logo />
            {children()}
        </div>
    }
}

// ** App **
// ==> Main application component with routing
// @returns: View containing the app with router and all pages
#[component]
fn App() -> impl IntoView {
    view! {
        <AuthProvider>
            <Router>
                <Background>
                    <Routes fallback=|| view! { <p>"Page not found"</p> }>
                        <Route path=path!("/") view=Vitrine />
                        <Route path=path!("/login") view=LoginPage />
                        <Route path=path!("/home") view=HomePage />
                    </Routes>
                </Background>
            </Router>
        </AuthProvider>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

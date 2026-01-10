//! Leptos App - Clean Hello World

use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Hello, Leptos!"</h1>
            <p>"This is a fresh start for the Leptos implementation."</p>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

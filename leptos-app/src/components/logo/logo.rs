//! Logo component displaying a hexagon with "P" inside and "PYCKX" text

use leptos::prelude::*;

// Import the logo CSS module from the same folder
stylance::import_crate_style!(logo_style, "src/components/logo/logo.module.css");

// ** Logo **
// ==> Displays the Pyckx logo with hexagon icon and text
// @returns: View containing the logo with hexagon and brand name
#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <a href="/" class=logo_style::logo tabindex=0 aria-label="Pyckx Home">
            <div class=logo_style::icon_wrapper>
                <svg
                    class=logo_style::hexagon
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                </svg>
                <span class=logo_style::letter>"P"</span>
            </div>
            <span class=logo_style::brand_name>"PYCKX"</span>
        </a>
    }
}

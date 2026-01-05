//! GameCard Component - Intello game selection card

use leptos::prelude::*;
use leptos::ev;
use crate::shared::components::{Icon, IconType};
use crate::intello::service::GameCardData;

#[component]
pub fn GameCard<F>(game: GameCardData, on_click: F) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <button class="game-card" on:click=on_click>
            <div class="game-card__content">
                <div class="game-card__icon">
                    <Icon icon=game.icon />
                </div>
                <div>
                    <h3 class="game-card__title">{game.title}</h3>
                    <p class="game-card__description">{game.description}</p>
                </div>
            </div>
            <div class="game-card__action">
                <Icon icon=IconType::ArrowRight />
            </div>
        </button>
    }
}

//! Intello Page View

use leptos::prelude::*;
use crate::shared::components::IconType;
use crate::intello::components::{GameCard, Section, IntelloHeader};
use crate::intello::service::{get_create_games, get_play_games, get_course_cards};

#[component]
pub fn IntelloPage(on_navigate: WriteSignal<String>) -> impl IntoView {
    let create_games = get_create_games();
    let play_games = get_play_games();
    let course_cards = get_course_cards();

    view! {
        <div>
            <IntelloHeader />

            <div class="sections-grid">
                <Section title="Create" subtitle="New Content" icon=IconType::Plus>
                    {create_games.into_iter().map(|game| {
                        let game_id = game.id.to_string();
                        view! { <GameCard game=game on_click=move |_| on_navigate.set(game_id.clone()) /> }
                    }).collect::<Vec<_>>()}
                </Section>

                <Section title="Play" subtitle="Your Library" icon=IconType::Gamepad>
                    {play_games.into_iter().map(|game| {
                        let game_id = game.id.to_string();
                        view! { <GameCard game=game on_click=move |_| on_navigate.set(game_id.clone()) /> }
                    }).collect::<Vec<_>>()}
                </Section>
            </div>

            <section class="courses-section">
                <div class="section-header">
                    <div class="section-icon">
                        <Icon icon=IconType::BookOpen />
                    </div>
                    <div>
                        <h2 class="section-title">"Courses"</h2>
                        <p class="section-subtitle">"Study Materials"</p>
                    </div>
                </div>
                <div class="courses-grid">
                    {course_cards.into_iter().map(|game| {
                        let game_id = game.id.to_string();
                        view! { <GameCard game=game on_click=move |_| on_navigate.set(game_id.clone()) /> }
                    }).collect::<Vec<_>>()}
                </div>
            </section>
        </div>
    }
}

use crate::shared::components::Icon;

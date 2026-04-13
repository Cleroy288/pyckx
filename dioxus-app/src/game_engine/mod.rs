//! Game engine — shared foundation for every game type.
//!
//! Exposes the generic list/play/generate pages, the
//! reactive player state, the progress + results views,
//! and the domain traits/types needed to plug a new game
//! in. One flat import surface:
//! `use crate::game_engine::*;`.

pub mod generate_form;
pub mod generate_page;
pub mod helpers;
pub mod list_page;
pub mod play_page;
pub mod player_state;
pub mod progress;
pub mod results;
pub mod set_card;
pub mod set_list;
pub mod traits;

pub use generate_form::{GenerateForm, GenerateFormData};
pub use generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
pub use helpers::{derive_field, shuffle};
pub use list_page::{DeleteFn, FetchSets, GameListPage};
pub use play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
pub use player_state::GamePlayerState;
pub use progress::GameProgress;
pub use results::{GameResults, ResultItem};
pub use set_card::GameSetCard;
pub use set_list::SetList;
pub use traits::{GameSetInfo, Level, NumQuestions, StudyApiError};

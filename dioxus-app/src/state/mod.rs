//! State module — Global application state management

pub mod auth;
pub mod auth_hooks;
pub mod coding;
pub mod fetch_hooks;
pub mod hooks;
pub mod nav_hooks;
pub mod palette;
pub mod palette_dom;
pub mod palette_storage;
pub mod palette_provider;
pub mod user_apps;

pub use auth::*;
pub use coding::CodingExerciseProvider;
pub use palette_provider::{PaletteProvider, use_palette};
pub use user_apps::UserAppsProvider;

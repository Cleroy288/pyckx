//! State module — Global application state management

pub mod auth;
pub mod hooks;
pub mod palette;
pub mod palette_dom;
pub mod palette_provider;
pub mod user_apps;

pub use auth::*;
pub use palette_provider::PaletteProvider;
pub use user_apps::UserAppsProvider;

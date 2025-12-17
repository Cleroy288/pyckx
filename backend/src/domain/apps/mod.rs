//! Apps domain module - Core entities for app management
//!
//! Contains domain entities for:
//! - `App` - Available application in the platform
//! - `UserApp` - User's enabled application
//! - `AppInstance` - App module trait and metadata

mod app;
mod app_instance;
mod user_app;

pub use app::App;
pub use app_instance::{AppId, AppInstance, AppModule};
pub use user_app::UserApp;

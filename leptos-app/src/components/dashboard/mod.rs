//! Dashboard Components
//! - WelcomeHeader: Greeting with username
//! - PalettePicker: Color theme selector
//! - AppStore: All-apps catalog with add/remove
//! - MyApps: User's subscribed apps nav cards

mod app_store;
mod my_apps;
mod palette_picker;
mod welcome_header;

pub use app_store::AppStore;
pub use my_apps::MyApps;
pub use palette_picker::PalettePicker;
pub use welcome_header::WelcomeHeader;

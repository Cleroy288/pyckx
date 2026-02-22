//! Pages Module
//! Exports all page components

pub mod admin;
pub mod collection;
pub mod home;
pub mod intello;
pub mod login;
pub mod register;
pub mod vitrine;

pub use admin::AdminPage;
pub use collection::CollectionPage;
pub use home::HomePage;
pub use login::LoginPage;
pub use register::RegisterPage;
pub use vitrine::Vitrine;

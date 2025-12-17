//! Authentication service module
//!
//! Handles login, register, and logout flows.

mod login;
mod logout;
mod register;
mod types;

pub use types::AuthService;

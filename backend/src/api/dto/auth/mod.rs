//! Auth DTOs - Request/Response types for authentication endpoints

mod request;
mod response;

pub use request::{LoginRequest, RegisterRequest};
pub use response::AuthResponse;

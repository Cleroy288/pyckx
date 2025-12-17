//! Data Transfer Objects - Request/Response types for API endpoints
//!
//! # Structure
//! - `apps/` - Apps DTOs
//! - `auth/` - Authentication DTOs
//! - `collection/` - Collection/DVD DTOs
//! - `intello/` - Intello/QCM/Open Question DTOs
//! - `user/` - User DTOs

pub mod apps;
pub mod auth;
pub mod collection;
pub mod intello;
pub mod user;

// Re-export apps types
pub use apps::{
    AddUserAppRequest, AppListResponse, AppResponse, AppSuccessResponse, CreateAppRequest,
    SuccessResponse, UpdateAppRequest, UserAppSuccessResponse,
};

// Re-export auth types
pub use auth::{AuthResponse, LoginRequest, RegisterRequest};

// Re-export collection types
pub use collection::{
    AddDvdRequest, CollectionItemsResponse, CollectionListResponse, CollectionSuccessResponse,
    CreateCollectionRequest, DeleteResponse, DvdListResponse, DvdResponse, DvdSuccessResponse,
    UpdateDvdRequest,
};

// Re-export user types
pub use user::UserResponse;

//! Supabase client module - Authentication operations
//!
//! This module provides:
//! - `SupabaseClient` - Login, register, logout operations

mod auth;
mod types;

pub use auth::SupabaseClient;

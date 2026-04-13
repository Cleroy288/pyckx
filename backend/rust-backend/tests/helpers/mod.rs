//! Shared test helpers for integration tests

pub mod factories;
pub mod stub_app_repo;
pub mod stub_collection_repo;
pub mod stub_dvd_repo;
pub mod stub_study;
pub mod stub_user_app_repo;

// E2E test helpers (real Supabase)
pub mod auth_helper;
pub mod cleanup;
pub mod test_app;

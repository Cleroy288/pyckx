// ============================================================================
// LAPP - Library crate (enables integration tests)
// ============================================================================
#![allow(non_snake_case)]

pub mod app;
pub mod configs;
pub mod http_api;
pub mod infra;
pub mod services;
pub mod shared;

#[cfg(test)]
pub mod tests;

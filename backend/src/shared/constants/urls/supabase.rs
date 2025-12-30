//! URL constants - API paths for external services

// ==============================
// Supabase API paths
// ==============================
pub const SUPABASE_AUTH_PATH: &str = "/auth/v1/token?grant_type=password";
#[allow(dead_code)] // Registration route is disabled but kept for future use
pub const SUPABASE_SIGNUP_PATH: &str = "/auth/v1/signup";
pub const SUPABASE_LOGOUT_PATH: &str = "/auth/v1/logout";

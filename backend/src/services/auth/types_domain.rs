//! AuthService struct and constructor

use crate::configs::Config;
use crate::infra::SessionStore;
use crate::infra::SupabaseClient;
use std::fmt;
use tracing::info;

/// Authentication service - coordinates auth flows
#[derive(Clone, Debug)]
pub struct AuthService {
    pub(super) supabase: SupabaseClient,
    pub(super) sessions: SessionStore,
}

impl AuthService {
    pub fn new(cfg: &Config, sessions: SessionStore) -> Self {
        info!("AuthService initialized");
        Self {
            supabase: SupabaseClient::new(cfg),
            sessions,
        }
    }
}

impl fmt::Display for AuthService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthService({})", self.supabase)
    }
}

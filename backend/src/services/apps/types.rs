//! AppService struct and constructor

use crate::infrastructure::{AppRepository, UserAppRepository};
use std::fmt;
use std::sync::Arc;
use tracing::info;

/// Service for managing apps and user app lists
pub struct AppService {
    pub(super) app_repo: Arc<dyn AppRepository>,
    pub(super) user_app_repo: Arc<dyn UserAppRepository>,
}

impl AppService {
    /// Create a new AppService with the given repositories
    pub fn new(
        app_repo: Arc<dyn AppRepository>,
        user_app_repo: Arc<dyn UserAppRepository>,
    ) -> Self {
        info!("AppService initialized");
        Self {
            app_repo,
            user_app_repo,
        }
    }
}

impl fmt::Debug for AppService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppService").finish()
    }
}

impl fmt::Display for AppService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppService")
    }
}

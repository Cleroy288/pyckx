//! AppInstance - Base representation for all apps in the platform

/// Unique identifier for an app
pub type AppId = &'static str;

/// Metadata for an app instance
#[derive(Debug, Clone)]
pub struct AppInstance {
    #[allow(dead_code)]
    pub id: AppId,
    pub name: &'static str,
    #[allow(dead_code)]
    pub description: &'static str,
}

impl AppInstance {
    /// Create a new app instance
    pub const fn new(id: AppId, name: &'static str, description: &'static str) -> Self {
        Self { id, name, description }
    }
}

/// Trait for app modules - implement this for each app
pub trait AppModule: Send + Sync {
    /// Get app metadata
    fn info(&self) -> &AppInstance;
    
    /// Get the app ID
    #[allow(dead_code)]
    fn id(&self) -> AppId {
        self.info().id
    }
    
    /// Get the app name
    fn name(&self) -> &'static str {
        self.info().name
    }
}

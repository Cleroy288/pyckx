//! App repository types - DTOs for app operations

use serde::Serialize;

// ============================================================================
// CREATE APP DTO
// ============================================================================

/// Data transfer object for creating a new app
#[derive(Debug, Clone, Serialize)]
pub struct CreateApp {
    pub name: String,
    pub description: Option<String>,
}

impl CreateApp {
    pub fn new(name: impl Into<String>, description: Option<String>) -> Self {
        Self {
            name: name.into(),
            description,
        }
    }
}

// ============================================================================
// UPDATE APP DTO
// ============================================================================

/// Data transfer object for updating an app
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateApp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateApp {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

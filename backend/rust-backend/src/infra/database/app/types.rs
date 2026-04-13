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

#[cfg(test)]
mod tests {
    use super::*;

    // -- CreateApp --

    #[test]
    fn test_create_app_new_maps_name() {
        // arrange / act
        let app = CreateApp::new("MyApp", None);

        // assert
        assert_eq!(app.name, "MyApp");
        assert!(app.description.is_none());
    }

    #[test]
    fn test_create_app_new_with_description() {
        // arrange / act
        let app = CreateApp::new(
            "MyApp",
            Some("A great app".into()),
        );

        // assert
        assert_eq!(
            app.description,
            Some("A great app".into())
        );
    }

    // -- UpdateApp --

    #[test]
    fn test_update_app_new_all_none() {
        // arrange / act
        let upd = UpdateApp::new();

        // assert
        assert!(upd.name.is_none());
        assert!(upd.description.is_none());
    }

    #[test]
    fn test_update_app_with_name() {
        // arrange / act
        let upd = UpdateApp::new().with_name("Renamed");

        // assert
        assert_eq!(upd.name, Some("Renamed".into()));
    }

    #[test]
    fn test_update_app_with_description() {
        // arrange / act
        let upd = UpdateApp::new()
            .with_description("New desc");

        // assert
        assert_eq!(
            upd.description,
            Some("New desc".into())
        );
    }
}

//! IntelloApp - App instance for learning games management
//!
//! Intello is a learning application that supports:
//! - QCM (Multiple Choice Quizzes)
//! - Flashcards
//! - Other learning games
//! - JSON-based storage for offline-first experience

use crate::services::app_registry::registry_domain::{AppInstance, AppModule};

// == INTELLO APP METADATA // ==

/// Intello app metadata
const INTELLO_APP: AppInstance = AppInstance::new(
    "intello",
    "Intello",
    "Learn and test your knowledge with QCM, flashcards, and other games",
);

// == INTELLO APP STRUCT // ==

/// IntelloApp - Learning Games Application
#[derive(Debug, Clone)]
pub struct IntelloApp {
    info: AppInstance,
}

impl IntelloApp {
    pub fn new() -> Self {
        Self { info: INTELLO_APP }
    }
}

impl Default for IntelloApp {
    fn default() -> Self {
        Self::new()
    }
}

impl AppModule for IntelloApp {
    fn info(&self) -> &AppInstance {
        &self.info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_intello_app_with_correct_id() {
        // arrange / act
        let app = IntelloApp::new();

        // assert
        assert_eq!(app.info().id, "intello");
    }

    #[test]
    fn test_new_returns_correct_name() {
        // arrange / act
        let app = IntelloApp::new();

        // assert
        assert_eq!(app.info().name, "Intello");
    }

    #[test]
    fn test_default_same_as_new() {
        // arrange / act
        let app = IntelloApp::default();

        // assert
        assert_eq!(app.info().id, "intello");
        assert_eq!(app.info().name, "Intello");
    }

    #[test]
    fn test_app_module_name_returns_intello() {
        // arrange
        let app = IntelloApp::new();

        // act - uses AppModule trait method
        let name = app.name();

        // assert
        assert_eq!(name, "Intello");
    }

    #[test]
    fn test_app_module_id_returns_intello() {
        // arrange
        let app = IntelloApp::new();

        // act - uses AppModule trait method
        let id = app.id();

        // assert
        assert_eq!(id, "intello");
    }
}

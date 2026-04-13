//! StudyApp - App instance for learning games management
//!
//! Study is a learning application that supports:
//! - QCM (Multiple Choice Quizzes)
//! - Flashcards
//! - Other learning games
//! - JSON-based storage for offline-first experience

use crate::services::app_registry::registry_domain::{AppInstance, AppModule};

/// Study app metadata
const STUDY_APP: AppInstance = AppInstance::new(
    "study",
    "Study",
    "Learn and test your knowledge with QCM, flashcards, and other games",
);

/// StudyApp - Learning Games Application
#[derive(Debug, Clone)]
pub struct StudyApp {
    info: AppInstance,
}

impl StudyApp {
    pub fn new() -> Self {
        Self { info: STUDY_APP }
    }
}

impl Default for StudyApp {
    fn default() -> Self {
        Self::new()
    }
}

impl AppModule for StudyApp {
    fn info(&self) -> &AppInstance {
        &self.info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_study_app_with_correct_id() {
        // arrange / act
        let app = StudyApp::new();

        // assert
        assert_eq!(app.info().id, "study");
    }

    #[test]
    fn test_new_returns_correct_name() {
        // arrange / act
        let app = StudyApp::new();

        // assert
        assert_eq!(app.info().name, "Study");
    }

    #[test]
    fn test_default_same_as_new() {
        // arrange / act
        let app = StudyApp::default();

        // assert
        assert_eq!(app.info().id, "study");
        assert_eq!(app.info().name, "Study");
    }

    #[test]
    fn test_app_module_name_returns_study() {
        // arrange
        let app = StudyApp::new();

        // act
        let name = app.name();

        // assert
        assert_eq!(name, "Study");
    }

    #[test]
    fn test_app_module_id_returns_study() {
        // arrange
        let app = StudyApp::new();

        // act - uses AppModule trait method
        let id = app.id();

        // assert
        assert_eq!(id, "study");
    }
}

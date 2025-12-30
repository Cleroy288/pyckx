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
        Self {
            info: INTELLO_APP,
        }
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

//! JSON-based Open Question Repository Implementation
//!
//! Implements OpenQuestionRepository trait using JSON file storage.

use crate::domain::intello::OpenQuestionSet;
use crate::error::IntelloError;
use crate::infrastructure::repository::OpenQuestionRepository;
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;
use tracing::info;

// == JSON OPEN QUESTION REPOSITORY // ==

/// JSON file-based implementation of OpenQuestionRepository
#[derive(Debug)]
pub struct JsonOpenQuestionRepository {
    file_path: PathBuf,
}

impl JsonOpenQuestionRepository {
    /// Create a new JsonOpenQuestionRepository with the specified file path
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    /// Load all open question sets from the JSON file
    fn load_all(&self) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| IntelloError::storage(format!("Failed to read file: {}", e)))?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        serde_json::from_str(&content)
            .map_err(|e| IntelloError::storage(format!("Failed to parse JSON: {}", e)))
    }

    /// Save all open question sets to the JSON file
    fn save_all(&self, sets: &[OpenQuestionSet]) -> Result<(), IntelloError> {
        // Ensure directory exists
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| IntelloError::storage(format!("Failed to create directory: {}", e)))?;
        }

        let json = serde_json::to_string_pretty(sets)
            .map_err(|e| IntelloError::storage(format!("Failed to serialize JSON: {}", e)))?;

        fs::write(&self.file_path, json)
            .map_err(|e| IntelloError::storage(format!("Failed to write file: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl OpenQuestionRepository for JsonOpenQuestionRepository {
    async fn insert(&self, set: &OpenQuestionSet) -> Result<OpenQuestionSet, IntelloError> {
        let mut sets = self.load_all()?;

        // Add the new set
        sets.push(set.clone());

        // Save back to file
        self.save_all(&sets)?;

        info!(set_id = %set.id, "Open question set inserted");
        Ok(set.clone())
    }

    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<OpenQuestionSet>, IntelloError> {
        let sets = self.load_all()?;

        let found = sets
            .into_iter()
            .find(|s| s.id == set_id && s.user_id == user_id);

        if found.is_some() {
            info!(set_id = %set_id, "Open question set found");
        } else {
            info!(set_id = %set_id, "Open question set not found");
        }

        Ok(found)
    }

    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        let sets = self.load_all()?;

        let user_sets: Vec<OpenQuestionSet> = sets
            .into_iter()
            .filter(|s| s.user_id == user_id)
            .collect();

        info!(count = user_sets.len(), user_id = %user_id, "Retrieved user open question sets");
        Ok(user_sets)
    }
}

//! JSON-based QCM Repository Implementation
//!
//! Implements QcmRepository trait using JSON file storage.
//! Storage logic moved from apps/intello/qcm.rs.

use crate::domain::intello::QcmSet;
use crate::error::IntelloError;
use crate::infrastructure::repository::QcmRepository;
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;
use tracing::info;

// == JSON QCM REPOSITORY // ==

/// JSON file-based implementation of QcmRepository
pub struct JsonQcmRepository {
    file_path: PathBuf,
}

impl JsonQcmRepository {
    /// Create a new JsonQcmRepository with the specified file path
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    /// Load all QCM sets from the JSON file
    fn load_all(&self) -> Result<Vec<QcmSet>, IntelloError> {
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

    /// Save all QCM sets to the JSON file
    fn save_all(&self, sets: &[QcmSet]) -> Result<(), IntelloError> {
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
impl QcmRepository for JsonQcmRepository {
    async fn insert(&self, qcm_set: &QcmSet) -> Result<QcmSet, IntelloError> {
        let mut sets = self.load_all()?;

        // Add the new set
        sets.push(qcm_set.clone());

        // Save back to file
        self.save_all(&sets)?;

        info!(set_id = %qcm_set.id, "QCM set inserted");
        Ok(qcm_set.clone())
    }

    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<QcmSet>, IntelloError> {
        let sets = self.load_all()?;

        let found = sets
            .into_iter()
            .find(|s| s.id == set_id && s.user_id == user_id);

        if found.is_some() {
            info!(set_id = %set_id, "QCM set found");
        } else {
            info!(set_id = %set_id, "QCM set not found");
        }

        Ok(found)
    }

    async fn find_by_user(&self, user_id: &str) -> Result<Vec<QcmSet>, IntelloError> {
        let sets = self.load_all()?;

        let user_sets: Vec<QcmSet> = sets
            .into_iter()
            .filter(|s| s.user_id == user_id)
            .collect();

        info!(count = user_sets.len(), user_id = %user_id, "Retrieved user QCM sets");
        Ok(user_sets)
    }

    async fn update(&self, qcm_set: &QcmSet) -> Result<bool, IntelloError> {
        let mut sets = self.load_all()?;

        // Find and update the set
        let mut found = false;
        for set in sets.iter_mut() {
            if set.id == qcm_set.id && set.user_id == qcm_set.user_id {
                *set = qcm_set.clone();
                found = true;
                break;
            }
        }

        if !found {
            info!(set_id = %qcm_set.id, "QCM set not found for update");
            return Ok(false);
        }

        // Save back to file
        self.save_all(&sets)?;

        info!(set_id = %qcm_set.id, "QCM set updated");
        Ok(true)
    }

    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        let mut sets = self.load_all()?;
        let original_len = sets.len();

        // Remove the set if it exists and belongs to the user
        sets.retain(|s| !(s.id == set_id && s.user_id == user_id));

        if sets.len() == original_len {
            info!(set_id = %set_id, "QCM set not found for deletion");
            return Ok(false);
        }

        // Save back to file
        self.save_all(&sets)?;

        info!(set_id = %set_id, "QCM set deleted");
        Ok(true)
    }
}

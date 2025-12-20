//! JSON-based Flashcard Repository Implementation

use crate::domain::intello::FlashcardSet;
use crate::error::IntelloError;
use crate::infrastructure::repository::GameSetRepository;
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;
use tracing::info;


/// JSON file-based implementation of FlashcardRepository
#[derive(Debug)]
pub struct JsonFlashcardRepository {
    file_path: PathBuf,
}

impl JsonFlashcardRepository {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    fn load_all(&self) -> Result<Vec<FlashcardSet>, IntelloError> {
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

    fn save_all(&self, sets: &[FlashcardSet]) -> Result<(), IntelloError> {
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
impl GameSetRepository<FlashcardSet> for JsonFlashcardRepository {
    async fn insert(&self, set: &FlashcardSet) -> Result<FlashcardSet, IntelloError> {
        let mut sets = self.load_all()?;
        sets.push(set.clone());
        self.save_all(&sets)?;
        info!(set_id = %set.id, "Flashcard set inserted");
        Ok(set.clone())
    }

    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<FlashcardSet>, IntelloError> {
        let sets = self.load_all()?;
        let found = sets.into_iter().find(|s| s.id == set_id && s.user_id == user_id);
        Ok(found)
    }

    async fn find_by_user(&self, user_id: &str) -> Result<Vec<FlashcardSet>, IntelloError> {
        let sets = self.load_all()?;
        let user_sets: Vec<FlashcardSet> = sets.into_iter().filter(|s| s.user_id == user_id).collect();
        info!(count = user_sets.len(), user_id = %user_id, "Retrieved user flashcard sets");
        Ok(user_sets)
    }

    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        let mut sets = self.load_all()?;
        let initial_len = sets.len();
        sets.retain(|s| !(s.id == set_id && s.user_id == user_id));
        let deleted = sets.len() < initial_len;
        if deleted {
            self.save_all(&sets)?;
            info!(set_id = %set_id, "Flashcard set deleted");
        }
        Ok(deleted)
    }
}

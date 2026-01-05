//! Open Question Cache - Temporary storage for document content
//!
//! This module provides an in-memory cache to store document content
//! while users are answering open questions. The content is needed
//! when grading answers so the AI has context from the source material.

use std::collections::HashMap;
use std::sync::RwLock;

/// Key for the cache: (user_id, open_question_set_id)
type CacheKey = (String, String);

/// In-memory cache for open question document content
///
/// This cache stores the source document content temporarily while
/// users are answering questions. When they submit answers for grading,
/// we retrieve the content to provide context to the AI.
#[derive(Debug)]
pub struct OpenQuestionCache {
    cache: RwLock<HashMap<CacheKey, String>>,
}

impl OpenQuestionCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Store document content for an open question set
    ///
    /// # Arguments
    /// * `user_id` - The user who owns the question set
    /// * `set_id` - The open question set ID
    /// * `source_content` - Combined content from all documents
    pub fn store(&self, user_id: &str, set_id: &str, source_content: String) {
        let key = (user_id.to_string(), set_id.to_string());
        let mut cache = self.cache.write().unwrap();
        cache.insert(key, source_content);
    }

    /// Retrieve document content for an open question set
    ///
    /// # Arguments
    /// * `user_id` - The user who owns the question set
    /// * `set_id` - The open question set ID
    ///
    /// # Returns
    /// The cached source content, or None if not found
    pub fn get(&self, user_id: &str, set_id: &str) -> Option<String> {
        let key = (user_id.to_string(), set_id.to_string());
        let cache = self.cache.read().unwrap();
        cache.get(&key).cloned()
    }
}

impl Default for OpenQuestionCache {
    fn default() -> Self {
        Self::new()
    }
}

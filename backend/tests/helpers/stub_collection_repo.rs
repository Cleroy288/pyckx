//! Stub CollectionRepository for integration tests

use async_trait::async_trait;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

use LAPP::infra::CollectionRepository;
use LAPP::services::collection::collection_domain::{
    CollectionItemType, UserCollection,
};
use LAPP::services::collection::error_domain::CollectionError;

/// In-memory stub for CollectionRepository
pub struct StubCollectionRepository {
    collections: Mutex<Vec<UserCollection>>,
    next_id: AtomicI32,
}

impl StubCollectionRepository {
    /// Create an empty stub
    pub fn new() -> Self {
        Self {
            collections: Mutex::new(vec![]),
            next_id: AtomicI32::new(1),
        }
    }
}

#[async_trait]
impl CollectionRepository for StubCollectionRepository {
    async fn create(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError> {
        let mut items = self.collections.lock().unwrap();
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let coll = UserCollection {
            id,
            user_id: user_id.to_string(),
            collection_type,
            created_at: chrono::Utc::now(),
        };
        items.push(coll.clone());
        Ok(coll)
    }

    async fn delete(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError> {
        let mut items = self.collections.lock().unwrap();
        let before = items.len();
        items.retain(|c| {
            !(c.user_id == user_id
                && c.collection_type == collection_type)
        });
        Ok(items.len() < before)
    }

    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserCollection>, CollectionError> {
        let items = self.collections.lock().unwrap();
        Ok(items
            .iter()
            .filter(|c| c.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn find_by_type(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<Option<UserCollection>, CollectionError> {
        let items = self.collections.lock().unwrap();
        Ok(items
            .iter()
            .find(|c| {
                c.user_id == user_id
                    && c.collection_type == collection_type
            })
            .cloned())
    }

    async fn get_or_create(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<UserCollection, CollectionError> {
        // Check if exists first
        {
            let items = self.collections.lock().unwrap();
            if let Some(existing) = items.iter().find(|c| {
                c.user_id == user_id
                    && c.collection_type == collection_type
            }) {
                return Ok(existing.clone());
            }
        }
        // Create new
        self.create(user_id, collection_type).await
    }

    async fn exists(
        &self,
        user_id: &str,
        collection_type: CollectionItemType,
    ) -> Result<bool, CollectionError> {
        let items = self.collections.lock().unwrap();
        Ok(items.iter().any(|c| {
            c.user_id == user_id
                && c.collection_type == collection_type
        }))
    }
}

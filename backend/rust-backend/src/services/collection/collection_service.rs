//! Collection management operations
//!
//! CRUD operations for collections themselves.

use super::CollectionService;
use crate::services::collection::collection_domain::{
    CollectionItemType, UserCollection,
};
use crate::shared::AppResult;
use tracing::{info, instrument};

impl CollectionService {
    /// Add a new collection for a user
    #[instrument(skip(self), fields(user_id = %user_id, item_type = %item_type))]
    pub async fn add_collection(
        &self,
        user_id: &str,
        item_type: CollectionItemType,
    ) -> AppResult<UserCollection> {
        let collection =
            self.collection_repo.create(user_id, item_type).await?;
        info!(collection_id = collection.id, "Collection created");
        Ok(collection)
    }

    /// Delete a collection and all its items
    #[instrument(skip(self), fields(user_id = %user_id, item_type = %item_type))]
    pub async fn delete_collection(
        &self,
        user_id: &str,
        item_type: CollectionItemType,
    ) -> AppResult<bool> {
        // First, find the collection to get its ID
        let collection = self
            .collection_repo
            .find_by_type(user_id, item_type)
            .await?;

        if let Some(coll) = collection {
            // Delete all items in the collection first (only DVDs supported)
            if item_type == CollectionItemType::Dvd {
                self.dvd_repo.delete_by_collection(coll.id).await?;
            }

            // Then delete the collection itself
            let deleted =
                self.collection_repo.delete(user_id, item_type).await?;
            info!(collection_id = coll.id, "Collection and items deleted");
            Ok(deleted)
        } else {
            Ok(false)
        }
    }

    /// Get all collections for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_collections(
        &self,
        user_id: &str,
    ) -> AppResult<Vec<UserCollection>> {
        let collections = self.collection_repo.find_by_user(user_id).await?;
        info!(count = collections.len(), "Retrieved user collections");
        Ok(collections)
    }

    /// Get or create a collection for a user (idempotent)
    #[instrument(skip(self), fields(user_id = %user_id, item_type = %item_type))]
    pub async fn get_or_create_collection(
        &self,
        user_id: &str,
        item_type: CollectionItemType,
    ) -> AppResult<UserCollection> {
        let collection = self
            .collection_repo
            .get_or_create(user_id, item_type)
            .await?;
        Ok(collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::{CollectionRepository, DvdRepository};
    use crate::infra::{CreateDvd, UpdateDvd};
    use crate::services::collection::dvd_domain::Dvd;
    use crate::services::collection::error_domain::CollectionError;
    use async_trait::async_trait;
    use chrono::Utc;
    use std::sync::{Arc, Mutex};

    // -- Stub CollectionRepository --

    struct StubCollRepo {
        items: Mutex<Vec<UserCollection>>,
    }

    impl StubCollRepo {
        fn new(items: Vec<UserCollection>) -> Self {
            Self { items: Mutex::new(items) }
        }
    }

    #[async_trait]
    impl CollectionRepository for StubCollRepo {
        async fn create(
            &self,
            user_id: &str,
            collection_type: CollectionItemType,
        ) -> Result<UserCollection, CollectionError> {
            let mut items = self.items.lock().unwrap();
            let id = items.len() as i32 + 1;
            let coll = UserCollection {
                id,
                user_id: user_id.to_string(),
                collection_type,
                created_at: Utc::now(),
            };
            items.push(coll.clone());
            Ok(coll)
        }

        async fn delete(
            &self,
            user_id: &str,
            ct: CollectionItemType,
        ) -> Result<bool, CollectionError> {
            let mut items = self.items.lock().unwrap();
            let len = items.len();
            items.retain(|c| {
                !(c.user_id == user_id
                    && c.collection_type == ct)
            });
            Ok(items.len() < len)
        }

        async fn find_by_user(
            &self,
            user_id: &str,
        ) -> Result<Vec<UserCollection>, CollectionError>
        {
            let items = self.items.lock().unwrap();
            Ok(items
                .iter()
                .filter(|c| c.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn find_by_type(
            &self,
            user_id: &str,
            ct: CollectionItemType,
        ) -> Result<
            Option<UserCollection>,
            CollectionError,
        > {
            let items = self.items.lock().unwrap();
            Ok(items
                .iter()
                .find(|c| {
                    c.user_id == user_id
                        && c.collection_type == ct
                })
                .cloned())
        }

        async fn get_or_create(
            &self,
            user_id: &str,
            ct: CollectionItemType,
        ) -> Result<UserCollection, CollectionError> {
            {
                let items = self.items.lock().unwrap();
                if let Some(c) = items.iter().find(|c| {
                    c.user_id == user_id
                        && c.collection_type == ct
                }) {
                    return Ok(c.clone());
                }
            }
            self.create(user_id, ct).await
        }

        async fn exists(
            &self,
            user_id: &str,
            ct: CollectionItemType,
        ) -> Result<bool, CollectionError> {
            let items = self.items.lock().unwrap();
            Ok(items.iter().any(|c| {
                c.user_id == user_id
                    && c.collection_type == ct
            }))
        }
    }

    // -- Minimal DvdRepository stub (unused here) --

    struct StubDvdRepo;

    #[async_trait]
    impl DvdRepository for StubDvdRepo {
        async fn insert(
            &self,
            _: &CreateDvd,
        ) -> Result<Dvd, CollectionError> {
            unimplemented!()
        }
        async fn find_by_id(
            &self,
            _: &str,
            _: &str,
        ) -> Result<Dvd, CollectionError> {
            unimplemented!()
        }
        async fn find_all(
            &self,
            _: &str,
        ) -> Result<Vec<Dvd>, CollectionError> {
            Ok(vec![])
        }
        async fn find_by_collection(
            &self,
            _: i32,
        ) -> Result<Vec<Dvd>, CollectionError> {
            Ok(vec![])
        }
        async fn update(
            &self,
            _: &str,
            _: &str,
            _: &UpdateDvd,
        ) -> Result<Dvd, CollectionError> {
            unimplemented!()
        }
        async fn delete(
            &self,
            _: &str,
            _: &str,
        ) -> Result<bool, CollectionError> {
            Ok(true)
        }
        async fn exists_by_name(
            &self,
            _: &str,
            _: &str,
        ) -> Result<bool, CollectionError> {
            Ok(false)
        }
        async fn delete_by_collection(
            &self,
            _: i32,
        ) -> Result<usize, CollectionError> {
            Ok(0)
        }
    }

    /// Build a CollectionService for collection tests
    fn make_service(
        colls: Vec<UserCollection>,
    ) -> CollectionService {
        CollectionService::new(
            Arc::new(StubCollRepo::new(colls)),
            Arc::new(StubDvdRepo),
        )
    }

    /// Create a test UserCollection
    fn test_coll(
        id: i32,
        user_id: &str,
        ct: CollectionItemType,
    ) -> UserCollection {
        UserCollection {
            id,
            user_id: user_id.to_string(),
            collection_type: ct,
            created_at: Utc::now(),
        }
    }

    // -- add_collection tests --

    #[tokio::test]
    async fn test_add_collection_creates_new() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let coll = svc
            .add_collection(
                "usr-1",
                CollectionItemType::Dvd,
            )
            .await
            .unwrap();

        // assert
        assert_eq!(coll.user_id, "usr-1");
        assert_eq!(
            coll.collection_type,
            CollectionItemType::Dvd
        );
    }

    // -- delete_collection tests --

    #[tokio::test]
    async fn test_delete_collection_existing_returns_true(
    ) {
        // arrange
        let coll = test_coll(
            1,
            "usr-1",
            CollectionItemType::Dvd,
        );
        let svc = make_service(vec![coll]);

        // act
        let deleted = svc
            .delete_collection(
                "usr-1",
                CollectionItemType::Dvd,
            )
            .await
            .unwrap();

        // assert
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_delete_collection_missing_returns_false(
    ) {
        // arrange
        let svc = make_service(vec![]);

        // act
        let deleted = svc
            .delete_collection(
                "usr-1",
                CollectionItemType::Dvd,
            )
            .await
            .unwrap();

        // assert
        assert!(!deleted);
    }

    // -- get_user_collections tests --

    #[tokio::test]
    async fn test_get_user_collections_returns_all() {
        // arrange
        let colls = vec![
            test_coll(
                1,
                "usr-1",
                CollectionItemType::Dvd,
            ),
            test_coll(
                2,
                "usr-1",
                CollectionItemType::Book,
            ),
        ];
        let svc = make_service(colls);

        // act
        let result = svc
            .get_user_collections("usr-1")
            .await
            .unwrap();

        // assert
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_user_collections_empty() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let result = svc
            .get_user_collections("usr-1")
            .await
            .unwrap();

        // assert
        assert!(result.is_empty());
    }

    // -- get_or_create_collection tests --

    #[tokio::test]
    async fn test_get_or_create_existing_returns_same() {
        // arrange
        let coll = test_coll(
            1,
            "usr-1",
            CollectionItemType::Dvd,
        );
        let svc = make_service(vec![coll]);

        // act
        let result = svc
            .get_or_create_collection(
                "usr-1",
                CollectionItemType::Dvd,
            )
            .await
            .unwrap();

        // assert
        assert_eq!(result.id, 1);
    }

    #[tokio::test]
    async fn test_get_or_create_new_creates_collection() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let result = svc
            .get_or_create_collection(
                "usr-1",
                CollectionItemType::Dvd,
            )
            .await
            .unwrap();

        // assert
        assert_eq!(result.user_id, "usr-1");
        assert_eq!(
            result.collection_type,
            CollectionItemType::Dvd
        );
    }
}

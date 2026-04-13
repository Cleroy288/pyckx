//! DVD operations
//!
//! CRUD operations for DVDs within a collection.

use super::CollectionService;
use crate::infra::{CreateDvd, UpdateDvd};
use crate::services::collection::collection_domain::CollectionItemType;
use crate::services::collection::dvd_domain::Dvd;
use crate::services::collection::error_domain::CollectionError;
use crate::shared::AppResult;
use chrono::{DateTime, Utc};
use tracing::{info, instrument};

impl CollectionService {
    /// Add a new DVD to a user's collection
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self), fields(user_id = %user_id, dvd_name = %name))]
    pub async fn add_dvd(
        &self,
        user_id: &str,
        name: String,
        year: DateTime<Utc>,
        realisator: Option<String>,
        actors: Vec<String>,
        genre: Option<String>,
    ) -> AppResult<Dvd> {
        // Get or create the DVD collection
        let collection = self
            .get_or_create_collection(user_id, CollectionItemType::Dvd)
            .await?;

        let create_dvd = CreateDvd::new(
            name,
            year,
            realisator,
            actors,
            genre,
            user_id,
            collection.id,
        );

        let dvd = self.dvd_repo.insert(&create_dvd).await?;
        info!(dvd_id = %dvd.id, "DVD added to collection");
        Ok(dvd)
    }

    /// Get all DVDs for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_dvds(&self, user_id: &str) -> AppResult<Vec<Dvd>> {
        let dvds = self.dvd_repo.find_all(user_id).await?;
        info!(count = dvds.len(), "Retrieved user DVDs");
        Ok(dvds)
    }

    /// Find a specific DVD by ID
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    pub async fn find_dvd(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> AppResult<Dvd> {
        let dvd = self.dvd_repo.find_by_id(user_id, dvd_id).await?;
        Ok(dvd)
    }

    /// Update a DVD's information
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    pub async fn update_dvd(
        &self,
        user_id: &str,
        dvd_id: &str,
        name: Option<String>,
        year: Option<DateTime<Utc>>,
        realisator: Option<String>,
        actors: Option<Vec<String>>,
        genre: Option<String>,
    ) -> AppResult<Dvd> {
        let update = UpdateDvd {
            name,
            year,
            realisator,
            actors: actors.map(|a| a.join(", ")),
            genre,
        };

        let updated = self.dvd_repo.update(user_id, dvd_id, &update).await?;
        info!(dvd_id = %dvd_id, "DVD updated");
        Ok(updated)
    }

    /// Delete a DVD from a user's collection
    #[instrument(skip(self), fields(user_id = %user_id, dvd_id = %dvd_id))]
    pub async fn delete_dvd(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> AppResult<bool> {
        let deleted = self.dvd_repo.delete(user_id, dvd_id).await?;

        if !deleted {
            return Err(CollectionError::dvd_not_found(dvd_id).into());
        }

        info!(dvd_id = %dvd_id, "DVD deleted from collection");
        Ok(true)
    }

    /// Get all DVDs for a user (collection items)
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_collection_dvds(
        &self,
        user_id: &str,
    ) -> AppResult<Vec<Dvd>> {
        let dvds = self.dvd_repo.find_all(user_id).await?;
        Ok(dvds)
    }

    /// Get the count of DVDs in a user's collection
    #[allow(dead_code)]
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_dvd_count(&self, user_id: &str) -> AppResult<usize> {
        let dvds = self.dvd_repo.find_all(user_id).await?;
        Ok(dvds.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::{
        CollectionRepository, CreateDvd, DvdRepository,
        UpdateDvd,
    };
    use crate::services::collection::collection_domain::{
        CollectionItemType, UserCollection,
    };
    use async_trait::async_trait;
    use chrono::Utc;
    use std::sync::{Arc, Mutex};

    // -- Stub CollectionRepository for dvd tests --

    struct StubCollRepo;

    #[async_trait]
    impl CollectionRepository for StubCollRepo {
        async fn create(
            &self,
            user_id: &str,
            ct: CollectionItemType,
        ) -> Result<UserCollection, CollectionError> {
            Ok(UserCollection {
                id: 1,
                user_id: user_id.to_string(),
                collection_type: ct,
                created_at: Utc::now(),
            })
        }
        async fn delete(
            &self,
            _: &str,
            _: CollectionItemType,
        ) -> Result<bool, CollectionError> {
            Ok(true)
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<UserCollection>, CollectionError>
        {
            Ok(vec![])
        }
        async fn find_by_type(
            &self,
            _: &str,
            _: CollectionItemType,
        ) -> Result<
            Option<UserCollection>,
            CollectionError,
        > {
            Ok(None)
        }
        async fn get_or_create(
            &self,
            user_id: &str,
            ct: CollectionItemType,
        ) -> Result<UserCollection, CollectionError> {
            self.create(user_id, ct).await
        }
        async fn exists(
            &self,
            _: &str,
            _: CollectionItemType,
        ) -> Result<bool, CollectionError> {
            Ok(false)
        }
    }

    // -- Stub DvdRepository --

    struct StubDvdRepo {
        dvds: Mutex<Vec<Dvd>>,
    }

    impl StubDvdRepo {
        fn new(dvds: Vec<Dvd>) -> Self {
            Self { dvds: Mutex::new(dvds) }
        }
    }

    /// Build a test Dvd entity
    fn test_dvd(id: &str, user_id: &str) -> Dvd {
        let now = Utc::now();
        Dvd {
            id: id.to_string(),
            collection_id: 1,
            user_id: user_id.to_string(),
            name: "Test Movie".to_string(),
            year: now,
            realisator: None,
            actors: "Actor A".to_string(),
            genre: Some("Drama".to_string()),
            created_at: now,
            updated_at: now,
        }
    }

    #[async_trait]
    impl DvdRepository for StubDvdRepo {
        async fn insert(
            &self,
            dvd: &CreateDvd,
        ) -> Result<Dvd, CollectionError> {
            let now = Utc::now();
            let new = Dvd {
                id: "new-dvd-1".to_string(),
                collection_id: dvd.collection_id,
                user_id: dvd.user_id.clone(),
                name: dvd.name.clone(),
                year: dvd.year,
                realisator: dvd.realisator.clone(),
                actors: dvd.actors.clone(),
                genre: dvd.genre.clone(),
                created_at: now,
                updated_at: now,
            };
            self.dvds.lock().unwrap().push(new.clone());
            Ok(new)
        }

        async fn find_by_id(
            &self,
            user_id: &str,
            dvd_id: &str,
        ) -> Result<Dvd, CollectionError> {
            self.dvds
                .lock()
                .unwrap()
                .iter()
                .find(|d| {
                    d.id == dvd_id
                        && d.user_id == user_id
                })
                .cloned()
                .ok_or_else(|| {
                    CollectionError::dvd_not_found(dvd_id)
                })
        }

        async fn find_all(
            &self,
            user_id: &str,
        ) -> Result<Vec<Dvd>, CollectionError> {
            Ok(self
                .dvds
                .lock()
                .unwrap()
                .iter()
                .filter(|d| d.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn find_by_collection(
            &self,
            coll_id: i32,
        ) -> Result<Vec<Dvd>, CollectionError> {
            Ok(self
                .dvds
                .lock()
                .unwrap()
                .iter()
                .filter(|d| d.collection_id == coll_id)
                .cloned()
                .collect())
        }

        async fn update(
            &self,
            user_id: &str,
            dvd_id: &str,
            upd: &UpdateDvd,
        ) -> Result<Dvd, CollectionError> {
            let mut dvds = self.dvds.lock().unwrap();
            let dvd = dvds
                .iter_mut()
                .find(|d| {
                    d.id == dvd_id
                        && d.user_id == user_id
                })
                .ok_or_else(|| {
                    CollectionError::dvd_not_found(dvd_id)
                })?;
            if let Some(n) = &upd.name {
                dvd.name = n.clone();
            }
            if let Some(g) = &upd.genre {
                dvd.genre = Some(g.clone());
            }
            Ok(dvd.clone())
        }

        async fn delete(
            &self,
            user_id: &str,
            dvd_id: &str,
        ) -> Result<bool, CollectionError> {
            let mut dvds = self.dvds.lock().unwrap();
            let len = dvds.len();
            dvds.retain(|d| {
                !(d.id == dvd_id
                    && d.user_id == user_id)
            });
            Ok(dvds.len() < len)
        }

        async fn exists_by_name(
            &self,
            user_id: &str,
            name: &str,
        ) -> Result<bool, CollectionError> {
            Ok(self.dvds.lock().unwrap().iter().any(|d| {
                d.user_id == user_id && d.name == name
            }))
        }

        async fn delete_by_collection(
            &self,
            coll_id: i32,
        ) -> Result<usize, CollectionError> {
            let mut dvds = self.dvds.lock().unwrap();
            let before = dvds.len();
            dvds.retain(|d| d.collection_id != coll_id);
            Ok(before - dvds.len())
        }
    }

    /// Build a CollectionService for dvd tests
    fn make_service(dvds: Vec<Dvd>) -> CollectionService {
        CollectionService::new(
            Arc::new(StubCollRepo),
            Arc::new(StubDvdRepo::new(dvds)),
        )
    }

    // -- add_dvd tests --

    #[tokio::test]
    async fn test_add_dvd_success() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let dvd = svc
            .add_dvd(
                "usr-1",
                "Matrix".to_string(),
                Utc::now(),
                None,
                vec!["Keanu".to_string()],
                Some("Sci-Fi".to_string()),
            )
            .await
            .unwrap();

        // assert
        assert_eq!(dvd.name, "Matrix");
        assert_eq!(dvd.user_id, "usr-1");
    }

    // -- get_user_dvds tests --

    #[tokio::test]
    async fn test_get_user_dvds_returns_all() {
        // arrange
        let dvds = vec![
            test_dvd("d-1", "usr-1"),
            test_dvd("d-2", "usr-1"),
        ];
        let svc = make_service(dvds);

        // act
        let result =
            svc.get_user_dvds("usr-1").await.unwrap();

        // assert
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_user_dvds_empty() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let result =
            svc.get_user_dvds("usr-1").await.unwrap();

        // assert
        assert!(result.is_empty());
    }

    // -- find_dvd tests --

    #[tokio::test]
    async fn test_find_dvd_found() {
        // arrange
        let svc =
            make_service(vec![test_dvd("d-1", "usr-1")]);

        // act
        let dvd =
            svc.find_dvd("usr-1", "d-1").await.unwrap();

        // assert
        assert_eq!(dvd.id, "d-1");
    }

    #[tokio::test]
    async fn test_find_dvd_not_found_returns_error() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let result =
            svc.find_dvd("usr-1", "missing").await;

        // assert
        assert!(result.is_err());
    }

    // -- update_dvd tests --

    #[tokio::test]
    async fn test_update_dvd_changes_name() {
        // arrange
        let svc =
            make_service(vec![test_dvd("d-1", "usr-1")]);

        // act
        let dvd = svc
            .update_dvd(
                "usr-1",
                "d-1",
                Some("New Name".to_string()),
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // assert
        assert_eq!(dvd.name, "New Name");
    }

    #[tokio::test]
    async fn test_update_dvd_not_found_returns_error() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let result = svc
            .update_dvd(
                "usr-1", "missing", None, None, None,
                None, None,
            )
            .await;

        // assert
        assert!(result.is_err());
    }

    // -- delete_dvd tests --

    #[tokio::test]
    async fn test_delete_dvd_existing_returns_true() {
        // arrange
        let svc =
            make_service(vec![test_dvd("d-1", "usr-1")]);

        // act
        let result =
            svc.delete_dvd("usr-1", "d-1").await.unwrap();

        // assert
        assert!(result);
    }

    #[tokio::test]
    async fn test_delete_dvd_missing_returns_error() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let result =
            svc.delete_dvd("usr-1", "missing").await;

        // assert
        assert!(result.is_err());
    }

    // -- get_collection_dvds tests --

    #[tokio::test]
    async fn test_get_collection_dvds_returns_all() {
        // arrange
        let svc =
            make_service(vec![test_dvd("d-1", "usr-1")]);

        // act
        let result = svc
            .get_collection_dvds("usr-1")
            .await
            .unwrap();

        // assert
        assert_eq!(result.len(), 1);
    }

    // -- get_dvd_count tests --

    #[tokio::test]
    async fn test_get_dvd_count_returns_count() {
        // arrange
        let dvds = vec![
            test_dvd("d-1", "usr-1"),
            test_dvd("d-2", "usr-1"),
        ];
        let svc = make_service(dvds);

        // act
        let count =
            svc.get_dvd_count("usr-1").await.unwrap();

        // assert
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_get_dvd_count_empty_returns_zero() {
        // arrange
        let svc = make_service(vec![]);

        // act
        let count =
            svc.get_dvd_count("usr-1").await.unwrap();

        // assert
        assert_eq!(count, 0);
    }
}

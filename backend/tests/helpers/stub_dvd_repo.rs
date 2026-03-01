//! Stub DvdRepository for integration tests

use async_trait::async_trait;
use std::sync::Mutex;

use LAPP::infra::{CreateDvd, DvdRepository, UpdateDvd};
use LAPP::services::collection::dvd_domain::Dvd;
use LAPP::services::collection::error_domain::CollectionError;

/// In-memory stub for DvdRepository
pub struct StubDvdRepository {
    dvds: Mutex<Vec<Dvd>>,
}

impl StubDvdRepository {
    /// Create an empty stub
    pub fn new() -> Self {
        Self {
            dvds: Mutex::new(vec![]),
        }
    }

    /// Create a stub pre-seeded with DVDs
    pub fn with_dvds(dvds: Vec<Dvd>) -> Self {
        Self {
            dvds: Mutex::new(dvds),
        }
    }
}

#[async_trait]
impl DvdRepository for StubDvdRepository {
    async fn insert(
        &self,
        dvd: &CreateDvd,
    ) -> Result<Dvd, CollectionError> {
        let mut items = self.dvds.lock().unwrap();
        let now = chrono::Utc::now();
        let new_dvd = Dvd {
            id: uuid::Uuid::new_v4().to_string(),
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
        items.push(new_dvd.clone());
        Ok(new_dvd)
    }

    async fn find_by_id(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> Result<Dvd, CollectionError> {
        let items = self.dvds.lock().unwrap();
        items
            .iter()
            .find(|d| d.user_id == user_id && d.id == dvd_id)
            .cloned()
            .ok_or_else(|| CollectionError::dvd_not_found(dvd_id))
    }

    async fn find_all(
        &self,
        user_id: &str,
    ) -> Result<Vec<Dvd>, CollectionError> {
        let items = self.dvds.lock().unwrap();
        Ok(items
            .iter()
            .filter(|d| d.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn find_by_collection(
        &self,
        collection_id: i32,
    ) -> Result<Vec<Dvd>, CollectionError> {
        let items = self.dvds.lock().unwrap();
        Ok(items
            .iter()
            .filter(|d| d.collection_id == collection_id)
            .cloned()
            .collect())
    }

    async fn update(
        &self,
        user_id: &str,
        dvd_id: &str,
        update: &UpdateDvd,
    ) -> Result<Dvd, CollectionError> {
        let mut items = self.dvds.lock().unwrap();
        let dvd = items
            .iter_mut()
            .find(|d| d.user_id == user_id && d.id == dvd_id)
            .ok_or_else(|| CollectionError::dvd_not_found(dvd_id))?;
        if let Some(ref name) = update.name {
            dvd.name = name.clone();
        }
        if let Some(ref year) = update.year {
            dvd.year = *year;
        }
        if let Some(ref realisator) = update.realisator {
            dvd.realisator = Some(realisator.clone());
        }
        if let Some(ref actors) = update.actors {
            dvd.actors = actors.clone();
        }
        if let Some(ref genre) = update.genre {
            dvd.genre = Some(genre.clone());
        }
        dvd.updated_at = chrono::Utc::now();
        Ok(dvd.clone())
    }

    async fn delete(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> Result<bool, CollectionError> {
        let mut items = self.dvds.lock().unwrap();
        let before = items.len();
        items
            .retain(|d| !(d.user_id == user_id && d.id == dvd_id));
        Ok(items.len() < before)
    }

    async fn exists_by_name(
        &self,
        user_id: &str,
        name: &str,
    ) -> Result<bool, CollectionError> {
        let items = self.dvds.lock().unwrap();
        Ok(items
            .iter()
            .any(|d| d.user_id == user_id && d.name == name))
    }

    async fn delete_by_collection(
        &self,
        collection_id: i32,
    ) -> Result<usize, CollectionError> {
        let mut items = self.dvds.lock().unwrap();
        let before = items.len();
        items.retain(|d| d.collection_id != collection_id);
        Ok(before - items.len())
    }
}

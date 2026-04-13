//! DVD Repository Trait - Abstraction for DVD storage operations

use super::types::{CreateDvd, UpdateDvd};
use crate::services::collection::dvd_domain::Dvd;
use crate::services::collection::error_domain::CollectionError;
use async_trait::async_trait;

/// Repository trait for DVD persistence operations
#[async_trait]
pub trait DvdRepository: Send + Sync {
    async fn insert(&self, dvd: &CreateDvd) -> Result<Dvd, CollectionError>;
    async fn find_by_id(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> Result<Dvd, CollectionError>;
    async fn find_all(
        &self,
        user_id: &str,
    ) -> Result<Vec<Dvd>, CollectionError>;
    #[allow(dead_code)]
    async fn find_by_collection(
        &self,
        collection_id: i32,
    ) -> Result<Vec<Dvd>, CollectionError>;
    async fn update(
        &self,
        user_id: &str,
        dvd_id: &str,
        update: &UpdateDvd,
    ) -> Result<Dvd, CollectionError>;
    async fn delete(
        &self,
        user_id: &str,
        dvd_id: &str,
    ) -> Result<bool, CollectionError>;
    async fn exists_by_name(
        &self,
        user_id: &str,
        name: &str,
    ) -> Result<bool, CollectionError>;
    async fn delete_by_collection(
        &self,
        collection_id: i32,
    ) -> Result<usize, CollectionError>;
}

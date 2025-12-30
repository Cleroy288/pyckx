//! CollectionService struct and constructor

use crate::infra::{CollectionRepository, DvdRepository};
use std::fmt;
use std::sync::Arc;
use tracing::info;

/// Service for managing user collections (DVDs)
///
/// This service uses generic repositories that implement the respective traits,
/// allowing for different storage backends (Supabase, mock, etc.)
pub struct CollectionService {
    /// Repository for collection management
    pub(super) collection_repo: Arc<dyn CollectionRepository>,
    /// Repository for DVD persistence
    pub(super) dvd_repo: Arc<dyn DvdRepository>,
}

impl CollectionService {
    /// Create a new CollectionService with the given repositories
    pub fn new(
        collection_repo: Arc<dyn CollectionRepository>,
        dvd_repo: Arc<dyn DvdRepository>,
    ) -> Self {
        info!("CollectionService initialized");
        Self {
            collection_repo,
            dvd_repo,
        }
    }
}

impl fmt::Debug for CollectionService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CollectionService").finish()
    }
}

impl fmt::Display for CollectionService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CollectionService")
    }
}

//! CollectionApp - App instance for managing collections

use super::collection_domain::CollectionItemType;
use crate::services::app_registry::registry_domain::{AppInstance, AppModule};

// == COLLECTION APP METADATA // ==

/// Collection app metadata
const COLLECTION_APP: AppInstance = AppInstance::new(
    "collection",
    "Collection",
    "Create and manage collections of items, bookmarks, and resources",
);

/// Available collection item types
#[allow(dead_code)] // Used in tests, for future app registry
const AVAILABLE_ITEM_TYPES: &[CollectionItemType] = &[
    CollectionItemType::Dvd,
    // CollectionItemType::Book, // Future
];

// == COLLECTION APP STRUCT // ==

/// CollectionApp - Manages user collections
#[derive(Debug, Clone)]
pub struct CollectionApp {
    info: AppInstance,
}

impl CollectionApp {
    pub fn new() -> Self {
        Self {
            info: COLLECTION_APP,
        }
    }

    /// Get the list of available collection item types
    #[allow(dead_code)] // Used in tests, for future app registry
    pub fn available_item_types(&self) -> &'static [CollectionItemType] {
        AVAILABLE_ITEM_TYPES
    }

    /// Check if a collection item type is supported
    #[allow(dead_code)] // Used in tests, for future app registry
    pub fn supports_item_type(&self, item_type: &CollectionItemType) -> bool {
        AVAILABLE_ITEM_TYPES.contains(item_type)
    }
}

impl Default for CollectionApp {
    fn default() -> Self {
        Self::new()
    }
}

impl AppModule for CollectionApp {
    fn info(&self) -> &AppInstance {
        &self.info
    }
}

// == UNIT TESTS // ==

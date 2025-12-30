//! Collection Response DTOs

use serde::Serialize;

/// Response for a single DVD
#[derive(Debug, Serialize)]
pub struct DvdResponse {
    pub id: String,
    pub name: String,
    pub year: i32,
    pub realisator: Option<String>,
    pub actors: Vec<String>,
    pub genre: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Response for DVD list
#[derive(Debug, Serialize)]
pub struct DvdListResponse {
    pub dvds: Vec<DvdResponse>,
    pub count: usize,
}

/// Response for successful DVD operations
#[derive(Debug, Serialize)]
pub struct DvdSuccessResponse {
    pub message: String,
    pub dvd: DvdResponse,
}

/// Response for successful deletion
#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub message: String,
    pub deleted: bool,
}

#[allow(dead_code)]
impl DeleteResponse {
    pub fn success() -> Self {
        Self {
            message: "Item deleted successfully".to_string(),
            deleted: true,
        }
    }

    pub fn dvd_success() -> Self {
        Self {
            message: "DVD deleted successfully".to_string(),
            deleted: true,
        }
    }

    pub fn collection_success() -> Self {
        Self {
            message: "Collection deleted successfully".to_string(),
            deleted: true,
        }
    }
}

/// Response for a collection
#[derive(Debug, Serialize)]
pub struct CollectionResponse {
    pub id: i32,
    pub item_type: String,
    pub display_name: String,
    pub created_at: String,
}

/// Response for collection list
#[derive(Debug, Serialize)]
pub struct CollectionListResponse {
    pub collections: Vec<CollectionResponse>,
    pub count: usize,
}

/// Response for collection items (DVDs only)
#[derive(Debug, Serialize)]
pub struct CollectionItemsResponse {
    pub item_type: String,
    pub items: serde_json::Value,
    pub count: usize,
}

/// Response for successful collection creation
#[derive(Debug, Serialize)]
pub struct CollectionSuccessResponse {
    pub message: String,
    pub collection: CollectionResponse,
}

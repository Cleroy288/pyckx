//! Collection DTO Conversions

use crate::domain::{Dvd, UserCollection};
use super::response::{
    DvdResponse, DvdListResponse, DvdSuccessResponse, 
    CollectionResponse, CollectionListResponse, CollectionItemsResponse, CollectionSuccessResponse
};

impl From<Dvd> for DvdResponse {
    fn from(dvd: Dvd) -> Self {
        Self {
            id: dvd.id.clone(),
            name: dvd.name.clone(),
            year: dvd.release_year(),
            realisator: dvd.realisator.clone(),
            actors: dvd.actors_vec(),
            genre: dvd.genre.clone(),
            created_at: dvd.created_at.to_rfc3339(),
            updated_at: dvd.updated_at.to_rfc3339(),
        }
    }
}

impl From<&Dvd> for DvdResponse {
    fn from(dvd: &Dvd) -> Self {
        Self {
            id: dvd.id.clone(),
            name: dvd.name.clone(),
            year: dvd.release_year(),
            realisator: dvd.realisator.clone(),
            actors: dvd.actors_vec(),
            genre: dvd.genre.clone(),
            created_at: dvd.created_at.to_rfc3339(),
            updated_at: dvd.updated_at.to_rfc3339(),
        }
    }
}

impl DvdListResponse {
    pub fn from_dvds(dvds: Vec<Dvd>) -> Self {
        let count = dvds.len();
        Self {
            dvds: dvds.into_iter().map(DvdResponse::from).collect(),
            count,
        }
    }
}

impl DvdSuccessResponse {
    pub fn created(dvd: Dvd) -> Self {
        Self {
            message: "DVD added successfully".to_string(),
            dvd: DvdResponse::from(dvd),
        }
    }

    pub fn updated(dvd: Dvd) -> Self {
        Self {
            message: "DVD updated successfully".to_string(),
            dvd: DvdResponse::from(dvd),
        }
    }
}

impl From<UserCollection> for CollectionResponse {
    fn from(collection: UserCollection) -> Self {
        Self {
            id: collection.id,
            item_type: collection.collection_type.to_string(),
            display_name: collection.display_name(),
            created_at: collection.created_at.to_rfc3339(),
        }
    }
}

impl CollectionListResponse {
    pub fn from_collections(collections: Vec<UserCollection>) -> Self {
        let count = collections.len();
        Self {
            collections: collections.into_iter().map(CollectionResponse::from).collect(),
            count,
        }
    }
}

impl CollectionItemsResponse {
    pub fn from_dvds(dvds: Vec<Dvd>) -> Self {
        let count = dvds.len();
        let items_json = serde_json::to_value(
            dvds.into_iter().map(DvdResponse::from).collect::<Vec<_>>()
        ).unwrap_or_default();

        Self {
            item_type: "dvd".to_string(),
            items: items_json,
            count,
        }
    }
}

impl CollectionSuccessResponse {
    pub fn created(collection: UserCollection) -> Self {
        Self {
            message: "Collection created successfully".to_string(),
            collection: CollectionResponse::from(collection),
        }
    }
}

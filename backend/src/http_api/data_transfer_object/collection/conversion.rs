//! Collection DTO Conversions

use super::response::{
    CollectionItemsResponse, CollectionListResponse, CollectionResponse,
    CollectionSuccessResponse, DvdListResponse, DvdResponse,
    DvdSuccessResponse,
};
use crate::services::collection::collection_domain::UserCollection;
use crate::services::collection::dvd_domain::Dvd;

impl From<Dvd> for DvdResponse {
    fn from(dvd: Dvd) -> Self {
        Self {
            id: dvd.id.to_string(),
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
            id: dvd.id.to_string(),
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
            collections: collections
                .into_iter()
                .map(CollectionResponse::from)
                .collect(),
            count,
        }
    }
}

impl CollectionItemsResponse {
    pub fn from_dvds(dvds: Vec<Dvd>) -> Self {
        let count = dvds.len();
        let items_json = serde_json::to_value(
            dvds.into_iter().map(DvdResponse::from).collect::<Vec<_>>(),
        )
        .unwrap_or_default();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::collection::collection_domain::CollectionItemType;
    use chrono::Utc;

    /// Build a test Dvd with known values
    fn test_dvd() -> Dvd {
        let now = Utc::now();
        Dvd {
            id: "dvd-1".into(),
            collection_id: 1,
            user_id: "user-1".into(),
            name: "Inception".into(),
            year: now,
            realisator: Some("Nolan".into()),
            actors: "Leo, Tom".into(),
            genre: Some("Sci-Fi".into()),
            created_at: now,
            updated_at: now,
        }
    }

    /// Build a test UserCollection with known values
    fn test_collection() -> UserCollection {
        UserCollection {
            id: 42,
            user_id: "user-1".into(),
            collection_type: CollectionItemType::Dvd,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_dvd_response_from_dvd_maps_id() {
        // arrange
        let dvd = test_dvd();

        // act
        let resp = DvdResponse::from(dvd);

        // assert
        assert_eq!(resp.id, "dvd-1");
    }

    #[test]
    fn test_dvd_response_from_dvd_maps_name() {
        // arrange
        let dvd = test_dvd();

        // act
        let resp = DvdResponse::from(dvd);

        // assert
        assert_eq!(resp.name, "Inception");
    }

    #[test]
    fn test_dvd_response_from_dvd_maps_actors() {
        // arrange
        let dvd = test_dvd();

        // act
        let resp = DvdResponse::from(dvd);

        // assert
        assert_eq!(resp.actors, vec!["Leo", "Tom"]);
    }

    #[test]
    fn test_dvd_response_from_ref_maps_fields() {
        // arrange
        let dvd = test_dvd();

        // act
        let resp = DvdResponse::from(&dvd);

        // assert
        assert_eq!(resp.id, "dvd-1");
        assert_eq!(resp.genre, Some("Sci-Fi".into()));
    }

    #[test]
    fn test_dvd_list_response_from_dvds_count() {
        // arrange
        let dvds = vec![test_dvd(), test_dvd()];

        // act
        let resp = DvdListResponse::from_dvds(dvds);

        // assert
        assert_eq!(resp.count, 2);
        assert_eq!(resp.dvds.len(), 2);
    }

    #[test]
    fn test_dvd_list_response_from_empty() {
        // arrange
        let dvds: Vec<Dvd> = vec![];

        // act
        let resp = DvdListResponse::from_dvds(dvds);

        // assert
        assert_eq!(resp.count, 0);
    }

    #[test]
    fn test_dvd_success_created_message() {
        // arrange
        let dvd = test_dvd();

        // act
        let resp = DvdSuccessResponse::created(dvd);

        // assert
        assert_eq!(resp.message, "DVD added successfully");
    }

    #[test]
    fn test_dvd_success_updated_message() {
        // arrange
        let dvd = test_dvd();

        // act
        let resp = DvdSuccessResponse::updated(dvd);

        // assert
        assert_eq!(resp.message, "DVD updated successfully");
    }

    #[test]
    fn test_collection_response_from_maps_fields() {
        // arrange
        let col = test_collection();
        let created = col.created_at.to_rfc3339();

        // act
        let resp = CollectionResponse::from(col);

        // assert
        assert_eq!(resp.id, 42);
        assert_eq!(resp.item_type, "dvd");
        assert_eq!(resp.display_name, "DVD Collection");
        assert_eq!(resp.created_at, created);
    }

    #[test]
    fn test_collection_list_from_collections_count() {
        // arrange
        let cols = vec![test_collection(), test_collection()];

        // act
        let resp = CollectionListResponse::from_collections(cols);

        // assert
        assert_eq!(resp.count, 2);
    }

    #[test]
    fn test_collection_items_from_dvds_type() {
        // arrange
        let dvds = vec![test_dvd()];

        // act
        let resp = CollectionItemsResponse::from_dvds(dvds);

        // assert
        assert_eq!(resp.item_type, "dvd");
        assert_eq!(resp.count, 1);
    }

    #[test]
    fn test_collection_success_created_message() {
        // arrange
        let col = test_collection();

        // act
        let resp = CollectionSuccessResponse::created(col);

        // assert
        assert_eq!(resp.message, "Collection created successfully");
    }
}

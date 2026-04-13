//! Collection DTOs - Request and Response types for collection API
//!
//! Supports DVD collections.

mod conversion;
mod request;
mod response;

pub use request::{AddDvdRequest, CreateCollectionRequest, UpdateDvdRequest};
pub use response::{
    CollectionItemsResponse, CollectionListResponse, CollectionSuccessResponse,
    DeleteResponse, DvdListResponse, DvdResponse, DvdSuccessResponse,
};

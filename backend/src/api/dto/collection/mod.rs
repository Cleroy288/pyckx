//! Collection DTOs - Request and Response types for collection API
//!
//! Supports DVD collections.

mod request;
mod response;
mod conversion;

pub use request::{CreateCollectionRequest, AddDvdRequest, UpdateDvdRequest};
pub use response::{
    DvdResponse, DvdListResponse, DvdSuccessResponse, DeleteResponse,
    CollectionListResponse, CollectionItemsResponse, CollectionSuccessResponse
};

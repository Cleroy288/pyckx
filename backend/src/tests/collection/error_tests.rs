use crate::services::collection::error_domain::CollectionError;
use actix_web::http::StatusCode;

#[test]
fn test_dvd_not_found_error() {
    let err = CollectionError::dvd_not_found("dvd-123");
    assert_eq!(err.code(), "COLLECTION_DVD_NOT_FOUND");
    assert_eq!(err.status(), StatusCode::NOT_FOUND);
    assert!(err.to_string().contains("dvd-123"));
}

#[test]
fn test_dvd_duplicate_error() {
    let err = CollectionError::dvd_duplicate("Inception");
    assert_eq!(err.code(), "COLLECTION_DVD_DUPLICATE");
    assert_eq!(err.status(), StatusCode::CONFLICT);
    assert!(err.to_string().contains("Inception"));
}

#[test]
fn test_user_not_found_error() {
    let err = CollectionError::user_not_found("user-456");
    assert_eq!(err.code(), "COLLECTION_USER_NOT_FOUND");
    assert_eq!(err.status(), StatusCode::NOT_FOUND);
    assert!(err.to_string().contains("user-456"));
}

#[test]
fn test_storage_error() {
    let err = CollectionError::storage_error("Failed to write file");
    assert_eq!(err.code(), "COLLECTION_STORAGE_ERROR");
    assert_eq!(err.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert!(err.to_string().contains("Failed to write file"));
}

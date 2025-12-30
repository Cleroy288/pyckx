use crate::services::intello::error_domain::IntelloError;
use actix_web::http::StatusCode;

#[test]
fn test_game_set_not_found_error() {
    let err = IntelloError::game_not_found("keywords", "kw-456");
    assert_eq!(err.code(), "INTELLO_GAME_SET_NOT_FOUND");
    assert_eq!(err.status(), StatusCode::NOT_FOUND);
    assert!(err.to_string().contains("keywords"));
    assert!(err.to_string().contains("kw-456"));
}

#[test]
fn test_validation_failed_error() {
    let err = IntelloError::validation("name", "Name cannot be empty");
    assert_eq!(err.code(), "INTELLO_VALIDATION_FAILED");
    assert_eq!(err.status(), StatusCode::BAD_REQUEST);
    assert!(err.to_string().contains("name"));
}

#[test]
fn test_storage_error() {
    let err = IntelloError::storage("Failed to write file");
    assert_eq!(err.code(), "INTELLO_STORAGE_ERROR");
    assert_eq!(err.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert!(err.to_string().contains("Failed to write file"));
}

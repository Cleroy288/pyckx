use crate::services::collection::error_domain::CollectionError;
use actix_web::http::StatusCode;

/// Table-driven: each CollectionError variant maps to the
/// correct code and status.
#[test]
fn test_collection_error_code_and_status_mapping() {
    // Arrange
    let cases: Vec<(
        &str,
        CollectionError,
        &str,
        StatusCode,
    )> = vec![
        (
            "dvd_not_found",
            CollectionError::dvd_not_found("dvd-123"),
            "COLLECTION_DVD_NOT_FOUND",
            StatusCode::NOT_FOUND,
        ),
        (
            "dvd_duplicate",
            CollectionError::dvd_duplicate("Inception"),
            "COLLECTION_DVD_DUPLICATE",
            StatusCode::CONFLICT,
        ),
        (
            "user_not_found",
            CollectionError::user_not_found("user-456"),
            "COLLECTION_USER_NOT_FOUND",
            StatusCode::NOT_FOUND,
        ),
        (
            "storage_error",
            CollectionError::storage_error("disk full"),
            "COLLECTION_STORAGE_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    ];

    for (label, err, code, status) in cases {
        // Act + Assert
        assert_eq!(
            err.code(),
            code,
            "Case '{}': wrong code",
            label,
        );
        assert_eq!(
            err.status(),
            status,
            "Case '{}': wrong status",
            label,
        );
    }
}

/// Table-driven: each error message includes its context.
#[test]
fn test_collection_error_message_contains_context() {
    // Arrange
    let cases: Vec<(&str, CollectionError, &str)> = vec![
        (
            "dvd_not_found",
            CollectionError::dvd_not_found("dvd-123"),
            "dvd-123",
        ),
        (
            "dvd_duplicate",
            CollectionError::dvd_duplicate("Inception"),
            "Inception",
        ),
        (
            "user_not_found",
            CollectionError::user_not_found("user-456"),
            "user-456",
        ),
        (
            "storage_error",
            CollectionError::storage_error("Failed to write"),
            "Failed to write",
        ),
    ];

    for (label, err, expected) in cases {
        // Act
        let msg = err.to_string();

        // Assert
        assert!(
            msg.contains(expected),
            "Case '{}': expected '{}' in '{}'",
            label,
            expected,
            msg,
        );
    }
}

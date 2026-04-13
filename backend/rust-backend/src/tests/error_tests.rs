use crate::services::error_domain::StudyError;
use actix_web::http::StatusCode;

/// Table-driven: each StudyError variant maps to the
/// correct code and status.
#[test]
fn test_study_error_code_and_status_mapping() {
    // Arrange
    let cases: Vec<(
        &str,
        StudyError,
        &str,
        StatusCode,
    )> = vec![
        (
            "game_not_found",
            StudyError::game_not_found("keywords", "kw-456"),
            "STUDY_GAME_SET_NOT_FOUND",
            StatusCode::NOT_FOUND,
        ),
        (
            "validation_failed",
            StudyError::validation("name", "empty"),
            "STUDY_VALIDATION_FAILED",
            StatusCode::BAD_REQUEST,
        ),
        (
            "storage_error",
            StudyError::storage("disk full"),
            "STUDY_STORAGE_ERROR",
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

/// game_not_found message includes game type and set id.
#[test]
fn test_study_error_game_not_found_message_content() {
    // Arrange
    let err =
        StudyError::game_not_found("keywords", "kw-456");

    // Act
    let msg = err.to_string();

    // Assert
    assert!(
        msg.contains("keywords") && msg.contains("kw-456"),
        "Message should contain game type and set id: {}",
        msg,
    );
}

/// validation error message includes the field name.
#[test]
fn test_study_error_validation_message_content() {
    // Arrange
    let err = StudyError::validation(
        "name",
        "Name cannot be empty",
    );

    // Act
    let msg = err.to_string();

    // Assert
    assert!(
        msg.contains("name"),
        "Message should contain field name: {}",
        msg,
    );
}

/// storage error message includes the detail string.
#[test]
fn test_study_error_storage_message_content() {
    // Arrange
    let err = StudyError::storage("Failed to write file");

    // Act
    let msg = err.to_string();

    // Assert
    assert!(
        msg.contains("Failed to write file"),
        "Message should contain detail: {}",
        msg,
    );
}

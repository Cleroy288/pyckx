use crate::services::crud::crud_service::validate_user_id;

#[test]
fn test_validate_user_id_with_empty_string_returns_err() {
    // Arrange
    let input = "";

    // Act
    let result = validate_user_id(input);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_validate_user_id_with_whitespace_returns_err() {
    // Arrange
    let input = "   ";

    // Act
    let result = validate_user_id(input);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_validate_user_id_with_valid_id_returns_ok() {
    // Arrange
    let input = "user-123";

    // Act
    let result = validate_user_id(input);

    // Assert
    assert!(result.is_ok());
}

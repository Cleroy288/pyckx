use crate::services::intello::crud_service::validate_user_id;

#[test]
fn test_validate_user_id_empty() {
    assert!(validate_user_id("").is_err());
    assert!(validate_user_id("   ").is_err());
}

#[test]
fn test_validate_user_id_valid() {
    assert!(validate_user_id("user-123").is_ok());
}

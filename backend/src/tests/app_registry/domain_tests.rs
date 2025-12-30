use crate::services::app_registry::registry_domain::UserApp;

#[test]
fn test_user_app_new() {
    let user_app = UserApp::new(1, "user-123", 5);

    assert_eq!(user_app.id, 1);
    assert_eq!(user_app.user_id, "user-123");
    assert_eq!(user_app.app_id, 5);
}

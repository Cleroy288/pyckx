//! App domain types — apps and user-app associations

use serde::{Deserialize, Serialize};

/// Single app data from API
#[derive(Debug, Clone, Deserialize)]
pub struct AppData {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// List of apps response
#[derive(Debug, Clone, Deserialize)]
pub struct AppListResponse {
    pub apps: Vec<AppData>,
    pub count: usize,
}

/// User-app association
#[derive(Debug, Clone, Deserialize)]
pub struct UserAppResponse {
    pub id: i32,
    pub app_id: i32,
    pub added_at: String,
}

/// Success response with user-app
#[derive(Debug, Clone, Deserialize)]
pub struct UserAppSuccessResponse {
    pub message: String,
    pub user_app: UserAppResponse,
}

/// Request to add app to user
#[derive(Debug, Clone, Serialize)]
pub struct AddUserAppRequest {
    pub app_name: String,
}

/// Generic success response
#[derive(Debug, Clone, Deserialize)]
pub struct SuccessResponse {
    pub message: String,
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_app_data() {
        let json = r#"{
            "id": 1,
            "name": "intello",
            "description": "Learn",
            "created_at": "2024-01-01",
            "updated_at": "2024-01-02"
        }"#;
        let a: AppData =
            serde_json::from_str(json).unwrap();
        assert_eq!(a.id, 1);
        assert_eq!(a.name, "intello");
    }

    #[test]
    fn test_deserialize_app_data_null_description() {
        let json = r#"{
            "id": 2, "name": "x",
            "description": null,
            "created_at": "", "updated_at": ""
        }"#;
        let a: AppData =
            serde_json::from_str(json).unwrap();
        assert!(a.description.is_none());
    }

    #[test]
    fn test_deserialize_user_app_response() {
        let json = r#"{
            "id": 5, "app_id": 3,
            "added_at": "2024-06-01"
        }"#;
        let u: UserAppResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(u.app_id, 3);
    }

    #[test]
    fn test_deserialize_success_response() {
        let json =
            r#"{"message":"ok","success":true}"#;
        let s: SuccessResponse =
            serde_json::from_str(json).unwrap();
        assert!(s.success);
    }

    #[test]
    fn test_serialize_add_user_app_request() {
        let req = AddUserAppRequest {
            app_name: "intello".into(),
        };
        let json =
            serde_json::to_string(&req).unwrap();
        assert!(json.contains("intello"));
    }
}

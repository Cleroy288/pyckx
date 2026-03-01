//! Collection domain types — DVDs

use serde::{Deserialize, Serialize};

/// DVD data from API
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Dvd {
    pub id: String,
    pub name: String,
    pub year: i32,
    pub realisator: Option<String>,
    pub actors: Vec<String>,
    pub genre: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// DVD list response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DvdListResponse {
    pub dvds: Vec<Dvd>,
    pub count: usize,
}

/// Request to add a new DVD
#[derive(Debug, Clone, Serialize)]
pub struct AddDvdRequest {
    pub name: String,
    pub year: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realisator: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
}

/// Request to update a DVD
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDvdRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realisator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
}

/// DVD success response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DvdSuccessResponse {
    pub message: String,
    pub dvd: Dvd,
}

/// Delete response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteResponse {
    pub message: String,
    pub deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_dvd() {
        let json = r#"{
            "id": "abc", "name": "Matrix",
            "year": 1999, "realisator": "Wachowski",
            "actors": ["Keanu"],
            "genre": "Sci-Fi",
            "created_at": "", "updated_at": ""
        }"#;
        let d: Dvd =
            serde_json::from_str(json).unwrap();
        assert_eq!(d.name, "Matrix");
        assert_eq!(d.year, 1999);
    }

    #[test]
    fn test_deserialize_dvd_optional_nulls() {
        let json = r#"{
            "id": "x", "name": "Film",
            "year": 2024, "realisator": null,
            "actors": [],
            "genre": null,
            "created_at": "", "updated_at": ""
        }"#;
        let d: Dvd =
            serde_json::from_str(json).unwrap();
        assert!(d.realisator.is_none());
        assert!(d.genre.is_none());
    }

    #[test]
    fn test_serialize_add_dvd_skips_empty() {
        let req = AddDvdRequest {
            name: "X".into(),
            year: "2024".into(),
            realisator: None,
            actors: vec![],
            genre: None,
        };
        let json =
            serde_json::to_string(&req).unwrap();
        assert!(!json.contains("realisator"));
        assert!(!json.contains("actors"));
    }

    #[test]
    fn test_deserialize_delete_response() {
        let json =
            r#"{"message":"done","deleted":true}"#;
        let d: DeleteResponse =
            serde_json::from_str(json).unwrap();
        assert!(d.deleted);
    }
}

//! Collection domain types — DVD models and API DTOs.

use serde::{Deserialize, Serialize};

/// A DVD record as returned by the backend.
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

/// List response wrapping DVDs and total count.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DvdListResponse {
    pub dvds: Vec<Dvd>,
    pub count: usize,
}

/// Success response wrapping a single DVD.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DvdSuccessResponse {
    pub message: String,
    pub dvd: Dvd,
}

/// Delete response with success flag.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteResponse {
    pub message: String,
    pub deleted: bool,
}

/// Payload to create a DVD.
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

/// Payload to partially update a DVD.
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

impl Dvd {
    /// Return an empty DVD suitable as a form default.
    pub fn empty() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            year: 0,
            realisator: None,
            actors: Vec::new(),
            genre: None,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_returns_blank_dvd() {
        let dvd = Dvd::empty();
        assert!(dvd.name.is_empty());
        assert_eq!(dvd.year, 0);
        assert!(dvd.actors.is_empty());
    }

    #[test]
    fn test_deserialize_dvd_with_nulls() {
        let json = r#"{
            "id": "x", "name": "Film", "year": 2024,
            "realisator": null, "actors": [],
            "genre": null,
            "created_at": "", "updated_at": ""
        }"#;
        let d: Dvd = serde_json::from_str(json).unwrap();
        assert!(d.realisator.is_none());
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
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("realisator"));
        assert!(!json.contains("actors"));
    }
}

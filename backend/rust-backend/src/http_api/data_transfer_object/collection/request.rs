//! Collection Request DTOs

use crate::services::collection::collection_domain::CollectionItemType;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use validator::Validate;

/// Result of parsing an optional year string
type ParseYearResult = Result<Option<DateTime<Utc>>, String>;

/// Request body for creating a new collection
#[derive(Debug, Deserialize)]
pub struct CreateCollectionRequest {
    /// Collection type (dvd, book)
    pub item_type: String,
}

impl CreateCollectionRequest {
    /// Parse the item_type string into CollectionItemType
    pub fn parse_type(&self) -> Result<CollectionItemType, String> {
        self.item_type
            .parse::<CollectionItemType>()
    }
}

/// Request body for adding a new DVD
#[derive(Debug, Deserialize, Validate)]
pub struct AddDvdRequest {
    /// DVD name (required, must be unique per user)
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    /// Release year (required, format: YYYY or YYYY-MM-DD)
    #[validate(length(min = 4, message = "Year is required"))]
    pub year: String,

    /// Director name (optional)
    #[serde(default)]
    pub realisator: Option<String>,

    /// List of actors (optional)
    #[serde(default)]
    pub actors: Vec<String>,

    /// Genre (optional)
    #[serde(default)]
    pub genre: Option<String>,
}

impl AddDvdRequest {
    /// Parse the year string into a `DateTime<Utc>`
    pub fn parse_year(&self) -> Result<DateTime<Utc>, String> {
        // Try parsing as full date first (YYYY-MM-DD)
        if let Ok(date) = NaiveDate::parse_from_str(&self.year, "%Y-%m-%d") {
            return Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }

        // Try parsing as year only (YYYY)
        if let Ok(year) = self.year.parse::<i32>() {
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                return Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
            }
        }

        Err(format!("Invalid year format: {}", self.year))
    }
}

/// Request body for updating a DVD
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateDvdRequest {
    /// New name (optional)
    pub name: Option<String>,

    /// New release year (optional, format: YYYY or YYYY-MM-DD)
    pub year: Option<String>,

    /// New director (optional)
    pub realisator: Option<String>,

    /// New actors list (optional)
    pub actors: Option<Vec<String>>,

    /// New genre (optional)
    pub genre: Option<String>,
}

impl UpdateDvdRequest {
    /// Parse the year string into a `DateTime<Utc>` if present
    pub fn parse_year(&self) -> ParseYearResult {
        match &self.year {
            Some(year_str) => {
                // Try parsing as full date first (YYYY-MM-DD)
                if let Ok(date) =
                    NaiveDate::parse_from_str(year_str, "%Y-%m-%d")
                {
                    return Ok(Some(
                        date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                    ));
                }

                // Try parsing as year only (YYYY)
                if let Some(date) = year_str
                    .parse::<i32>()
                    .ok()
                    .and_then(|yr| NaiveDate::from_ymd_opt(yr, 1, 1))
                {
                    return Ok(Some(
                        date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                    ));
                }

                Err(format!("Invalid year format: {}", year_str))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- CreateCollectionRequest::parse_type --

    #[test]
    fn test_parse_type_dvd_lowercase() {
        // arrange
        let req = CreateCollectionRequest {
            item_type: "dvd".into(),
        };

        // act
        let result = req.parse_type();

        // assert
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CollectionItemType::Dvd
        );
    }

    #[test]
    fn test_parse_type_dvd_mixed_case() {
        // arrange
        let req = CreateCollectionRequest {
            item_type: "Dvd".into(),
        };

        // act
        let result = req.parse_type();

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_type_book() {
        // arrange
        let req = CreateCollectionRequest {
            item_type: "book".into(),
        };

        // act
        let result = req.parse_type();

        // assert
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CollectionItemType::Book
        );
    }

    #[test]
    fn test_parse_type_invalid_returns_error() {
        // arrange
        let req = CreateCollectionRequest {
            item_type: "vinyl".into(),
        };

        // act
        let result = req.parse_type();

        // assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("vinyl"));
    }

    // -- AddDvdRequest::parse_year --

    #[test]
    fn test_parse_year_yyyy_format() {
        // arrange
        let req = AddDvdRequest {
            name: "Test".into(),
            year: "2024".into(),
            realisator: None,
            actors: vec![],
            genre: None,
        };

        // act
        let result = req.parse_year();

        // assert
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.format("%Y").to_string(), "2024");
    }

    #[test]
    fn test_parse_year_full_date_format() {
        // arrange
        let req = AddDvdRequest {
            name: "Test".into(),
            year: "2024-06-15".into(),
            realisator: None,
            actors: vec![],
            genre: None,
        };

        // act
        let result = req.parse_year();

        // assert
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-06-15");
    }

    #[test]
    fn test_parse_year_invalid_returns_error() {
        // arrange
        let req = AddDvdRequest {
            name: "Test".into(),
            year: "not-a-year".into(),
            realisator: None,
            actors: vec![],
            genre: None,
        };

        // act
        let result = req.parse_year();

        // assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not-a-year"));
    }

    // -- UpdateDvdRequest::parse_year --

    #[test]
    fn test_update_parse_year_none_returns_ok_none() {
        // arrange
        let req = UpdateDvdRequest {
            name: None,
            year: None,
            realisator: None,
            actors: None,
            genre: None,
        };

        // act
        let result = req.parse_year();

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_update_parse_year_valid_returns_some() {
        // arrange
        let req = UpdateDvdRequest {
            name: None,
            year: Some("2023".into()),
            realisator: None,
            actors: None,
            genre: None,
        };

        // act
        let result = req.parse_year();

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_update_parse_year_invalid_returns_error() {
        // arrange
        let req = UpdateDvdRequest {
            name: None,
            year: Some("xyz".into()),
            realisator: None,
            actors: None,
            genre: None,
        };

        // act
        let result = req.parse_year();

        // assert
        assert!(result.is_err());
    }
}

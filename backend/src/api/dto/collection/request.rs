//! Collection Request DTOs

use crate::domain::CollectionItemType;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use validator::Validate;

/// Request body for creating a new collection
#[derive(Debug, Deserialize)]
pub struct CreateCollectionRequest {
    /// Collection type (dvd, book)
    pub item_type: String,
}

impl CreateCollectionRequest {
    /// Parse the item_type string into CollectionItemType
    pub fn parse_type(&self) -> Result<CollectionItemType, String> {
        CollectionItemType::from_str(&self.item_type)
            .ok_or_else(|| format!("Invalid collection type: {}", self.item_type))
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
    pub fn parse_year(&self) -> Result<Option<DateTime<Utc>>, String> {
        match &self.year {
            Some(year_str) => {
                // Try parsing as full date first (YYYY-MM-DD)
                if let Ok(date) = NaiveDate::parse_from_str(year_str, "%Y-%m-%d") {
                    return Ok(Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc()));
                }

                // Try parsing as year only (YYYY)
                if let Ok(year) = year_str.parse::<i32>() {
                    if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                        return Ok(Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc()));
                    }
                }

                Err(format!("Invalid year format: {}", year_str))
            }
            None => Ok(None),
        }
    }
}

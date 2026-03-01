//! Validation service
//!
//! Validation helper functions.

use super::ValidationError;
use crate::shared::{AppError, AppResult};
use validator::Validate;

/// Validate a request using the Validate trait
pub fn validate_request<T: Validate>(req: &T) -> AppResult<()> {
    if let Err(errors) = req.validate() {
        for (field, field_errors) in errors.field_errors() {
            if let Some(err) = field_errors.first() {
                let message = err
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "Validation failed".to_string());

                let static_field: &'static str = match field {
                    "name" => "name",
                    "year" => "year",
                    "realisator" => "realisator",
                    "genre" => "genre",
                    "email" => "email",
                    "password" => "password",
                    "app_name" => "app_name",
                    "app_id" => "app_id",
                    _ => "unknown",
                };

                return Err(AppError::Validation(ValidationError::new(
                    static_field,
                    message,
                )));
            }
        }
    }
    Ok(())
}

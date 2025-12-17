//! Shared validation utilities for API handlers
//!
//! Provides a common validation function that can be used across all handlers
//! to validate request DTOs using the validator crate.

use crate::error::{AppError, AppResult};
use validator::Validate;

/// Validate a request DTO and return an appropriate error if validation fails.
///
/// This function extracts the first validation error and converts it to an AppError
/// with the field name and error message.
///
/// # Example
/// ```ignore
/// use crate::api::validation::validate_request;
///
/// fn handler(req: web::Json<MyRequest>) -> AppResult<HttpResponse> {
///     validate_request(&req.0)?;
///     // ... rest of handler
/// }
/// ```
pub fn validate_request<T: Validate>(req: &T) -> AppResult<()> {
    if let Err(errors) = req.validate() {
        for (field, field_errors) in errors.field_errors() {
            if let Some(err) = field_errors.first() {
                let message = err
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "Validation failed".to_string());

                // Map field names to static strings for error response
                let static_field: &'static str = match field {
                    // Auth fields
                    "email" => "email",
                    "password" => "password",
                    "username" => "username",
                    // App fields
                    "name" => "name",
                    "app_name" => "app_name",
                    "description" => "description",
                    // Fallback
                    _ => "unknown",
                };

                return Err(AppError::validation(static_field, message));
            }
        }
    }
    Ok(())
}

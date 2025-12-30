//! Session utilities - Common functions for working with user sessions

use crate::app::App;
use crate::infra::session::SessionError;
use crate::shared::{AppError, AppResult};
use actix_web::HttpRequest;

/// Extract user ID from session cookie
///
/// # Arguments
/// - `app`: Application state containing session store
/// - `req`: HTTP request containing session cookie
///
/// # Returns
/// - `Ok(String)` with user ID if session is valid
/// - `Err(AppError)` if session is not found or invalid
pub fn get_user_id_from_session(app: &App, req: &HttpRequest) -> AppResult<String> {
    let session_id = req
        .cookie("session_id")
        .map(|c| c.value().to_string())
        .ok_or(AppError::Session(SessionError::NotFound))?;

    let user = app
        .auth
        .sessions()
        .get_user(&session_id)
        .ok_or(AppError::Session(SessionError::NotFound))?;

    Ok(user.id.to_string())
}

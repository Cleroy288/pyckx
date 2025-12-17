//! Session helper functions

use actix_web::HttpRequest;

/// Extract session ID from request cookies
pub fn extract_session_id(req: &HttpRequest) -> Option<String> {
    let cookies = req.cookies().ok()?;
    cookies
        .iter()
        .find(|c| c.name() == "session_id")
        .map(|c| c.value().to_string())
}

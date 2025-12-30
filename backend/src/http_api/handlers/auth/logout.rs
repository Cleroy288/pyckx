//! Logout handler

use super::helpers::extract_session_id;
use crate::app::App;
use actix_web::cookie::Cookie;
use actix_web::{post, web, HttpRequest, HttpResponse};
use tracing::{info, instrument};

/// POST /auth/logout
#[post("/logout")]
#[instrument(skip(app, req))]
pub async fn logout_handler(app: web::Data<App>, req: HttpRequest) -> HttpResponse {
    let session_id = extract_session_id(&req);

    if let Some(ref sid) = session_id {
        app.auth.logout(sid).await;
    } else {
        info!("Logout called without session cookie");
    }

    let session_cookie = Cookie::build("session_id", "")
        .http_only(true)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();

    HttpResponse::Ok()
        .cookie(session_cookie)
        .json(serde_json::json!({"message": "Logged out successfully"}))
}

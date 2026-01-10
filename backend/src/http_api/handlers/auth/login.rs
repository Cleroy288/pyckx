//! Login handler

use crate::app::App;
use crate::http_api::data_transfer_object::{AuthResponse, LoginRequest};
use crate::http_api::utils::validation::validate_request;
use crate::shared::AppResult;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse};
use tracing::{info, instrument};

/// POST /auth/login
#[post("/login")]
#[instrument(skip(app, req), fields(email = %req.email))]
pub async fn login_handler(
    app: web::Data<App>,
    req: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    validate_request(&req.0)?;

    let user = app.auth.login(&req.email, &req.password).await?;
    let session_id = app.auth.sessions().create_session(user.clone());

    let session_cookie = Cookie::build("session_id", session_id.clone())
        .http_only(true)
        .secure(app.config.secure_http.to_lowercase() == "true")
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    let response = AuthResponse::from_user(&user);

    info!(session_id = %session_id, "Login successful");
    Ok(HttpResponse::Ok().cookie(session_cookie).json(response))
}

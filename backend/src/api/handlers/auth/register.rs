//! Register handler

use crate::api::dto::{AuthResponse, RegisterRequest};
use crate::api::validation::validate_request;
use crate::app::App;
use crate::error::AppResult;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse};
use tracing::{info, instrument};

/// POST /auth/register
#[post("/register")]
#[instrument(skip(app, req), fields(email = %req.email, username = %req.username))]
pub async fn register_handler(
    app: web::Data<App>,
    req: web::Json<RegisterRequest>,
) -> AppResult<HttpResponse> {
    validate_request(&req.0)?;

    let user = app
        .auth
        .register(
            &req.email,
            &req.password,
            &req.username,
            req.phone_country_code.as_deref(),
            req.phone_number.as_deref(),
        )
        .await?;

    let session_id = app.auth.sessions().create_session(user.clone());

    let session_cookie = Cookie::build("session_id", session_id.clone())
        .http_only(true)
        .secure(app.config.secure_http.parse().unwrap())
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    let response = AuthResponse::from_user(&user);

    info!(session_id = %session_id, "Registration successful");
    Ok(HttpResponse::Created()
        .cookie(session_cookie)
        .json(response))
}

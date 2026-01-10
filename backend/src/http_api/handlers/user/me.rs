//! Current user handler

use super::helpers::extract_session_id;
use crate::app::App;
use crate::http_api::data_transfer_object::UserResponse;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

/// GET /user/me - Get current user from session
#[get("/me")]
pub async fn me_handler(app: web::Data<App>, req: HttpRequest) -> impl Responder {
    let session_id = match extract_session_id(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match app.auth.sessions().get_user(&session_id) {
        Some(user) => HttpResponse::Ok().json(UserResponse::from(&user)),
        None => HttpResponse::Unauthorized().finish(),
    }
}

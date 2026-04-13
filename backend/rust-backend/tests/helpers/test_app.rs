//! Test app factory — spawns a real actix-test server
//! with real Supabase repositories.

use actix_web::{web, App as ActixApp};
use LAPP::app::App;

/// Build a real actix-test service with the full App state.
///
/// Requires `.env` with valid Supabase credentials.
pub async fn spawn_app() -> actix_test::TestServer {
    let app = App::new()
        .await
        .expect("App::new() requires .env with SP_URL, SP_ANON, etc.");

    let app_data = web::Data::new(app);

    actix_test::start(move || {
        ActixApp::new()
            .app_data(app_data.clone())
            .configure(LAPP::http_api::init)
    })
}

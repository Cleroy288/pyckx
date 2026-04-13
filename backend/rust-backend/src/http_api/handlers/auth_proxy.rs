//! Auth proxy - Forwards auth requests to the auth
//! microservice.
//!
//! Transparently proxies /api/auth/* and /api/user/me
//! to the Hono auth-service, preserving cookies and
//! headers.

use crate::app::App;
use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{debug, warn};

/// Header name for content-type
const HEADER_CONTENT_TYPE: &str = "content-type";
/// Header name for cookie
const HEADER_COOKIE: &str = "cookie";
/// Header name for Set-Cookie (response)
const HEADER_SET_COOKIE: &str = "set-cookie";
/// Header name for Set-Cookie (actix response)
const HEADER_SET_COOKIE_TITLE: &str = "Set-Cookie";

/// Forward request to auth-service, return response
pub async fn proxy(
    app: web::Data<App>,
    req: HttpRequest,
    body: web::Bytes,
) -> HttpResponse {
    let url = build_target_url(&app, &req);
    debug!(target = %url, "Proxying to auth-service");

    match forward_request(&app, &req, &body, &url)
        .await
    {
        Ok(res) => res,
        Err(err) => {
            warn!(error = %err, "Auth proxy failed");
            HttpResponse::BadGateway().json(
                serde_json::json!({
                    "error": "Auth service unavailable"
                }),
            )
        }
    }
}

/// Build target URL: auth_service_url + path + query
fn build_target_url(
    app: &App,
    req: &HttpRequest,
) -> String {
    let path = req.uri().path();
    let query = req
        .uri()
        .query()
        .map(|q| format!("?{q}"))
        .unwrap_or_default();
    format!(
        "{}{path}{query}",
        app.config.auth_service_url
    )
}

/// Forward request using shared HTTP client
async fn forward_request(
    app: &App,
    req: &HttpRequest,
    body: &[u8],
    target_url: &str,
) -> Result<HttpResponse, reqwest::Error> {
    let raw = req.method().as_str().as_bytes();
    let method = reqwest::Method::from_bytes(raw)
        .unwrap_or_else(|err| {
            warn!(
                error = %err,
                method = req.method().as_str(),
                "Invalid HTTP method, falling \
                 back to GET"
            );
            reqwest::Method::GET
        });

    let mut upstream =
        app.http_client.request(method, target_url);
    upstream = forward_header(
        req, upstream, HEADER_CONTENT_TYPE,
    );
    upstream = forward_header(
        req, upstream, HEADER_COOKIE,
    );

    let res =
        upstream.body(body.to_vec()).send().await?;
    build_response(res).await
}

/// Copy a header from actix request to reqwest
fn forward_header(
    req: &HttpRequest,
    mut builder: reqwest::RequestBuilder,
    name: &str,
) -> reqwest::RequestBuilder {
    if let Some(val) = req.headers().get(name) {
        if let Ok(s) = val.to_str() {
            builder = builder.header(name, s);
        }
    }
    builder
}

/// Build actix HttpResponse from reqwest Response
async fn build_response(
    upstream: reqwest::Response,
) -> Result<HttpResponse, reqwest::Error> {
    let status = upstream.status().as_u16();

    let cookies: Vec<String> = upstream
        .headers()
        .get_all(HEADER_SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .map(String::from)
        .collect();

    let body = upstream.bytes().await?;

    let mut response = HttpResponse::build(
        actix_web::http::StatusCode::from_u16(status)
            .unwrap_or(
                actix_web::http::StatusCode
                    ::INTERNAL_SERVER_ERROR,
            ),
    );

    for cookie in &cookies {
        response.append_header((
            HEADER_SET_COOKIE_TITLE,
            cookie.as_str(),
        ));
    }

    Ok(response.body(body))
}

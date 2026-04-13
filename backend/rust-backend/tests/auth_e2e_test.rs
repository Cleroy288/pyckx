//! Auth E2E tests — login, logout, session protection.

mod helpers;

use helpers::auth_helper::{
    login, session_cookie_value, test_credentials,
};
use helpers::test_app::spawn_app;
use serde_json::json;

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_login_valid_credentials_returns_200() {
    // arrange
    let srv = spawn_app().await;
    let (email, pw) = test_credentials();

    // act
    let resp = srv
        .post("/api/auth/login")
        .send_json(&json!({
            "email": email,
            "password": pw,
        }))
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 200);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_login_valid_credentials_sets_session_cookie() {
    // arrange
    let srv = spawn_app().await;
    let (email, pw) = test_credentials();

    // act
    let resp = srv
        .post("/api/auth/login")
        .send_json(&json!({
            "email": email,
            "password": pw,
        }))
        .await
        .unwrap();

    // assert
    let has_session = resp
        .headers()
        .get_all("set-cookie")
        .into_iter()
        .any(|v| {
            v.to_str()
                .map(|s| s.contains("session_id"))
                .unwrap_or(false)
        });
    assert!(has_session, "session_id cookie missing");
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_login_invalid_password_returns_401() {
    // arrange
    let srv = spawn_app().await;
    let (email, _) = test_credentials();

    // act
    let resp = srv
        .post("/api/auth/login")
        .send_json(&json!({
            "email": email,
            "password": "wrong_password_xyz_123",
        }))
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_protected_route_without_cookie_returns_error() {
    // arrange
    let srv = spawn_app().await;

    // act
    let resp = srv.get("/api/user/me").send().await.unwrap();

    // assert
    assert_ne!(resp.status().as_u16(), 200);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_logout_clears_session() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);

    // act — logout
    let logout = srv
        .post("/api/auth/logout")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    assert_eq!(logout.status().as_u16(), 200);

    // assert — session invalidated
    let me = srv
        .get("/api/user/me")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    assert_ne!(me.status().as_u16(), 200);
}

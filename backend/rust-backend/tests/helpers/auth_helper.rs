//! Auth helper — login as test user, extract session cookie.

use actix_test::TestServer;

/// Session cookie value extracted after login.
pub struct TestSession {
    /// Raw `session_id=<uuid>` cookie value
    pub cookie: String,
}

/// Load test credentials from environment.
pub fn test_credentials() -> (String, String) {
    dotenv::dotenv().ok();
    let email = std::env::var("TEST_EMAIL")
        .expect("TEST_EMAIL env var required");
    let password = std::env::var("TEST_PSSWRD")
        .expect("TEST_PSSWRD env var required");
    (email, password)
}

/// Login as the test user and return the session cookie.
pub async fn login(srv: &TestServer) -> TestSession {
    let (email, password) = test_credentials();

    let mut resp = srv
        .post("/api/auth/login")
        .send_json(&serde_json::json!({
            "email": email,
            "password": password,
        }))
        .await
        .expect("login request failed");

    let status = resp.status().as_u16();
    if status != 200 {
        panic!(
            "login helper: expected 200, got {}",
            status
        );
    }

    let cookie = resp
        .headers()
        .get_all("set-cookie")
        .into_iter()
        .find_map(|v| {
            let s = v.to_str().ok()?;
            if s.starts_with("session_id=") {
                Some(s.to_string())
            } else {
                None
            }
        })
        .expect("Login must set session_id cookie");

    TestSession { cookie }
}

/// Extract `session_id=<value>` for use in Cookie header.
pub fn session_cookie_value(
    session: &TestSession,
) -> String {
    session
        .cookie
        .split(';')
        .next()
        .unwrap_or(&session.cookie)
        .to_string()
}

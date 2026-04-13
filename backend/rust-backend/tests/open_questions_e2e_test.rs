//! Open Questions E2E tests — list + create via HTTP.
//!
//! Note: no DELETE endpoint exists for open questions.
//! Created sets leak in the test DB.

mod helpers;

use helpers::auth_helper::{login, session_cookie_value};
use helpers::test_app::spawn_app;
use serde_json::Value;

/// Build a multipart body for POST /api/study/open-questions.
/// Returns (content_type_header, raw_bytes).
fn open_questions_multipart() -> (String, Vec<u8>) {
    let boundary = "----TestBoundary7MA4YWxk";
    let metadata = serde_json::json!({
        "name": "E2E Open Questions",
        "description": "test set",
        "instructions": "Generate open questions",
        "level": "easy",
        "language": "en",
        "subjects": ["math"],
        "num_questions": 2
    });
    let mut body = Vec::new();
    // metadata part
    body.extend_from_slice(
        format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"metadata\"\r\n\
             Content-Type: application/json\r\n\r\n\
             {metadata}\r\n"
        )
        .as_bytes(),
    );
    // file part
    body.extend_from_slice(
        format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; \
             name=\"files\"; filename=\"test.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             The Pythagorean theorem states that \
             a^2 + b^2 = c^2.\r\n\
             --{boundary}--\r\n"
        )
        .as_bytes(),
    );
    let ct = format!("multipart/form-data; boundary={boundary}");
    (ct, body)
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_list_open_questions_returns_200() {
    // Arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let cookie = session_cookie_value(&session);

    // Act
    let resp = srv
        .get("/api/study/open-questions")
        .insert_header(("Cookie", cookie.as_str()))
        .send()
        .await
        .expect("failed to send request");

    // Assert
    assert_eq!(resp.status().as_u16(), 200);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_list_open_questions_has_sets_array() {
    // Arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let cookie = session_cookie_value(&session);

    // Act
    let mut resp = srv
        .get("/api/study/open-questions")
        .insert_header(("Cookie", cookie.as_str()))
        .send()
        .await
        .expect("failed to send request");
    let body: Value = resp.json().await.unwrap();

    // Assert
    assert!(
        body["sets"].is_array(),
        "expected 'sets' array, got: {}",
        body["sets"]
    );
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_create_open_questions_set_returns_201() {
    // Arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let (ct, body) = open_questions_multipart();

    // Act
    let resp = srv
        .post("/api/study/open-questions")
        .insert_header(("Cookie", ck.as_str()))
        .insert_header(("Content-Type", ct.as_str()))
        .send_body(body)
        .await
        .expect("create request failed");

    // Assert — no DELETE endpoint; set leaks
    assert_eq!(resp.status().as_u16(), 201);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_create_open_questions_set_and_list_returns_it() {
    // Arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let (ct, body) = open_questions_multipart();

    // Act — create
    let mut create_resp = srv
        .post("/api/study/open-questions")
        .insert_header(("Cookie", ck.as_str()))
        .insert_header(("Content-Type", ct.as_str()))
        .send_body(body)
        .await
        .expect("create request failed");
    let created: Value = create_resp.json().await.unwrap();
    let created_id = created["id"].as_str().unwrap();

    // Act — list
    let mut list_resp = srv
        .get("/api/study/open-questions")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    let list: Value = list_resp.json().await.unwrap();

    // Assert — created ID appears in the list
    let ids: Vec<&str> = list["sets"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|s| s["id"].as_str())
        .collect();
    assert!(
        ids.contains(&created_id),
        "created id {created_id} not in list: {ids:?}"
    );
}

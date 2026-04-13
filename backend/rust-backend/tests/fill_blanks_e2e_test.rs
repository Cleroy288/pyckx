//! Fill Blanks E2E tests — list + create via HTTP.

mod helpers;

use helpers::auth_helper::{login, session_cookie_value};
use helpers::test_app::spawn_app;
use serde_json::Value;

/// Build a multipart body for fill-blanks creation.
/// Returns (content_type_header, body_bytes).
fn build_fill_blanks_multipart() -> (String, Vec<u8>) {
    let boundary = "----TestBoundary";
    let metadata = serde_json::json!({
        "name": "E2E Fill Blanks Set",
        "description": "Created by E2E test",
        "instructions": "Generate fill-blank questions",
        "language": "en",
        "level": "easy",
        "subjects": ["test"],
        "num_questions": 2
    });
    let mut body = Vec::new();
    // metadata part
    body.extend_from_slice(
        format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; \
             name=\"metadata\"\r\n\
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
             name=\"files\"; \
             filename=\"test.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             The capital of France is Paris.\r\n\
             --{boundary}--\r\n"
        )
        .as_bytes(),
    );
    let ct = format!(
        "multipart/form-data; boundary={boundary}"
    );
    (ct, body)
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_list_fill_blanks_returns_200() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let cookie = session_cookie_value(&session);

    // act
    let resp = srv
        .get("/api/study/fill-blanks")
        .insert_header(("Cookie", cookie.as_str()))
        .send()
        .await
        .expect("failed to send request");

    // assert
    assert_eq!(resp.status().as_u16(), 200);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_list_fill_blanks_has_sets_array() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let cookie = session_cookie_value(&session);

    // act
    let mut resp = srv
        .get("/api/study/fill-blanks")
        .insert_header(("Cookie", cookie.as_str()))
        .send()
        .await
        .expect("failed to send request");
    let body: Value = resp
        .json()
        .await
        .expect("failed to parse JSON body");

    // assert
    assert!(
        body["sets"].is_array(),
        "expected 'sets' array, got: {}",
        body["sets"]
    );
}

/// POST fill-blanks + verify created set appears in list.
/// NOTE: no DELETE endpoint exists for fill-blanks —
/// test data leaks into the test database.
#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_create_fill_blanks_set_and_list_returns_it() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let (ct, body) = build_fill_blanks_multipart();

    // act — create
    let mut resp = srv
        .post("/api/study/fill-blanks")
        .insert_header(("Cookie", ck.as_str()))
        .insert_header(("Content-Type", ct.as_str()))
        .send_body(body)
        .await
        .unwrap();
    let created: Value = resp.json().await.unwrap();
    let id = created["id"].as_str().expect("missing id");

    // act — list
    let mut list_resp = srv
        .get("/api/study/fill-blanks")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    let list: Value = list_resp.json().await.unwrap();

    // assert — created set appears in list
    let ids: Vec<&str> = list["sets"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|s| s["id"].as_str())
        .collect();
    assert!(
        ids.contains(&id),
        "created id {id} not found in list: {ids:?}"
    );
}

//! QCM E2E tests — full CRUD via HTTP.

mod helpers;

use helpers::auth_helper::{login, session_cookie_value};
use helpers::cleanup::delete_qcm_set;
use helpers::test_app::spawn_app;
use serde_json::{json, Value};

/// POST /api/study/qcm — returns (status, body).
async fn create_qcm_set(
    srv: &actix_test::TestServer,
    cookie: &str,
) -> (u16, Value) {
    let body = json!({
        "name": "Test QCM Set",
        "description": "A test set",
        "level": "easy",
        "language": "en",
        "subjects": ["math"],
        "questions": [{
            "question": "What is 2+2?",
            "wrong_answers": ["1", "3", "5"],
            "right_answer": "4",
            "explanation": "Basic arithmetic."
        }]
    });
    let mut resp = srv
        .post("/api/study/qcm")
        .insert_header(("Cookie", cookie))
        .send_json(&body)
        .await
        .unwrap();
    let status = resp.status().as_u16();
    let json: Value = resp.json().await.unwrap();
    (status, json)
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_create_qcm_set_returns_201() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);

    // act
    let (status, body) = create_qcm_set(&srv, &ck).await;

    // assert
    assert_eq!(status, 201);
    assert!(body["id"].is_string());

    let id = body["id"].as_str().unwrap();
    delete_qcm_set(&srv, &ck, id).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_list_qcm_sets_returns_200() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);

    // act
    let mut resp = srv
        .get("/api/study/qcm")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["sets"].is_array());
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_get_qcm_set_by_id_returns_questions() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let (_, created) = create_qcm_set(&srv, &ck).await;
    let id = created["id"].as_str().unwrap().to_string();

    // act
    let mut resp = srv
        .get(&format!("/api/study/qcm/{}", id))
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["questions"].is_array());

    delete_qcm_set(&srv, &ck, &id).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_update_qcm_set_changes_name() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let (_, created) = create_qcm_set(&srv, &ck).await;
    let id = created["id"].as_str().unwrap().to_string();

    // act
    srv.put(&format!("/api/study/qcm/{}", id))
        .insert_header(("Cookie", ck.as_str()))
        .send_json(&json!({"name": "Updated Name"}))
        .await
        .unwrap();
    let mut resp = srv
        .get(&format!("/api/study/qcm/{}", id))
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();

    // assert
    assert_eq!(body["name"].as_str().unwrap(), "Updated Name");

    delete_qcm_set(&srv, &ck, &id).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_delete_qcm_set_removes_from_list() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let (_, created) = create_qcm_set(&srv, &ck).await;
    let id = created["id"].as_str().unwrap().to_string();

    // act
    delete_qcm_set(&srv, &ck, &id).await;
    let resp = srv
        .get(&format!("/api/study/qcm/{}", id))
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();

    // assert
    assert!(resp.status().is_client_error());
}

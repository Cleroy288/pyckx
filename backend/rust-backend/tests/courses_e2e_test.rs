//! Courses E2E tests — course CRUD + sessions.

mod helpers;

use helpers::auth_helper::{login, session_cookie_value};
use helpers::cleanup::{delete_course, delete_session};
use helpers::test_app::spawn_app;
use serde_json::{json, Value};

/// Create a test course, return response JSON.
async fn create_test_course(
    srv: &actix_test::TestServer,
    cookie: &str,
) -> Value {
    let mut resp = srv
        .post("/api/study/courses")
        .insert_header(("Cookie", cookie))
        .send_json(&json!({
            "name": "Test Course",
            "description": "Created by E2E test"
        }))
        .await
        .unwrap();
    let status = resp.status().as_u16();
    assert!(
        (200..300).contains(&status),
        "create_test_course: expected 2xx, got {}",
        status
    );
    resp.json().await.unwrap()
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_list_courses_returns_200() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);

    // act
    let resp = srv
        .get("/api/study/courses")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 200);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_create_course_returns_201() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);

    // act
    let body = create_test_course(&srv, &ck).await;

    // assert
    let id = body["course"]["id"].as_str().unwrap();
    assert!(!id.is_empty());

    delete_course(&srv, &ck, id).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_delete_course_removes_from_list() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let created = create_test_course(&srv, &ck).await;
    let id =
        created["course"]["id"].as_str().unwrap().to_string();

    // act
    delete_course(&srv, &ck, &id).await;
    let mut resp = srv
        .get("/api/study/courses")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();

    // assert
    let empty = vec![];
    let ids: Vec<&str> = body["courses"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter_map(|c| c["id"].as_str())
        .collect();
    assert!(!ids.contains(&id.as_str()));
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_create_session_returns_201() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let created = create_test_course(&srv, &ck).await;
    let cid =
        created["course"]["id"].as_str().unwrap().to_string();

    // act
    let resp = srv
        .post(&format!(
            "/api/study/courses/{}/sessions",
            cid
        ))
        .insert_header(("Cookie", ck.as_str()))
        .send_json(&json!({
            "topic": "Test Topic",
            "instructions": "",
            "keywords": [],
            "language": "en"
        }))
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 201);

    delete_course(&srv, &ck, &cid).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_list_sessions_returns_created() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let created = create_test_course(&srv, &ck).await;
    let cid =
        created["course"]["id"].as_str().unwrap().to_string();
    let mut sr = srv
        .post(&format!(
            "/api/study/courses/{}/sessions",
            cid
        ))
        .insert_header(("Cookie", ck.as_str()))
        .send_json(&json!({
            "topic": "Listed Topic",
            "instructions": "",
            "keywords": [],
            "language": "en"
        }))
        .await
        .unwrap();
    let sb: Value = sr.json().await.unwrap();
    let sid =
        sb["session"]["id"].as_str().unwrap().to_string();

    // act
    let mut resp = srv
        .get(&format!(
            "/api/study/courses/{}/sessions",
            cid
        ))
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();

    // assert
    let empty = vec![];
    let ids: Vec<&str> = body["sessions"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter_map(|s| s["id"].as_str())
        .collect();
    assert!(ids.contains(&sid.as_str()));

    delete_session(&srv, &ck, &cid, &sid).await;
    delete_course(&srv, &ck, &cid).await;
}

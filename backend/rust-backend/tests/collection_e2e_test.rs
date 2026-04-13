//! Collection (DVD) E2E tests — full CRUD via HTTP.

mod helpers;

use helpers::auth_helper::{login, session_cookie_value};
use helpers::cleanup::delete_dvd;
use helpers::test_app::spawn_app;
use serde_json::{json, Value};

/// Add a test DVD, return response JSON.
async fn add_test_dvd(
    srv: &actix_test::TestServer,
    cookie: &str,
) -> Value {
    let name =
        format!("TestDVD-{}", uuid::Uuid::new_v4());
    let mut resp = srv
        .post("/api/collection/dvds")
        .insert_header(("Cookie", cookie))
        .send_json(&json!({
            "name": name,
            "year": "2000",
            "realisator": "Test Director",
            "actors": ["Actor One"],
            "genre": "drama"
        }))
        .await
        .unwrap();
    let status = resp.status();
    assert!(
        status.is_success(),
        "add_test_dvd: expected 2xx, got {}",
        status
    );
    resp.json().await.unwrap()
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_get_dvds_returns_200() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);

    // act
    let resp = srv
        .get("/api/collection/dvds")
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 200);
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_add_dvd_returns_201() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);

    // act
    let mut resp = srv
        .post("/api/collection/dvds")
        .insert_header(("Cookie", ck.as_str()))
        .send_json(&json!({
            "name": format!("DVD-{}", uuid::Uuid::new_v4()),
            "year": "1999",
            "realisator": "Wachowski",
            "actors": ["Keanu"],
            "genre": "sci-fi"
        }))
        .await
        .unwrap();

    // assert
    assert_eq!(resp.status().as_u16(), 201);
    let body: Value = resp.json().await.unwrap();
    let dvd_id = body["dvd"]["id"].as_str().unwrap();
    assert!(!dvd_id.is_empty());

    delete_dvd(&srv, &ck, dvd_id).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_get_dvd_by_id_returns_dvd() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let created = add_test_dvd(&srv, &ck).await;
    let dvd_id = created["dvd"]["id"].as_str().unwrap();
    let expected = created["dvd"]["name"].as_str().unwrap();

    // act
    let mut resp = srv
        .get(&format!("/api/collection/dvds/{}", dvd_id))
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();

    // assert
    assert_eq!(body["name"].as_str().unwrap(), expected);

    delete_dvd(&srv, &ck, dvd_id).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_update_dvd_changes_name() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let created = add_test_dvd(&srv, &ck).await;
    let dvd_id = created["dvd"]["id"].as_str().unwrap();
    let new_name = format!("Upd-{}", uuid::Uuid::new_v4());

    // act
    srv.put(&format!("/api/collection/dvds/{}", dvd_id))
        .insert_header(("Cookie", ck.as_str()))
        .send_json(&json!({ "name": new_name }))
        .await
        .unwrap();
    let mut resp = srv
        .get(&format!("/api/collection/dvds/{}", dvd_id))
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();

    // assert
    assert_eq!(body["name"].as_str().unwrap(), new_name);

    delete_dvd(&srv, &ck, dvd_id).await;
}

#[actix_rt::test]
#[ignore = "requires Supabase connection"]
async fn test_delete_dvd_removes_from_list() {
    // arrange
    let srv = spawn_app().await;
    let session = login(&srv).await;
    let ck = session_cookie_value(&session);
    let created = add_test_dvd(&srv, &ck).await;
    let dvd_id =
        created["dvd"]["id"].as_str().unwrap().to_string();

    // act
    delete_dvd(&srv, &ck, &dvd_id).await;
    let resp = srv
        .get(&format!("/api/collection/dvds/{}", dvd_id))
        .insert_header(("Cookie", ck.as_str()))
        .send()
        .await
        .unwrap();

    // assert
    assert!(resp.status().is_client_error());
}

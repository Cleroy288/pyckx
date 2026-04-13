//! Cleanup utilities — delete test-created data via HTTP.
//!
//! Each helper calls the corresponding DELETE endpoint to
//! remove data created during a test.

use actix_test::TestServer;

/// Delete a QCM set by ID via HTTP.
/// Accepts 2xx and 404; panics on 5xx.
pub async fn delete_qcm_set(
    srv: &TestServer,
    cookie: &str,
    set_id: &str,
) {
    let path = format!("/api/study/qcm/{}", set_id);
    let resp = srv
        .delete(&path)
        .insert_header(("Cookie", cookie))
        .send()
        .await
        .expect("cleanup: delete qcm set");
    check_cleanup_status(resp.status().as_u16(), &path);
}

/// Delete a DVD by ID via HTTP.
/// Accepts 2xx and 404; panics on 5xx.
pub async fn delete_dvd(
    srv: &TestServer,
    cookie: &str,
    dvd_id: &str,
) {
    let path = format!("/api/collection/dvds/{}", dvd_id);
    let resp = srv
        .delete(&path)
        .insert_header(("Cookie", cookie))
        .send()
        .await
        .expect("cleanup: delete dvd");
    check_cleanup_status(resp.status().as_u16(), &path);
}

/// Delete a course by ID via HTTP.
/// Accepts 2xx and 404; panics on 5xx.
pub async fn delete_course(
    srv: &TestServer,
    cookie: &str,
    course_id: &str,
) {
    let path = format!("/api/study/courses/{}", course_id);
    let resp = srv
        .delete(&path)
        .insert_header(("Cookie", cookie))
        .send()
        .await
        .expect("cleanup: delete course");
    check_cleanup_status(resp.status().as_u16(), &path);
}

/// Delete a study session by course + session ID via HTTP.
/// Accepts 2xx and 404; panics on 5xx.
pub async fn delete_session(
    srv: &TestServer,
    cookie: &str,
    course_id: &str,
    session_id: &str,
) {
    let path = format!(
        "/api/study/courses/{}/sessions/{}",
        course_id, session_id
    );
    let resp = srv
        .delete(&path)
        .insert_header(("Cookie", cookie))
        .send()
        .await
        .expect("cleanup: delete session");
    check_cleanup_status(resp.status().as_u16(), &path);
}

/// Panic on 5xx; silently accept 2xx and 404.
fn check_cleanup_status(status: u16, path: &str) {
    if status >= 500 {
        panic!(
            "cleanup: DELETE {} returned {}",
            path, status
        );
    }
}

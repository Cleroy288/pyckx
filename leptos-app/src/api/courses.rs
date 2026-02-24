//! Courses API — CRUD, resources, sessions, generation

use super::{endpoints, helpers};
use crate::domain::course_types::*;

/// Create a new course
pub async fn create_course(
    req: &CreateCourseRequest,
) -> Result<CourseData, String> {
    helpers::post_json(endpoints::COURSES, req).await
}

/// List all courses
pub async fn list_courses(
) -> Result<Vec<CourseData>, String> {
    let resp: CourseListResponse =
        helpers::get_json(endpoints::COURSES).await?;
    Ok(resp.courses)
}

/// Delete a course
pub async fn delete_course(
    id: &str,
) -> Result<bool, String> {
    let url = format!("{}/{}", endpoints::COURSES, id);
    helpers::delete_request(&url).await
}

/// Upload a resource to a course (multipart)
pub async fn upload_resource(
    course_id: &str,
    form: &web_sys::FormData,
) -> Result<ResourceData, String> {
    let url = format!(
        "{}/{}/resources",
        endpoints::COURSES,
        course_id
    );
    helpers::post_multipart(&url, form).await
}

/// Get resources for a course
pub async fn get_resources(
    course_id: &str,
) -> Result<Vec<ResourceData>, String> {
    let url = format!(
        "{}/{}/resources",
        endpoints::COURSES,
        course_id
    );
    helpers::get_json(&url).await
}

/// Create a study session
pub async fn create_session(
    course_id: &str,
    req: &CreateSessionRequest,
) -> Result<SessionData, String> {
    let url = format!(
        "{}/{}/sessions",
        endpoints::COURSES,
        course_id
    );
    helpers::post_json(&url, req).await
}

/// List sessions for a course
pub async fn list_sessions(
    course_id: &str,
) -> Result<Vec<SessionData>, String> {
    let url = format!(
        "{}/{}/sessions",
        endpoints::COURSES,
        course_id
    );
    helpers::get_json(&url).await
}

/// Get a specific session
pub async fn get_session(
    course_id: &str,
    session_id: &str,
) -> Result<SessionData, String> {
    let url = format!(
        "{}/{}/sessions/{}",
        endpoints::COURSES,
        course_id,
        session_id
    );
    helpers::get_json(&url).await
}

/// Generate course content via AI
pub async fn generate_course(
    req: &GenerateCourseRequest,
) -> Result<GenerateCourseResponse, String> {
    helpers::post_json(endpoints::GENERATE_COURSE, req)
        .await
}

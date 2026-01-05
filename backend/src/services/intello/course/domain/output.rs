// Re-export GeneratedCourse from DTO layer
// The actual definition stays in http_api/data_transfer_object/intello/course.rs
// because it's primarily used for HTTP serialization
pub use crate::http_api::data_transfer_object::intello::course::GeneratedCourse;

//! Courses domain types — re-exported from
//! `crate::domain::course_types` so the feature has one
//! import surface.

pub use crate::domain::course_types::{
    ContentBlock, CourseData, CourseMetadata,
    CourseModule, CreateCourseRequest,
    CreateSessionRequest, FlashcardPayload,
    FlashcardSetPayload, GenerateCourseRequest,
    GenerateCourseResponse, GeneratedCourse,
    QcmQuestionPayload, QcmSetPayload, ResourceData,
    SessionData, TrueFalseSetPayload,
    TrueFalseStatementPayload,
};

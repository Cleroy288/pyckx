//! Courses API surface — re-exports HTTP functions for
//! courses, resources and session lifecycle behind one
//! `use crate::courses::api::*;` entry point.

pub use crate::api::courses::{
    create_course, create_session, delete_course,
    generate_course, get_resources, get_session,
    list_courses, list_sessions, upload_resource,
};

pub use crate::api::resources::{
    check_resource_exists, get_user_resources,
    link_resource,
};

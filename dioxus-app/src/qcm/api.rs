//! QCM API — thin facade over `crate::api::study`.
//!
//! Keeps QCM HTTP calls reachable via `crate::qcm::api::*`
//! so the feature has a self-contained import surface.

pub use crate::api::study::{
    create_qcm_set, delete_qcm_set,
    delete_qcm_set_owned, generate_qcm,
    generate_quick_qcm, get_qcm_set, get_qcm_sets,
};

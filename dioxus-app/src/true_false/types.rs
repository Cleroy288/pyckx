//! True/False domain re-exports.
//!
//! Single import surface for the feature — every file
//! in this module pulls its types from here, never from
//! the deep `domain::true_false_types` path.

pub use crate::domain::true_false_types::{
    CreateTrueOrFalseResponse as CreateResponse,
    TrueOrFalseSet as TrueFalseSet,
    TrueOrFalseSetListResponse as ListResponse,
    TrueOrFalseStatement as TrueFalseStatement,
};

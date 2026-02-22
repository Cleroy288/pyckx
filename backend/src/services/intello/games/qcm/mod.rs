pub mod crud;
pub mod domain;
pub mod generation;
pub mod parser;
pub mod prompt;
pub mod quick_generation;

// Re-export commonly used types
pub use domain::{QcmQuestion, QcmSet};
pub use quick_generation::QuickQcmInput;

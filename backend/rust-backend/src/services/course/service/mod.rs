pub mod assembly_service;
pub mod crud;
pub mod generation;
pub mod ideas_service;
pub mod plan_service;
pub mod resources;
pub mod section_service;
pub mod synthesis_service;

// Re-export assembly function for convenience
pub use assembly_service::assemble_complete_course;

// All service functions are implemented via StudyService trait
// No re-exports needed since they're already public on StudyService

pub mod ideas_prompt;
pub mod plan_prompt;
pub mod section_prompt;
pub mod synthesis_prompt;

// Re-export for convenience
pub use ideas_prompt::build_ideas_extraction_prompt;
pub use plan_prompt::build_course_plan_prompt;
pub use section_prompt::build_section_prompt;
pub use synthesis_prompt::build_synthesis_prompt;

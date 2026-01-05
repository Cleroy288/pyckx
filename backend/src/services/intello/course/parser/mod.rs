pub mod block_parsers;
// pub mod course_parser;
pub mod metadata_parser;
pub mod module_parser;
pub mod ideas_parser;
pub mod plan_parser;
pub mod section_parser;
pub mod synthesis_parser;

// Re-export main entry point
// pub use course_parser::CourseParser;
pub use ideas_parser::parse_extracted_ideas;
pub use plan_parser::parse_course_plan;
pub use section_parser::parse_generated_section;
pub use synthesis_parser::parse_generated_synthesis;

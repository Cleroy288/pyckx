pub mod entities;
pub mod ideas_domain;
pub mod input;
pub mod output;
pub mod plan_domain;
pub mod section_domain;
pub mod synthesis_domain;

// Re-export commonly used types
pub use entities::{Course, CreateCourseInput, ResourceSummary, UserResource};
pub use ideas_domain::ExtractedIdeas;
pub use input::GenerateCourseInput;
pub use output::GeneratedCourse;
pub use plan_domain::{CoursePlan, SectionPlan};
pub use section_domain::ParsedSection;
pub use synthesis_domain::ParsedSynthesis;

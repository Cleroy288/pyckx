/// Input for generating a full AI course
#[derive(Debug, Clone)]
pub struct GenerateCourseInput {
    pub topic: String,
    pub keywords: Vec<String>,
    pub instructions: String,
    pub resources: String,
    pub session_id: Option<String>,
    pub text_length: Option<String>,
    pub exercise_depth: Option<String>,
}

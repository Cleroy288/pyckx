// ** CoursePlan **
// ==> High-level course structure from AI
//
// @ title : Course title
// @ subtitle : Short description
// @ sections : Ordered list of section plans
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct CoursePlan {
    pub title: String,
    pub subtitle: String,
    pub sections: Vec<SectionPlan>,
}

// ** SectionPlan **
// ==> Plan for a single section (before AI generation)
//
// @ order : Section order (1-based)
// @ title : Section title
// @ key_concepts : 3-5 concepts to cover
// @ qcm_count : Number of QCM questions (3-5)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct SectionPlan {
    pub order: u8,
    pub title: String,
    pub key_concepts: Vec<String>,
    pub qcm_count: u8,
}

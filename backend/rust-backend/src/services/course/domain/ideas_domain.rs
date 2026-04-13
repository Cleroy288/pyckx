// ** ExtractedIdeas **
// ==> User demand analysis result from AI
//
// @ core_intent : Main learning objective
// @ target_level : Beginner/Intermediate/Advanced
// @ mandatory_topics : Topics user explicitly wants
// @ suggested_topics : Topics AI recommends adding
// @ constraints : Time limits, prerequisites, etc.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ExtractedIdeas {
    pub core_intent: String,
    pub target_level: String,
    pub mandatory_topics: Vec<String>,
    pub suggested_topics: Vec<String>,
    pub constraints: Vec<String>,
}

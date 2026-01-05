use crate::http_api::data_transfer_object::intello::course::{ContentBlock, QcmSetPayload};

// ** ParsedSection **
// ==> Fully parsed section with content and QCM
//
// @ order : Section order (1-based)
// @ title : Section title  
// @ content_blocks : Rich content (text, schemas, etc.)
// @ qcm_set : REQUIRED QCM questions for this section
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedSection {
    pub order: u8,
    pub title: String,
    pub content_blocks: Vec<ContentBlock>,
    pub qcm_set: QcmSetPayload,  // Required - every section must have a QCM
}

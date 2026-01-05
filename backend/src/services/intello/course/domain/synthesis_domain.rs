use crate::http_api::data_transfer_object::intello::course::QcmSetPayload;

// ** ParsedSynthesis **
// ==> Final course synthesis with comprehensive QCM
//
// @ summary_text : Course summary (500+ words)
// @ key_takeaways : Bullet points of main learnings
// @ final_qcm : Comprehensive QCM (10-15 questions)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedSynthesis {
    pub summary_text: String,
    pub key_takeaways: Vec<String>,
    pub final_qcm: QcmSetPayload,
}

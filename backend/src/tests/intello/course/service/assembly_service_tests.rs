use crate::http_api::data_transfer_object::intello::course::{ContentBlock, QcmSetPayload};
use crate::services::intello::course::domain::{CoursePlan, ParsedSection, ParsedSynthesis};
use crate::services::intello::course::service::assembly_service::assemble_complete_course;

#[test]
fn test_assembles_course() {
    let plan = CoursePlan {
        title: "Test Course".to_string(),
        subtitle: "A test".to_string(),
        sections: vec![],
    };

    let section = ParsedSection {
        order: 1,
        title: "Section 1".to_string(),
        content_blocks: vec![ContentBlock::Text {
            content: "Content".to_string(),
        }],
        qcm_set: QcmSetPayload {
            name: "Section QCM".to_string(),
            description: "Test".to_string(),
            level: "easy".to_string(),
            subjects: vec![],
            questions: vec![],
        },
    };

    let synthesis = ParsedSynthesis {
        summary_text: "Summary".to_string(),
        key_takeaways: vec!["Takeaway 1".to_string()],
        final_qcm: QcmSetPayload {
            name: "Final".to_string(),
            description: "Test".to_string(),
            level: "easy".to_string(),
            subjects: vec![],
            questions: vec![],
        },
    };

    let course = assemble_complete_course(&plan, vec![section], synthesis).unwrap();
    assert_eq!(course.course_metadata.title, "Test Course");
    assert_eq!(course.modules.len(), 1);
}

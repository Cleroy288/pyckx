use crate::http_api::data_transfer_object::course::{
    ContentBlock, QcmSetPayload,
};
use crate::services::course::domain::{
    CoursePlan, ParsedSection, ParsedSynthesis,
};
use crate::services::course::service::{
    assembly_service::assemble_complete_course,
};

/// Build a minimal CoursePlan for testing.
fn make_plan() -> CoursePlan {
    CoursePlan {
        title: "Test Course".to_string(),
        subtitle: "A test".to_string(),
        sections: vec![],
    }
}

/// Build a minimal ParsedSection for testing.
fn make_section() -> ParsedSection {
    ParsedSection {
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
    }
}

/// Build a minimal ParsedSynthesis for testing.
fn make_synthesis() -> ParsedSynthesis {
    ParsedSynthesis {
        summary_text: "Summary".to_string(),
        key_takeaways: vec!["Takeaway 1".to_string()],
        final_qcm: QcmSetPayload {
            name: "Final".to_string(),
            description: "Test".to_string(),
            level: "easy".to_string(),
            subjects: vec![],
            questions: vec![],
        },
    }
}

#[test]
fn test_assemble_complete_course_sets_title() {
    // Arrange
    let plan = make_plan();
    let sections = vec![make_section()];
    let synthesis = make_synthesis();

    // Act
    let course = assemble_complete_course(
        &plan, sections, synthesis,
    )
    .unwrap();

    // Assert
    assert_eq!(
        course.course_metadata.title, "Test Course",
    );
}

#[test]
fn test_assemble_complete_course_builds_modules() {
    // Arrange
    let plan = make_plan();
    let sections = vec![make_section()];
    let synthesis = make_synthesis();

    // Act
    let course = assemble_complete_course(
        &plan, sections, synthesis,
    )
    .unwrap();

    // Assert
    assert_eq!(course.modules.len(), 1);
}

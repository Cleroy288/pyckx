use crate::http_api::data_transfer_object::course::{
    ContentBlock, CourseMetadata, CourseModule, GeneratedCourse,
};
use crate::services::course::domain::{
    CoursePlan, ParsedSection, ParsedSynthesis,
};
use crate::services::error_domain::StudyError;

/// Default difficulty level for assembled courses
const DEFAULT_LEVEL: &str = "intermediate";
/// Title for the synthesis module
const SYNTHESIS_TITLE: &str = "Course Synthesis";
/// Subtitle for the course summary section
const SUMMARY_SUBTITLE: &str = "Course Summary";
/// Subtitle for the key takeaways section
const TAKEAWAYS_SUBTITLE: &str = "Key Takeaways";

// ** assemble_complete_course **
// ==> Combines all generated parts into final GeneratedCourse structure
//
// @ course_plan : Original plan with metadata
// @ parsed_sections : All generated sections
// @ parsed_synthesis : Generated synthesis
// @ returns : CompleteCourse matching existing DTO format
// @ errors : ValidationFailed if assembly fails
pub fn assemble_complete_course(
    course_plan: &CoursePlan,
    parsed_sections: Vec<ParsedSection>,
    parsed_synthesis: ParsedSynthesis,
) -> Result<GeneratedCourse, StudyError> {
    // Step 1: Create course metadata from plan
    let course_metadata = CourseMetadata {
        title: course_plan.title.clone(),
        description: course_plan.subtitle.clone(),
        level: DEFAULT_LEVEL.to_string(),
    };

    // Step 2: Convert parsed sections to CourseModule format
    let modules: Vec<CourseModule> = parsed_sections
        .into_iter()
        .map(|mut section| {
            // Append the required Knowledge Test QCM to the content blocks
            section.content_blocks.push(ContentBlock::QcmSet {
                data: section.qcm_set,
            });

            CourseModule {
                title: section.title,
                blocks: section.content_blocks,
            }
        })
        .collect();

    // Step 3: Create synthesis module from parsed synthesis
    let synthesis_blocks = create_synthesis_blocks(&parsed_synthesis)?;
    let synthesis = CourseModule {
        title: SYNTHESIS_TITLE.to_string(),
        blocks: synthesis_blocks,
    };

    // Step 4: Assemble final course
    let course = GeneratedCourse {
        course_metadata,
        modules,
        synthesis,
    };

    // Step 5: Return assembled course
    Ok(course)
}

// ** create_synthesis_blocks **
// ==> Converts ParsedSynthesis to ContentBlock array
//
// @ synthesis : ParsedSynthesis with summary and QCM
// @ returns : Vec of ContentBlocks for synthesis module
fn create_synthesis_blocks(
    synthesis: &ParsedSynthesis,
) -> Result<Vec<ContentBlock>, StudyError> {
    // Step 1: Create summary text block
    let mut blocks = vec![
        ContentBlock::Subtitle {
            content: SUMMARY_SUBTITLE.to_string(),
        },
        ContentBlock::Text {
            content: synthesis.summary_text.clone(),
        },
    ];

    // Step 2: Add key takeaways section
    blocks.push(ContentBlock::Subtitle {
        content: TAKEAWAYS_SUBTITLE.to_string(),
    });

    let takeaways_text = synthesis
        .key_takeaways
        .iter()
        .enumerate()
        .map(|(i, t)| format!("{}. {}", i + 1, t))
        .collect::<Vec<_>>()
        .join("\n\n");

    blocks.push(ContentBlock::Text {
        content: takeaways_text,
    });

    // Step 3: Add final QCM block
    blocks.push(ContentBlock::QcmSet {
        data: synthesis.final_qcm.clone(),
    });

    // Step 4: Return blocks
    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_api::data_transfer_object::course::{
        QcmQuestionPayload, QcmSetPayload,
    };
    use crate::services::course::domain::SectionPlan;

    // -- test data factories --

    fn test_qcm_payload() -> QcmSetPayload {
        QcmSetPayload {
            name: "Quiz".to_string(),
            description: "Test quiz".to_string(),
            level: "beginner".to_string(),
            subjects: vec![],
            questions: vec![QcmQuestionPayload {
                question: "Q?".to_string(),
                right_answer: "A".to_string(),
                wrong_answers: vec!["B".to_string()],
                explanation: "Because".to_string(),
            }],
        }
    }

    fn test_course_plan(
        title: &str,
        subtitle: &str,
    ) -> CoursePlan {
        CoursePlan {
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            sections: vec![SectionPlan {
                order: 1,
                title: "S1".to_string(),
                key_concepts: vec![],
                qcm_count: 1,
            }],
        }
    }

    fn test_synthesis(summary: &str) -> ParsedSynthesis {
        ParsedSynthesis {
            summary_text: summary.to_string(),
            key_takeaways: vec![
                "Point one".to_string(),
            ],
            final_qcm: test_qcm_payload(),
        }
    }

    // -- tests --

    #[test]
    fn test_assemble_course_empty_sections_returns_ok() {
        // arrange
        let plan = test_course_plan("Title", "Sub");
        let sections: Vec<ParsedSection> = vec![];
        let synthesis = test_synthesis("Summary");

        // act
        let result = assemble_complete_course(
            &plan, sections, synthesis,
        );

        // assert
        assert_eq!(result.unwrap().modules.len(), 0);
    }

    #[test]
    fn test_assemble_course_preserves_title() {
        // arrange
        let plan = test_course_plan("My Course", "Sub");
        let synthesis = test_synthesis("Sum");

        // act
        let course = assemble_complete_course(
            &plan, vec![], synthesis,
        )
        .unwrap();

        // assert
        assert_eq!(
            course.course_metadata.title,
            "My Course"
        );
    }

    #[test]
    fn test_assemble_course_sets_description_from_subtitle()
    {
        // arrange
        let plan =
            test_course_plan("Title", "Subtitle");
        let synthesis = test_synthesis("Sum");

        // act
        let course = assemble_complete_course(
            &plan, vec![], synthesis,
        )
        .unwrap();

        // assert
        assert_eq!(
            course.course_metadata.description,
            "Subtitle"
        );
    }

    #[test]
    fn test_assemble_course_synthesis_has_summary() {
        // arrange
        let plan = test_course_plan("T", "S");
        let synthesis =
            test_synthesis("Summary here");

        // act
        let course = assemble_complete_course(
            &plan, vec![], synthesis,
        )
        .unwrap();

        // assert
        let has_summary =
            course.synthesis.blocks.iter().any(|b| {
                matches!(
                    b,
                    ContentBlock::Text { content }
                    if content == "Summary here"
                )
            });
        assert!(
            has_summary,
            "synthesis blocks should contain \
             the summary text"
        );
    }
}

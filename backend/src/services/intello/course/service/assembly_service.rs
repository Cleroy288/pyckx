use crate::http_api::data_transfer_object::intello::course::{ContentBlock, CourseMetadata, CourseModule, GeneratedCourse};
use crate::services::intello::course::domain::{CoursePlan, ParsedSection, ParsedSynthesis};
use crate::services::intello::error_domain::IntelloError;

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
) -> Result<GeneratedCourse, IntelloError> {
    // Step 1: Create course metadata from plan
    let course_metadata = CourseMetadata {
        title: course_plan.title.clone(),
        description: course_plan.subtitle.clone(),
        level: "intermediate".to_string(), // Default, could be extracted from ideas
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
        title: "Course Synthesis".to_string(),
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
fn create_synthesis_blocks(synthesis: &ParsedSynthesis) -> Result<Vec<ContentBlock>, IntelloError> {
    // Step 1: Create summary text block
    let mut blocks = vec![
        ContentBlock::Subtitle {
            content: "Course Summary".to_string(),
        },
        ContentBlock::Text {
            content: synthesis.summary_text.clone(),
        },
    ];

    // Step 2: Add key takeaways section
    blocks.push(ContentBlock::Subtitle {
        content: "Key Takeaways".to_string(),
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


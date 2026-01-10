use crate::services::intello::course::parser::plan_parser::parse_course_plan;

#[test]
fn test_parses_valid_plan() {
    let json = r#"{
        "title": "Async Rust Course",
        "subtitle": "Learn async programming",
        "sections": [
            {
                "order": 1,
                "title": "Basics",
                "key_concepts": ["tokio", "futures"],
                "qcm_count": 5
            }
        ]
    }"#;

    let plan = parse_course_plan(json).unwrap();
    assert_eq!(plan.title, "Async Rust Course");
    assert_eq!(plan.sections.len(), 1);
    assert_eq!(plan.sections[0].order, 1);
}

#[test]
fn test_rejects_empty_sections() {
    let json = r#"{
        "title": "Course",
        "subtitle": "Description",
        "sections": []
    }"#;

    let result = parse_course_plan(json);
    assert!(result.is_err());
}

#[test]
fn test_rejects_section_without_concepts() {
    let json = r#"{
        "title": "Course",
        "subtitle": "Description",
        "sections": [{
            "order": 1,
            "title": "Section",
            "key_concepts": [],
            "qcm_count": 5
        }]
    }"#;

    let result = parse_course_plan(json);
    assert!(result.is_err());
}

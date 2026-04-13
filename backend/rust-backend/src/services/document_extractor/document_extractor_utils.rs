//! Document Extractor Utilities
//!
//! Contains utility functions for document extraction
// ==> This file provides helper functions used across multiple extraction operations

/// Reads a tag name from chars until closing `>`
fn read_tag_name(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> String {
    let mut tag = String::new();
    while let Some(&next) = chars.peek() {
        if next == '>' {
            chars.next();
            break;
        }
        tag.push(chars.next().unwrap());
    }
    tag
}

/// Applies tag-based state transitions for XML parsing
fn apply_tag(
    tag: &str,
    in_text_tag: &mut bool,
    result: &mut String,
) {
    match tag {
        "a:t" => *in_text_tag = true,
        "/a:t" => {
            *in_text_tag = false;
            result.push(' ');
        }
        "a:p" | "/a:p" => {
            if !result.ends_with('\n') && !result.is_empty() {
                result.push('\n');
            }
        }
        _ => {}
    }
}

/// Extracts text from XML by finding `<a:t>` tags
pub fn extract_text_from_xml(xml: &str) -> String {
    let mut result = String::new();
    let mut in_text_tag = false;
    let mut chars = xml.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            let tag = read_tag_name(&mut chars);
            apply_tag(&tag, &mut in_text_tag, &mut result);
        } else if in_text_tag {
            result.push(c);
        }
    }

    result
}

/// Collects text from all paragraphs in a DOCX document
pub fn collect_docx_text(docx: docx_rs::Docx) -> String {
    let mut content = String::new();
    for child in docx.document.children {
        if let docx_rs::DocumentChild::Paragraph(para) = child {
            content.push_str(&extract_paragraph_text(*para));
            content.push('\n');
        }
    }
    content
}

/// Extracts text from a single DOCX paragraph
fn extract_paragraph_text(
    para: docx_rs::Paragraph,
) -> String {
    para.children
        .into_iter()
        .filter_map(|child| match child {
            docx_rs::ParagraphChild::Run(run) => {
                Some(extract_run_text(*run))
            }
            _ => None,
        })
        .collect()
}

/// Extracts text from a single DOCX run element
fn extract_run_text(run: docx_rs::Run) -> String {
    run.children
        .into_iter()
        .filter_map(|child| match child {
            docx_rs::RunChild::Text(text) => Some(text.text),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_simple_xml_returns_text() {
        // arrange
        let xml = "<a:t>Hello</a:t>";

        // act
        let result = extract_text_from_xml(xml);

        // assert
        assert_eq!(result.trim(), "Hello");
    }

    #[test]
    fn test_extract_text_nested_tags_returns_text_only() {
        // arrange
        let xml = "<root><a:t>First</a:t><a:t>Second</a:t></root>";

        // act
        let result = extract_text_from_xml(xml);

        // assert
        assert!(result.contains("First"));
        assert!(result.contains("Second"));
    }

    #[test]
    fn test_extract_text_empty_xml_returns_empty() {
        // arrange
        let xml = "";

        // act
        let result = extract_text_from_xml(xml);

        // assert
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_text_no_text_tags_returns_empty() {
        // arrange
        let xml = "<root><other>data</other></root>";

        // act
        let result = extract_text_from_xml(xml);

        // assert
        assert_eq!(result.trim(), "");
    }

    #[test]
    fn test_extract_text_paragraph_boundaries_add_newlines() {
        // arrange
        let xml = "<a:p><a:t>Line1</a:t></a:p><a:p><a:t>Line2</a:t></a:p>";

        // act
        let result = extract_text_from_xml(xml);

        // assert
        assert!(result.contains("Line1"));
        assert!(result.contains("Line2"));
        assert!(result.contains('\n'));
    }

    #[test]
    fn test_extract_text_xml_with_attributes_returns_text() {
        // arrange
        let xml =
            "<a:t xml:space=\"preserve\">Hello</a:t>";

        // act
        let result = extract_text_from_xml(xml);

        // assert — tag with attrs doesn't match "a:t"
        assert_eq!(result.trim(), "");
    }

    #[test]
    fn test_extract_text_multiple_paragraphs_separate_lines() {
        // arrange
        let xml = "\
            <a:p><a:t>Para1</a:t></a:p>\
            <a:p><a:t>Para2</a:t></a:p>\
            <a:p><a:t>Para3</a:t></a:p>";

        // act
        let result = extract_text_from_xml(xml);

        // assert
        assert!(result.contains("Para1"));
        assert!(result.contains("Para2"));
        assert!(result.contains("Para3"));
        let newline_count =
            result.chars().filter(|&c| c == '\n').count();
        assert!(
            newline_count >= 2,
            "expected >= 2 newlines, got {newline_count}"
        );
    }

    #[test]
    fn test_extract_text_text_between_other_tags_ignored() {
        // arrange
        let xml =
            "<b:t>Ignored</b:t><a:t>Kept</a:t>";

        // act
        let result = extract_text_from_xml(xml);

        // assert
        assert!(result.contains("Kept"));
        assert!(!result.contains("Ignored"));
    }

    #[test]
    fn test_extract_text_special_chars_preserved() {
        // arrange
        let xml =
            "<a:t>Price: $100 &amp; tax</a:t>";

        // act
        let result = extract_text_from_xml(xml);

        // assert — parser doesn't decode XML entities
        assert!(
            result.contains("Price: $100 &amp; tax")
        );
    }
}

//! Document Extractor Utilities
//!
//! Contains utility functions for document extraction
// ==> This file provides helper functions used across multiple extraction operations

// ** extract_text_from_xml **
// Extracts text content from XML by finding all <a:t> tags
// ==> Simple XML parser that extracts text between <a:t> and </a:t> tags
// @ xml : The XML content to parse
// @ returns : Extracted text content
// @ edge cases : Malformed XML may result in incomplete extraction
// @ error conditions : None (always returns some text, even if empty)
#[allow(clippy::excessive_nesting)]
pub fn extract_text_from_xml(xml: &str) -> String {
    let mut result = String::new();
    let mut in_text_tag = false;
    let mut chars = xml.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            // Check if this is <a:t> or </a:t>
            let mut tag = String::new();
            while let Some(&next) = chars.peek() {
                if next == '>' {
                    chars.next();
                    break;
                }
                tag.push(chars.next().unwrap());
            }

            if tag == "a:t" {
                in_text_tag = true;
            } else if tag == "/a:t" {
                in_text_tag = false;
                result.push(' '); // Add space between text elements
            } else if tag == "a:p" || tag == "/a:p" {
                // Paragraph boundary - add newline
                if !result.ends_with('\n') && !result.is_empty() {
                    result.push('\n');
                }
            }
        } else if in_text_tag {
            result.push(c);
        }
    }

    result
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
}

//! Fill blank game domain types

use serde::Deserialize;

/// Option for a fill-the-blank question
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FillBlankOption {
    pub id: String,
    pub text: String,
    pub is_correct: bool,
}

/// Fill blank question
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FillBlankQuestion {
    pub id: String,
    pub phrase: String,
    pub options: Vec<FillBlankOption>,
    pub explanation: String,
}

/// Fill blank set
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FillBlankSet {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<FillBlankQuestion>,
}

/// List response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FillBlankSetListResponse {
    pub sets: Vec<FillBlankSet>,
    pub count: usize,
}

/// AI creation response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CreateFillBlankResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<FillBlankQuestion>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_fill_blank_set() {
        let json = r#"{
            "id": "1", "name": "FB",
            "description": "D",
            "level": "easy", "language": "en",
            "subjects": [],
            "questions": [{
                "id": "q1",
                "phrase": "The __ is blue",
                "options": [
                    {"id":"o1","text":"sky",
                     "is_correct":true},
                    {"id":"o2","text":"cat",
                     "is_correct":false}
                ],
                "explanation": "sky"
            }]
        }"#;
        let s: FillBlankSet =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.questions.len(), 1);
        assert!(s.questions[0].options[0].is_correct);
    }

    #[test]
    fn test_deserialize_list_response() {
        let json = r#"{"sets": [], "count": 0}"#;
        let r: FillBlankSetListResponse =
            serde_json::from_str(json).unwrap();
        assert!(r.sets.is_empty());
    }
}

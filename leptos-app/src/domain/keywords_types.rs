//! Keywords game domain types

use serde::Deserialize;

/// Single keyword in a question
#[derive(Debug, Clone, Deserialize)]
pub struct Keyword {
    pub id: String,
    pub word: String,
    pub is_correct: bool,
}

/// Keywords question with statement and choices
#[derive(Debug, Clone, Deserialize)]
pub struct KeywordQuestion {
    pub id: String,
    pub statement: String,
    pub keywords: Vec<Keyword>,
    pub explanation: String,
}

/// Keywords set
#[derive(Debug, Clone, Deserialize)]
pub struct KeywordSet {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<KeywordQuestion>,
}

/// List response
#[derive(Debug, Clone, Deserialize)]
pub struct KeywordSetListResponse {
    pub sets: Vec<KeywordSet>,
    pub count: usize,
}

/// AI creation response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateKeywordsResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<KeywordQuestion>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_keyword_set() {
        let json = r#"{
            "id": "1", "name": "KW",
            "description": "D",
            "level": "medium", "language": "en",
            "subjects": [],
            "questions": [{
                "id": "q1",
                "statement": "Pick keywords",
                "keywords": [
                    {"id":"k1","word":"rust",
                     "is_correct":true}
                ],
                "explanation": "Rust is correct"
            }]
        }"#;
        let s: KeywordSet =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.questions.len(), 1);
        assert!(s.questions[0].keywords[0].is_correct);
    }

    #[test]
    fn test_deserialize_list_response() {
        let json = r#"{"sets": [], "count": 0}"#;
        let r: KeywordSetListResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(r.count, 0);
    }
}

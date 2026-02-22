//! True/False domain types

use serde::Deserialize;

/// Single true/false statement
#[derive(Debug, Clone, Deserialize)]
pub struct TrueOrFalseStatement {
    pub id: String,
    pub statement: String,
    pub answer: bool,
    pub explanation: String,
}

/// True/false set with statements
#[derive(Debug, Clone, Deserialize)]
pub struct TrueOrFalseSet {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub statements: Vec<TrueOrFalseStatement>,
}

/// List response for true/false sets
#[derive(Debug, Clone, Deserialize)]
pub struct TrueOrFalseSetListResponse {
    pub sets: Vec<TrueOrFalseSet>,
    pub count: usize,
}

/// AI creation response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTrueOrFalseResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub statements: Vec<TrueOrFalseStatement>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_true_false_set() {
        let json = r#"{
            "id": "1", "name": "TF",
            "description": "D",
            "level": "hard", "language": "en",
            "subjects": ["bio"],
            "statements": [{
                "id": "s1",
                "statement": "Sky is blue",
                "answer": true,
                "explanation": "Yes"
            }]
        }"#;
        let s: TrueOrFalseSet =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.statements.len(), 1);
        assert!(s.statements[0].answer);
    }

    #[test]
    fn test_deserialize_list_response() {
        let json = r#"{"sets": [], "count": 0}"#;
        let r: TrueOrFalseSetListResponse =
            serde_json::from_str(json).unwrap();
        assert!(r.sets.is_empty());
    }

    #[test]
    fn test_deserialize_create_response() {
        let json = r#"{
            "success": true, "message": "ok",
            "id": "x",
            "total_token_count": 100,
            "documents_processed": 1,
            "statements": []
        }"#;
        let r: CreateTrueOrFalseResponse =
            serde_json::from_str(json).unwrap();
        assert!(r.success);
    }
}

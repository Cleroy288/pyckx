//! Order phrase game domain types

use serde::Deserialize;

/// Single word in a phrase to order
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderPhraseWord {
    pub id: String,
    pub word: String,
    pub position: u8,
}

/// Order phrase question
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderPhraseQuestion {
    pub id: String,
    pub original_phrase: String,
    pub words: Vec<OrderPhraseWord>,
    pub hint: String,
}

/// Order phrase set
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderPhraseSet {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<OrderPhraseQuestion>,
}

/// List response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderPhraseSetListResponse {
    pub sets: Vec<OrderPhraseSet>,
    pub count: usize,
}

/// AI creation response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CreateOrderPhraseResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<OrderPhraseQuestion>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_order_phrase_set() {
        let json = r#"{
            "id": "1", "name": "OP",
            "description": "D",
            "level": "easy", "language": "fr",
            "subjects": [],
            "questions": [{
                "id": "q1",
                "original_phrase": "Le chat dort",
                "words": [
                    {"id":"w1","word":"chat",
                     "position":1},
                    {"id":"w2","word":"Le",
                     "position":0}
                ],
                "hint": "animal"
            }]
        }"#;
        let s: OrderPhraseSet =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.questions[0].words.len(), 2);
        assert_eq!(s.questions[0].words[0].position, 1);
    }

    #[test]
    fn test_deserialize_list_response() {
        let json = r#"{"sets": [], "count": 0}"#;
        let r: OrderPhraseSetListResponse =
            serde_json::from_str(json).unwrap();
        assert!(r.sets.is_empty());
    }
}

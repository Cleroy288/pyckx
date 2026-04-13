//! Order Phrase Response DTOs

use crate::services::OrderPhraseWord;
use serde::Serialize;

/// Response for a single word in a phrase puzzle
#[derive(Debug, Serialize)]
pub struct OrderPhraseWordResponse {
    pub id: String,
    pub word: String,
    pub position: u8,
}

impl From<&OrderPhraseWord> for OrderPhraseWordResponse {
    fn from(w: &OrderPhraseWord) -> Self {
        Self {
            id: w.id.to_string(),
            word: w.word.clone(),
            position: w.position,
        }
    }
}

/// Response for a single order phrase question
#[derive(Debug, Serialize)]
pub struct OrderPhraseQuestionResponse {
    pub id: String,
    pub original_phrase: String,
    pub words: Vec<OrderPhraseWordResponse>,
    pub hint: String,
}

/// Response for creating order phrase questions (AI generation)
#[derive(Debug, Serialize)]
pub struct CreateOrderPhraseResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<OrderPhraseQuestionResponse>,
}

/// Response for listing order phrase sets (with full questions for playing)
#[derive(Debug, Serialize)]
pub struct OrderPhraseSetListResponse {
    pub sets: Vec<OrderPhraseSetWithQuestionsResponse>,
    pub count: usize,
}

/// Response for an order phrase set with full questions (for playing)
#[derive(Debug, Serialize)]
pub struct OrderPhraseSetWithQuestionsResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<OrderPhraseQuestionResponse>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::OptionId;

    #[test]
    fn test_order_phrase_word_response_from() {
        // arrange
        let word = OrderPhraseWord {
            id: OptionId::from_string("w-1".into()),
            word: "hello".into(),
            position: 0,
        };

        // act
        let resp = OrderPhraseWordResponse::from(&word);

        // assert
        assert_eq!(resp.id, "w-1");
        assert_eq!(resp.word, "hello");
        assert_eq!(resp.position, 0);
    }

    #[test]
    fn test_order_phrase_word_response_position() {
        // arrange
        let word = OrderPhraseWord {
            id: OptionId::from_string("w-2".into()),
            word: "world".into(),
            position: 3,
        };

        // act
        let resp = OrderPhraseWordResponse::from(&word);

        // assert
        assert_eq!(resp.position, 3);
    }
}

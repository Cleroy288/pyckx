//! Order Phrase AI response types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrderPhraseAiResponse {
    pub questions: Vec<OrderPhraseAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub struct OrderPhraseAiQuestion {
    pub original_phrase: String,
    pub words: Vec<OrderPhraseAiWord>,
    pub hint: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderPhraseAiWord {
    pub word: String,
    pub position: u8,
}

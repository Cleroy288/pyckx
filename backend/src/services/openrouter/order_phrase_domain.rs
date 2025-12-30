//! Order Phrase AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct OrderPhraseAiResponse {
    pub questions: Vec<OrderPhraseAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub(super) struct OrderPhraseAiQuestion {
    pub original_phrase: String,
    pub words: Vec<OrderPhraseAiWord>,
    pub hint: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct OrderPhraseAiWord {
    pub word: String,
    pub position: u8,
}

//! GameSetInfo — common interface for all game sets

use super::fill_blank_types::FillBlankSet;
use super::flashcard_types::FlashcardSet;
use super::keywords_types::KeywordSet;
use super::open_question_types::OpenQuestionSet;
use super::order_phrase_types::OrderPhraseSet;
use super::qcm_types::QcmSet;
use super::true_false_types::TrueOrFalseSet;

/// Common interface for game set types
pub trait GameSetInfo: Clone {
    /// Unique set identifier
    fn id(&self) -> &str;
    /// Set display name
    fn name(&self) -> &str;
    /// Level as a display string
    fn level_str(&self) -> String;
    /// Subject tags
    fn subjects(&self) -> &[String];
    /// Number of playable items
    fn item_count(&self) -> usize;
    /// Label for item type ("questions", "cards"…)
    fn item_label() -> &'static str;
}

/// DRY macro for GameSetInfo where level is a String.
/// QcmSet has an enum level — uses a manual impl.
macro_rules! impl_game_set_info {
    ($ty:ty, $items:ident, $label:literal) => {
        impl GameSetInfo for $ty {
            fn id(&self) -> &str { &self.id }
            fn name(&self) -> &str { &self.name }
            fn level_str(&self) -> String {
                self.level.clone()
            }
            fn subjects(&self) -> &[String] {
                &self.subjects
            }
            fn item_count(&self) -> usize {
                self.$items.len()
            }
            fn item_label() -> &'static str {
                $label
            }
        }
    };
}

// QcmSet: level is Level enum, needs as_str()
impl GameSetInfo for QcmSet {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { &self.name }
    fn level_str(&self) -> String {
        self.level.as_str().to_string()
    }
    fn subjects(&self) -> &[String] {
        &self.subjects
    }
    fn item_count(&self) -> usize {
        self.questions.len()
    }
    fn item_label() -> &'static str { "questions" }
}

// All others: level is String
impl_game_set_info!(FlashcardSet, cards, "cards");
impl_game_set_info!(
    TrueOrFalseSet, statements, "statements"
);
impl_game_set_info!(
    OpenQuestionSet, questions, "questions"
);
impl_game_set_info!(
    KeywordSet, questions, "questions"
);
impl_game_set_info!(
    OrderPhraseSet, questions, "questions"
);
impl_game_set_info!(
    FillBlankSet, questions, "questions"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::intello_types::Level;

    #[test]
    fn test_qcm_set_level_str_returns_enum_str() {
        let set = QcmSet {
            id: "1".into(),
            user_id: "u".into(),
            name: "Q".into(),
            description: "D".into(),
            level: Level::Hard,
            language: "en".into(),
            subjects: vec!["math".into()],
            questions: vec![],
        };
        assert_eq!(set.level_str(), "hard");
    }

    #[test]
    fn test_flashcard_set_item_label_is_cards() {
        assert_eq!(
            FlashcardSet::item_label(),
            "cards",
        );
    }

    #[test]
    fn test_true_false_set_item_label() {
        assert_eq!(
            TrueOrFalseSet::item_label(),
            "statements",
        );
    }

    #[test]
    fn test_trait_item_count_empty() {
        let set = FillBlankSet {
            id: "1".into(),
            name: "FB".into(),
            description: "D".into(),
            level: "easy".into(),
            language: "en".into(),
            subjects: vec![],
            questions: vec![],
        };
        assert_eq!(set.item_count(), 0);
    }

    #[test]
    fn test_all_item_labels() {
        let cases = vec![
            (QcmSet::item_label(), "questions"),
            (FlashcardSet::item_label(), "cards"),
            (
                TrueOrFalseSet::item_label(),
                "statements",
            ),
            (
                OpenQuestionSet::item_label(),
                "questions",
            ),
            (KeywordSet::item_label(), "questions"),
            (
                OrderPhraseSet::item_label(),
                "questions",
            ),
            (FillBlankSet::item_label(), "questions"),
        ];
        for (label, expected) in cases {
            assert_eq!(
                label, expected,
                "item_label mismatch",
            );
        }
    }

    #[test]
    fn test_trait_id_name_subjects() {
        let set = KeywordSet {
            id: "abc".into(),
            name: "My Set".into(),
            description: "D".into(),
            level: "hard".into(),
            language: "en".into(),
            subjects: vec![
                "math".into(),
                "science".into(),
            ],
            questions: vec![],
        };
        assert_eq!(set.id(), "abc");
        assert_eq!(set.name(), "My Set");
        assert_eq!(set.subjects().len(), 2);
    }

    #[test]
    fn test_string_level_str_clones_value() {
        let cases =
            vec!["easy", "medium", "hard", "custom"];
        for level in cases {
            let set = TrueOrFalseSet {
                id: "1".into(),
                name: "T".into(),
                description: "D".into(),
                level: level.into(),
                language: "en".into(),
                subjects: vec![],
                statements: vec![],
            };
            assert_eq!(
                set.level_str(),
                level,
                "level_str for '{level}'",
            );
        }
    }
}

//! Order Phrases — domain types (re-exported from the
//! shared `domain` module to keep a single source of
//! truth for serde shapes).

pub use crate::domain::order_phrase_types::{
    CreateOrderPhraseResponse, OrderPhraseQuestion,
    OrderPhraseSet, OrderPhraseSetListResponse,
    OrderPhraseWord,
};

/// Build the user phrase from a list of placed words.
pub fn join_words(words: &[String]) -> String {
    words.join(" ")
}

/// Toggle a word in/out of the placed list.
/// Returns the new list (immutable transform).
pub fn toggle_placed(
    placed: &[String],
    word: &str,
) -> Vec<String> {
    if placed.iter().any(|w| w == word) {
        placed.iter().filter(|w| *w != word).cloned().collect()
    } else {
        let mut next = placed.to_vec();
        next.push(word.to_string());
        next
    }
}

/// Return true when the user phrase matches the original.
pub fn is_phrase_correct(
    user_phrase: &str,
    original: &str,
) -> bool {
    user_phrase == original
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_words_with_three_words() {
        let result = join_words(&[
            "Le".into(),
            "chat".into(),
            "dort".into(),
        ]);
        assert_eq!(result, "Le chat dort");
    }

    #[test]
    fn test_join_words_with_empty_list() {
        let result = join_words(&[]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_toggle_placed_adds_missing_word() {
        let placed: Vec<String> = vec!["Le".into()];
        let next = toggle_placed(&placed, "chat");
        assert_eq!(next, vec!["Le", "chat"]);
    }

    #[test]
    fn test_toggle_placed_removes_existing_word() {
        let placed: Vec<String> =
            vec!["Le".into(), "chat".into()];
        let next = toggle_placed(&placed, "Le");
        assert_eq!(next, vec!["chat"]);
    }

    #[test]
    fn test_is_phrase_correct_matches_original() {
        assert!(is_phrase_correct(
            "Le chat dort",
            "Le chat dort",
        ));
    }

    #[test]
    fn test_is_phrase_correct_rejects_different() {
        assert!(!is_phrase_correct(
            "chat Le dort",
            "Le chat dort",
        ));
    }
}

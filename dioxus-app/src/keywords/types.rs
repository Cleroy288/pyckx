//! Keywords domain types — re-exported for module use.

pub use crate::domain::keywords_types::{
    CreateKeywordsResponse, Keyword, KeywordQuestion,
    KeywordSet, KeywordSetListResponse,
};

/// True when `selected` matches the correct keyword IDs
/// of `question` exactly (same set, no extra, no missing).
pub fn is_answer_correct(
    question: &KeywordQuestion,
    selected: &[String],
) -> bool {
    let correct_ids = collect_correct_ids(question);
    correct_ids.len() == selected.len()
        && correct_ids.iter().all(|id| selected.contains(id))
}

/// Return all IDs of correct keywords in `question`.
fn collect_correct_ids(
    question: &KeywordQuestion,
) -> Vec<String> {
    question
        .keywords
        .iter()
        .filter(|k| k.is_correct)
        .map(|k| k.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_question() -> KeywordQuestion {
        KeywordQuestion {
            id: "q1".into(),
            statement: "Pick rust words".into(),
            explanation: "rust is correct".into(),
            keywords: vec![
                make_keyword("k1", "rust", true),
                make_keyword("k2", "python", false),
                make_keyword("k3", "cargo", true),
            ],
        }
    }

    fn make_keyword(
        id: &str,
        word: &str,
        is_correct: bool,
    ) -> Keyword {
        Keyword {
            id: id.into(),
            word: word.into(),
            is_correct,
        }
    }

    #[test]
    fn test_is_answer_correct_with_exact_match() {
        let q = make_question();
        let selected =
            vec!["k1".to_string(), "k3".to_string()];
        assert!(is_answer_correct(&q, &selected));
    }

    #[test]
    fn test_is_answer_correct_with_missing_one() {
        let q = make_question();
        let selected = vec!["k1".to_string()];
        assert!(!is_answer_correct(&q, &selected));
    }

    #[test]
    fn test_is_answer_correct_with_extra_wrong() {
        let q = make_question();
        let selected = vec![
            "k1".to_string(),
            "k2".to_string(),
            "k3".to_string(),
        ];
        assert!(!is_answer_correct(&q, &selected));
    }

    #[test]
    fn test_is_answer_correct_with_empty_selection() {
        let q = make_question();
        assert!(!is_answer_correct(&q, &[]));
    }
}

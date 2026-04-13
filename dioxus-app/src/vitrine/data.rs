//! Static copy for the vitrine page.
//!
//! All user-facing strings live here so translators and
//! designers have one file to edit.

/// Single hero stat (big value + caption).
pub struct Stat {
    pub value: &'static str,
    pub label: &'static str,
}

/// Hero stats row.
pub const STATS: [Stat; 3] = [
    Stat { value: "7+", label: "Types de jeux" },
    Stat { value: "10s", label: "PDF \u{2192} Quiz" },
    Stat { value: "0\u{20AC}", label: "Gratuit" },
];

/// Pricing section bullet points.
pub const PRICE_FEATURES: [&str; 6] = [
    "7 types de jeux illimites",
    "Cours generes par IA",
    "Collection DVD incluse",
    "Generations AI illimitees",
    "Support prioritaire",
    "Aucune carte requise",
];

/// One entry in the "game formats" grid.
pub struct GameType {
    pub tag: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub featured: bool,
}

/// The seven game formats showcased in `AppsSection`.
pub const GAMES: [GameType; 7] = [
    GameType {
        tag: "01",
        name: "QCM",
        desc: "Questions a choix multiples generees par IA",
        featured: true,
    },
    GameType {
        tag: "02",
        name: "Flashcards",
        desc: "Memorisation par repetition espacee",
        featured: false,
    },
    GameType {
        tag: "03",
        name: "Vrai / Faux",
        desc: "Validation rapide des concepts cles",
        featured: false,
    },
    GameType {
        tag: "04",
        name: "Questions ouvertes",
        desc: "Reponses libres corrigees par IA",
        featured: false,
    },
    GameType {
        tag: "05",
        name: "Mots-cles",
        desc: "Identifiez les termes essentiels du cours",
        featured: false,
    },
    GameType {
        tag: "06",
        name: "Ordre",
        desc: "Remettez les etapes dans le bon ordre",
        featured: false,
    },
    GameType {
        tag: "07",
        name: "Texte a trous",
        desc: "Completez les passages manquants",
        featured: false,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_has_three_entries() {
        assert_eq!(STATS.len(), 3);
    }

    #[test]
    fn test_price_features_has_six_entries() {
        assert_eq!(PRICE_FEATURES.len(), 6);
    }

    #[test]
    fn test_games_has_seven_entries() {
        assert_eq!(GAMES.len(), 7);
    }

    #[test]
    fn test_games_first_is_featured() {
        assert!(GAMES[0].featured);
    }

    #[test]
    fn test_games_only_qcm_is_featured() {
        let featured_count =
            GAMES.iter().filter(|g| g.featured).count();
        assert_eq!(featured_count, 1);
    }
}

//! Static data constants for the vitrine page

/// Single stat item (value + label)
pub struct Stat {
    /// Display value (e.g. "7+")
    pub value: &'static str,
    /// Description label
    pub label: &'static str,
}

/// Hero stats row data
pub const STATS: [Stat; 3] = [
    Stat {
        value: "7+",
        label: "Types de jeux",
    },
    Stat {
        value: "10s",
        label: "PDF \u{2192} Quiz",
    },
    Stat {
        value: "0\u{20AC}",
        label: "Gratuit",
    },
];

/// Intello featured-card bullet points
pub const INTELLO_FEATURES: [&str; 5] = [
    "7 types de jeux generes par IA",
    "PDF, Word, TXT \u{2192} Quiz en 10s",
    "Cours complets en 5 etapes",
    "Correction automatique IA",
    "Mode rapide sans compte",
];

/// Pricing section feature list
pub const PRICE_FEATURES: [&str; 6] = [
    "Intello \u{2014} Acces complet",
    "Collection \u{2014} Acces complet",
    "Toutes les futures apps",
    "Generations AI illimitees",
    "Support prioritaire",
    "Aucune carte requise",
];

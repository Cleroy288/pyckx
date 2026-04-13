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

/// Pricing section feature list
pub const PRICE_FEATURES: [&str; 6] = [
    "7 types de jeux illimites",
    "Cours generes par IA",
    "Collection DVD incluse",
    "Generations AI illimitees",
    "Support prioritaire",
    "Aucune carte requise",
];

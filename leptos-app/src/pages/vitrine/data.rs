// ** data.rs **
// ==> Data constants for the vitrine page

#[derive(Clone)]
pub struct Card {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub const APP_CARDS: &[Card] = &[
    Card {
        icon: "Book",
        title: "Intello",
        description: "Your complete student companion. Organize courses, track progress, and excel in your studies with powerful learning tools.",
    },
    Card {
        icon: "Disc",
        title: "Collection",
        description: "Manage your DVD collection effortlessly. Catalog, organize, and discover your favorite movies and shows.",
    },
];

#[derive(Clone)]
pub struct PricingTier {
    pub name: &'static str,
    pub price: &'static str,
    pub period: &'static str,
    pub features: &'static [&'static str],
}

pub const PRICING_TIERS: &[PricingTier] = &[
    PricingTier {
        name: "Monthly",
        price: "5",
        period: "month",
        features: &["All apps included", "All features", "Regular updates"],
    },
    PricingTier {
        name: "Yearly",
        price: "50",
        period: "year",
        features: &["All apps included", "All features", "Regular updates", "Save 17%"],
    },
];

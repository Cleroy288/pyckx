//! Intello Service - Game card data and navigation

use crate::shared::components::IconType;

#[derive(Clone, Copy, PartialEq)]
pub enum GameVariant {
    CreateAI,
    CreateManual,
    Play,
}

#[derive(Clone)]
pub struct GameCardData {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: IconType,
    pub variant: GameVariant,
}

/// Returns the list of "Create" games
pub fn get_create_games() -> Vec<GameCardData> {
    vec![
        GameCardData { id: "manual-qcm", title: "Manual QCM", description: "Create manually", icon: IconType::Brain, variant: GameVariant::CreateManual },
        GameCardData { id: "ai-qcm", title: "AI QCM", description: "From documents", icon: IconType::Sparkles, variant: GameVariant::CreateAI },
        GameCardData { id: "ai-open", title: "Open Questions", description: "AI-graded", icon: IconType::Sparkles, variant: GameVariant::CreateAI },
        GameCardData { id: "ai-flashcard", title: "Flashcards", description: "Study cards", icon: IconType::Sparkles, variant: GameVariant::CreateAI },
        GameCardData { id: "ai-true-false", title: "True or False", description: "Verification", icon: IconType::Sparkles, variant: GameVariant::CreateAI },
        GameCardData { id: "ai-keywords", title: "Keywords", description: "Recognition", icon: IconType::Sparkles, variant: GameVariant::CreateAI },
        GameCardData { id: "ai-order-phrase", title: "Order Phrase", description: "Word ordering", icon: IconType::Sparkles, variant: GameVariant::CreateAI },
        GameCardData { id: "ai-fill-blank", title: "Fill Blank", description: "Complete text", icon: IconType::Sparkles, variant: GameVariant::CreateAI },
    ]
}

/// Returns the list of "Play" games
pub fn get_play_games() -> Vec<GameCardData> {
    vec![
        GameCardData { id: "play-qcm", title: "QCM Sets", description: "Multiple choice", icon: IconType::Brain, variant: GameVariant::Play },
        GameCardData { id: "play-open", title: "Open Questions", description: "Written answers", icon: IconType::FileText, variant: GameVariant::Play },
        GameCardData { id: "play-flashcard", title: "Flashcards", description: "Study cards", icon: IconType::Layers, variant: GameVariant::Play },
        GameCardData { id: "play-true-false", title: "True or False", description: "Statements", icon: IconType::CheckCircle, variant: GameVariant::Play },
        GameCardData { id: "play-keywords", title: "Keywords", description: "Recognition", icon: IconType::Tags, variant: GameVariant::Play },
        GameCardData { id: "play-order-phrase", title: "Order Phrase", description: "Arrange words", icon: IconType::ListOrdered, variant: GameVariant::Play },
        GameCardData { id: "play-fill-blank", title: "Fill Blank", description: "Complete text", icon: IconType::TextCursor, variant: GameVariant::Play },
    ]
}

/// Returns course-related cards
pub fn get_course_cards() -> Vec<GameCardData> {
    vec![
        GameCardData { id: "courses", title: "My Courses", description: "View all", icon: IconType::BookOpen, variant: GameVariant::CreateAI },
        GameCardData { id: "create-course", title: "New Course", description: "Start fresh", icon: IconType::Plus, variant: GameVariant::CreateAI },
    ]
}

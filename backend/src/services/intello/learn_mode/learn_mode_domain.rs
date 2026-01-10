/*
 *
 * le mode learn c'est quand un user veut apprendre un truc
 * on dois donc continuer à lui générer des cours tant que il n'a pas 90% de réussite.
 *
 * on doit donc continuer à lui générer des qcm , etc tant que il n'a pas 90% de réussite
 *
 * quand un user fait un cours à la fin on lui propose un mode learn,
 * ça veut dire que on lui laisse le cours affiché et on lui génère une section extra sous la synthèse
 * et donc on lui génère un nouveau text (synthèse à apprendre)
 * et on lui génère des jeux pour apprendre ce text
 * puis une fois terminé le joueur nous envoi ses résultats, si il n'a pas plus de 90% on arrête pas le mode learn
 * sinon on re généère des jeux jusqu'a ce que il atteigne les 90%
 *
 * le jeux pour le mode learn devraient avoir plus d'info que les jeux normaux, le but est que le user apprend
 * vraiment avec ces jeux
 *
 *
 * process :
 * - 1 quand un user clique sur "learn mode" en dessou d'un cours, on récupère le résultat de ses jeux pour ce cours
 * on récupère aussi le sujet de son cours etc ...
 * - 2 on génère un text à apprendre (synthèse)
 * avec des jeux dedans , qcm, frai faux et aussi openquestion
 * les jeux doivent avoir plus de text et explications que les jeux normaux
 * - 3 à la fin on réucpère les résultats du user, on analayse ses erreurs, et si il n'a pas 90%
 * on re généère une synthèse basé sur ses erreurs et on re génère des jeux
 * - 4 une fois que le user à réussi on lui génère une nouvelle synthèse (de fin)
 *
 *
 */

use crate::http_api::data_transfer_object::intello::course::CourseModule;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a Learn Mode session for a specific user and course.
/// Tracks the user's progress through multiple learning rounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnModeSession {
    /// Unique identifier for the learn session
    pub id: String,
    /// The user engaging in learning
    pub user_id: String,
    /// The course being learned
    pub course_id: String,
    /// Current status of the session
    pub status: LearnSessionStatus,
    /// History of rounds (attempts)
    pub rounds: Vec<LearnRound>,
    /// When the session started
    pub created_at: DateTime<Utc>,
    /// When the session was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearnSessionStatus {
    InProgress,
    Completed,
    Abandoned,
}

/// Represents a single iteration in the Learn Mode process.
/// Includes the generated content (to learn) and the results of testing that content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnRound {
    /// Unique identifier for the round
    pub id: String,
    /// Sequential number of the round (1, 2, 3...)
    pub number: u32,
    /// The content generated for this round.
    /// Reuses `CourseModule` because it perfectly encapsulates:
    /// - Title
    /// - Text blocks (Synthesis to learn)
    /// - Schema blocks (Visual aids)
    /// - Game blocks (Exercises to test knowledge)
    pub content: CourseModule,
    /// The results of this round, if completed.
    pub result: Option<LearnRoundResult>,
    /// When this round was generated
    pub created_at: DateTime<Utc>,
}

/// The outcome of a Learn Round.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnRoundResult {
    /// The specific round this result belongs to
    pub round_id: String,
    /// Overall score for this round (0-100)
    pub global_score: u32,
    /// Individual results for each game played in this round
    pub game_results: Vec<LearnGameResult>,
    /// AI analysis of the user's performance (feedback)
    pub analysis: Option<String>,
    /// When the round was completed
    pub completed_at: DateTime<Utc>,
}

/// Detailed result for a specific game within a round.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnGameResult {
    /// Identifier of the game (within the content block)
    pub game_id: String,
    /// Type of game (qcm, true_false, etc.)
    pub game_type: String,
    /// Score achieved (0-100)
    pub score: u32,
    /// Maximum possible score
    pub max_score: u32,
    /// JSON structure storing user's specific answers for detailed analysis
    pub user_answers: serde_json::Value,
    /// Difficulty level of this specific game instance
    pub difficulty: String,
}

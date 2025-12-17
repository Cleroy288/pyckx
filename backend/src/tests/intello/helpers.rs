//! Intello test helpers

use crate::infrastructure::{JsonFlashcardRepository, JsonOpenQuestionRepository, JsonQcmRepository};
use crate::services::{IntelloService, OpenRouterService};
use crate::shared::OpenQuestionCache;
use std::sync::Arc;

/// Path to test QCM sets JSON file
pub const TEST_QCMSET_FILE: &str = "data/intello/qcmset.json";
/// Path to test AI-generated QCM sets JSON file
pub const TEST_AI_QCMSET_FILE: &str = "data/intello/ai_qcmset.json";
/// Path to test open question sets JSON file
pub const TEST_OPEN_QUESTION_FILE: &str = "data/intello/openquestion.json";
/// Path to test flashcard sets JSON file
pub const TEST_FLASHCARD_FILE: &str = "data/intello/flashcard.json";

/// Create a test IntelloService with all required dependencies
pub fn create_test_service() -> IntelloService {
    let qcm_repo = Arc::new(JsonQcmRepository::new(TEST_QCMSET_FILE));
    let ai_qcm_repo = Arc::new(JsonQcmRepository::new(TEST_AI_QCMSET_FILE));
    let open_question_repo = Arc::new(JsonOpenQuestionRepository::new(TEST_OPEN_QUESTION_FILE));
    let flashcard_repo = Arc::new(JsonFlashcardRepository::new(TEST_FLASHCARD_FILE));
    let openrouter_service = Arc::new(OpenRouterService::new(String::new()));
    let open_question_cache = Arc::new(OpenQuestionCache::new());

    IntelloService::new(
        qcm_repo,
        ai_qcm_repo,
        open_question_repo,
        flashcard_repo,
        openrouter_service,
        open_question_cache,
    )
}

/// Get test user ID (using a fixed test user for unit tests)
pub fn get_test_user_id() -> String {
    "test-user-intello".to_string()
}

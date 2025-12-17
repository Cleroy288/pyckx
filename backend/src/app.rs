//! Application state - Main application struct holding all services

use crate::apps::{CollectionApp, IntelloApp};
use crate::config::{Config, ConfigError};
use crate::domain::{AppModule, SessionStore};
use crate::infrastructure::{
    SupabaseAppRepository, SupabaseCollectionRepository, SupabaseDvdRepository,
    SupabaseFlashcardRepository, SupabaseKeywordsRepository, SupabaseOpenQuestionRepository, SupabaseOrderPhraseRepository, SupabaseQcmRepository,
    SupabaseTrueOrFalseRepository, SupabaseUserAppRepository,
};
use crate::services::{AppService, AuthService, CollectionService, IntelloService, OpenRouterService};
use crate::shared::OpenQuestionCache;
use std::sync::Arc;
use tracing::info;

// Intello repositories now use Supabase PostgreSQL storage
// All data is stored in tables: qcm_sets, open_question_sets, flashcard_sets, intello_true_false_sets

// == APPLICATION STATE // ==

/// Main application struct - holds all services and shared state
///
/// Architecture: Handler → IntelloService → Repositories / OpenRouterService
#[derive(Debug)]
pub struct App {
    pub name: String,
    pub version: String,
    pub config: Config,
    pub auth: AuthService,
    // Services (business logic layer)
    pub collection_service: CollectionService,
    pub app_service: AppService,
    pub intello_service: IntelloService,
    // Apps
    #[allow(dead_code)] // App metadata, used for future app registry
    pub collection: CollectionApp,
    #[allow(dead_code)] // App metadata, used for future app registry
    pub intello: IntelloApp,
}

impl App {
    /// Create a new application instance.
    ///
    /// # Returns
    /// - `Ok(App)` if configuration loads successfully
    /// - `Err(ConfigError)` if any required config is missing
    pub fn new() -> Result<Self, ConfigError> {
        let cfg = Config::from_env()?;
        let sessions = SessionStore::new();
        let auth = AuthService::new(&cfg, sessions);
        let collection = CollectionApp::new();
        let intello = IntelloApp::new();

        // Create Supabase repositories
        let collection_repo = Arc::new(SupabaseCollectionRepository::new(&cfg));
        let dvd_repo = Arc::new(SupabaseDvdRepository::new(&cfg));
        let app_repository = Arc::new(SupabaseAppRepository::new(&cfg));
        let user_app_repository = Arc::new(SupabaseUserAppRepository::new(&cfg));

        // Create Supabase repositories for Intello
        let qcm_repo = Arc::new(SupabaseQcmRepository::new(&cfg));
        let ai_qcm_repo = Arc::new(SupabaseQcmRepository::new(&cfg)); // Same table, user_id distinguishes
        let open_question_repo = Arc::new(SupabaseOpenQuestionRepository::new(&cfg));
        let flashcard_repo = Arc::new(SupabaseFlashcardRepository::new(&cfg));
        let true_false_repo = Arc::new(SupabaseTrueOrFalseRepository::new(&cfg));
        let keywords_repo = Arc::new(SupabaseKeywordsRepository::new(&cfg));
        let order_phrase_repo = Arc::new(SupabaseOrderPhraseRepository::new(&cfg));

        // Create caches
        let open_question_cache = Arc::new(OpenQuestionCache::new());

        // Create OpenRouter service (use empty string if no API key configured)
        let openrouter_api_key = cfg.openrouter_api_key.clone().unwrap_or_default();
        let openrouter_service = Arc::new(OpenRouterService::new(openrouter_api_key));

        // Create services with all dependencies injected
        let collection_service = CollectionService::new(collection_repo, dvd_repo);
        let app_service = AppService::new(app_repository, user_app_repository);
        
        // IntelloService now handles all game types with unified business logic
        let intello_service = IntelloService::new(
            qcm_repo,
            ai_qcm_repo,
            open_question_repo,
            flashcard_repo,
            true_false_repo,
            keywords_repo,
            order_phrase_repo,
            openrouter_service,
            open_question_cache,
        );

        info!(
            collection_app = %collection.name(),
            intello_app = %intello.name(),
            "Apps initialized with Supabase repositories"
        );

        Ok(Self {
            name: "LAPP".to_string(),
            version: "0.1.0".to_string(),
            config: cfg,
            auth,
            collection_service,
            app_service,
            intello_service,
            collection,
            intello,
        })
    }
}

impl Clone for App {
    fn clone(&self) -> Self {
        // Re-create the app with fresh repository instances
        // This is safe because the repositories are stateless
        let collection_repo = Arc::new(SupabaseCollectionRepository::new(&self.config));
        let dvd_repo = Arc::new(SupabaseDvdRepository::new(&self.config));
        let app_repository = Arc::new(SupabaseAppRepository::new(&self.config));
        let user_app_repository = Arc::new(SupabaseUserAppRepository::new(&self.config));
        
        // Intello repositories (Supabase)
        let qcm_repo = Arc::new(SupabaseQcmRepository::new(&self.config));
        let ai_qcm_repo = Arc::new(SupabaseQcmRepository::new(&self.config));
        let open_question_repo = Arc::new(SupabaseOpenQuestionRepository::new(&self.config));
        let flashcard_repo = Arc::new(SupabaseFlashcardRepository::new(&self.config));
        let true_false_repo = Arc::new(SupabaseTrueOrFalseRepository::new(&self.config));
        let keywords_repo = Arc::new(SupabaseKeywordsRepository::new(&self.config));
        let order_phrase_repo = Arc::new(SupabaseOrderPhraseRepository::new(&self.config));
        let open_question_cache = Arc::new(OpenQuestionCache::new());
        let openrouter_api_key = self.config.openrouter_api_key.clone().unwrap_or_default();
        let openrouter_service = Arc::new(OpenRouterService::new(openrouter_api_key));

        Self {
            name: self.name.clone(),
            version: self.version.clone(),
            config: self.config.clone(),
            auth: self.auth.clone(),
            collection_service: CollectionService::new(collection_repo, dvd_repo),
            app_service: AppService::new(app_repository, user_app_repository),
            intello_service: IntelloService::new(
                qcm_repo,
                ai_qcm_repo,
                open_question_repo,
                flashcard_repo,
                true_false_repo,
                keywords_repo,
                order_phrase_repo,
                openrouter_service,
                open_question_cache,
            ),
            collection: self.collection.clone(),
            intello: self.intello.clone(),
        }
    }
}


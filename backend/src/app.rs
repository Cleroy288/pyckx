//! Application state - Main application struct holding all services

use crate::apps::{CollectionApp, IntelloApp};
use crate::config::{Config, ConfigError};
use crate::domain::{AppModule, SessionStore};
use crate::infrastructure::{
    SupabaseAppRepository, SupabaseCollectionRepository, SupabaseDvdRepository,
    SupabaseFillBlankRepository, SupabaseFlashcardRepository, SupabaseKeywordsRepository,
    SupabaseOpenQuestionRepository, SupabaseOrderPhraseRepository, SupabaseQcmRepository,
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
/// Architecture: Handler → Service (Arc-wrapped) → Repositories
///
/// Services are wrapped in Arc to enable cheap cloning across worker threads.
/// Actix-web clones app state for each worker, so Arc avoids recreating
/// 12+ repositories on every clone.
#[derive(Debug)]
pub struct App {
    pub name: String,
    pub version: String,
    pub config: Config,
    pub auth: AuthService,
    // Services (business logic layer) - Arc-wrapped for cheap cloning
    pub collection_service: Arc<CollectionService>,
    pub app_service: Arc<AppService>,
    pub intello_service: Arc<IntelloService>,
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

        // Create Supabase repositories for Intello (bundled for cleaner construction)
        let intello_repos = crate::services::IntelloRepositories {
            qcm_repo: Arc::new(SupabaseQcmRepository::new(&cfg)),
            ai_qcm_repo: Arc::new(SupabaseQcmRepository::new(&cfg)), // Same table, user_id distinguishes
            open_question_repo: Arc::new(SupabaseOpenQuestionRepository::new(&cfg)),
            flashcard_repo: Arc::new(SupabaseFlashcardRepository::new(&cfg)),
            true_false_repo: Arc::new(SupabaseTrueOrFalseRepository::new(&cfg)),
            keywords_repo: Arc::new(SupabaseKeywordsRepository::new(&cfg)),
            order_phrase_repo: Arc::new(SupabaseOrderPhraseRepository::new(&cfg)),
            fill_blank_repo: Arc::new(SupabaseFillBlankRepository::new(&cfg)),
        };

        // Create caches
        let open_question_cache = Arc::new(OpenQuestionCache::new());

        // Create OpenRouter service (use empty string if no API key configured)
        let openrouter_api_key = cfg.openrouter_api_key.clone().unwrap_or_default();
        let google_ai_key = cfg.google_ai_key.clone();
        let openrouter_service = Arc::new(OpenRouterService::with_google_key(openrouter_api_key, google_ai_key));

        // Create services with all dependencies injected (wrapped in Arc for cheap cloning)
        let collection_service = Arc::new(CollectionService::new(collection_repo, dvd_repo));
        let app_service = Arc::new(AppService::new(app_repository, user_app_repository));
        
        // IntelloService uses builder pattern for cleaner construction
        let intello_service = Arc::new(
            IntelloService::builder()
                .with_repositories(intello_repos)
                .with_openrouter(openrouter_service)
                .with_cache(open_question_cache)
                .build()
                .expect("IntelloService must have all dependencies"),
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
        // Arc::clone is O(1) - just increments reference count
        // No repository recreation needed!
        Self {
            name: self.name.clone(),
            version: self.version.clone(),
            config: self.config.clone(),
            auth: self.auth.clone(),
            collection_service: Arc::clone(&self.collection_service),
            app_service: Arc::clone(&self.app_service),
            intello_service: Arc::clone(&self.intello_service),
            collection: self.collection.clone(),
            intello: self.intello.clone(),
        }
    }
}


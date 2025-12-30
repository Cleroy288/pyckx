//! Application state - Main application struct holding all services

use crate::services::app_registry::{CollectionApp, IntelloApp};
use crate::configs::{Config, ConfigError};
use crate::services::app_registry::registry_domain::AppModule;
use crate::infra::SessionStore;
use crate::infra::{
    SupabaseAiUsageRepository, SupabaseAppRepository, SupabaseCollectionRepository, SupabaseDvdRepository,
    SupabaseFillBlankRepository, SupabaseFlashcardRepository, SupabaseHttpClient, SupabaseKeywordsRepository,
    SupabaseOpenQuestionRepository, SupabaseOrderPhraseRepository, SupabaseQcmRepository,
    SupabaseTrueOrFalseRepository, SupabaseUserAppRepository,
    SupabaseCourseRepository, SupabaseStudySessionRepository,
};
use crate::services::{AppService, AuthService, CollectionService, IntelloService, OpenRouterService};
use crate::services::intello::open_question_cache_service::OpenQuestionCache;
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
    pub openrouter_service: Arc<OpenRouterService>,  // AI service for course generation
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

        /* Create shared Supabase HTTP client (single connection pool) */
        let supabase_client = Arc::new(SupabaseHttpClient::new(&cfg));

        /* Create Supabase repositories (non-Intello) */
        let collection_repo = Arc::new(SupabaseCollectionRepository::new(Arc::clone(&supabase_client)));
        let dvd_repo = Arc::new(SupabaseDvdRepository::new(Arc::clone(&supabase_client)));
        let app_repository = Arc::new(SupabaseAppRepository::new(Arc::clone(&supabase_client)));
        let user_app_repository = Arc::new(SupabaseUserAppRepository::new(Arc::clone(&supabase_client)));

        /* Create Supabase repositories for Intello (bundled for cleaner construction) */
        let intello_repos = crate::services::IntelloRepositories {
            qcm_repo: Arc::new(SupabaseQcmRepository::new(Arc::clone(&supabase_client))),
            ai_qcm_repo: Arc::new(SupabaseQcmRepository::new(Arc::clone(&supabase_client))),
            open_question_repo: Arc::new(SupabaseOpenQuestionRepository::new(Arc::clone(&supabase_client))),
            flashcard_repo: Arc::new(SupabaseFlashcardRepository::new(Arc::clone(&supabase_client))),
            true_false_repo: Arc::new(SupabaseTrueOrFalseRepository::new(Arc::clone(&supabase_client))),
            keywords_repo: Arc::new(SupabaseKeywordsRepository::new(Arc::clone(&supabase_client))),
            order_phrase_repo: Arc::new(SupabaseOrderPhraseRepository::new(Arc::clone(&supabase_client))),
            fill_blank_repo: Arc::new(SupabaseFillBlankRepository::new(Arc::clone(&supabase_client))),
            course_repo: Arc::new(SupabaseCourseRepository::new(Arc::clone(&supabase_client))),
            ai_usage_repo: Arc::new(SupabaseAiUsageRepository::new(Arc::clone(&supabase_client))),
            study_session_repo: Arc::new(SupabaseStudySessionRepository::new(Arc::clone(&supabase_client))),
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
                .with_openrouter(Arc::clone(&openrouter_service))
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
            openrouter_service,
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
            openrouter_service: Arc::clone(&self.openrouter_service),
            collection: self.collection.clone(),
            intello: self.intello.clone(),
        }
    }
}


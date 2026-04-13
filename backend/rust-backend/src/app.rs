//! Application state - Main application struct holding all services

use crate::configs::{Config, ConfigError};
use crate::infra::openrouter::OpenRouterClient;
use crate::infra::SessionStore;
use crate::infra::{
    SupabaseAiUsageRepository, SupabaseAppRepository, SupabaseCollectionRepository,
    SupabaseCourseRepository, SupabaseDvdRepository, SupabaseFillBlankRepository,
    SupabaseFlashcardRepository, SupabaseHttpClient, SupabaseKeywordsRepository,
    SupabaseOpenQuestionRepository, SupabaseOrderPhraseRepository, SupabaseQcmRepository,
    SupabaseStudySessionRepository, SupabaseTrueOrFalseRepository,
    SupabaseUserAppRepository,
};
use crate::services::app_registry::registry_domain::AppModule;
use crate::services::app_registry::{CollectionApp, StudyApp};
use crate::services::games::open_question::open_question_cache_service::OpenQuestionCache;
use crate::services::{AppService, AuthService, CollectionService, StudyService};
use std::sync::Arc;
use tracing::info;

// Study repositories use Supabase PostgreSQL storage

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
    pub study_service: Arc<StudyService>,
    pub openrouter_client: Arc<OpenRouterClient>,
    /// Shared HTTP client for proxy requests
    pub http_client: reqwest::Client,
    // Apps
    #[allow(dead_code)] // App metadata, used for future app registry
    pub collection: CollectionApp,
    #[allow(dead_code)] // App metadata, used for future app registry
    pub study: StudyApp,
}

impl App {
    /// Create a new application instance.
    ///
    /// # Returns
    /// - `Ok(App)` if configuration loads successfully
    /// - `Err(ConfigError)` if any required config is missing
    pub async fn new() -> Result<Self, ConfigError> {
        let cfg = Config::from_env()?;

        /* Create shared Supabase HTTP client (single connection pool) */
        let supabase_client = Arc::new(SupabaseHttpClient::new(&cfg));

        /* Create Redis-backed session store */
        let redis_client = redis::Client::open(
            cfg.redis_url.as_str(),
        )
        .expect("Invalid REDIS_URL");
        let redis_conn = redis_client
            .get_connection_manager()
            .await
            .expect("Failed to connect to Redis");
        let sessions = SessionStore::new(redis_conn);

        let auth = AuthService::new(&cfg, sessions);
        let collection = CollectionApp::new();
        let study = StudyApp::new();

        /* Create Supabase repositories */
        let collection_repo = Arc::new(SupabaseCollectionRepository::new(
            Arc::clone(&supabase_client),
        ));
        let dvd_repo =
            Arc::new(SupabaseDvdRepository::new(Arc::clone(&supabase_client)));
        let app_repository =
            Arc::new(SupabaseAppRepository::new(Arc::clone(&supabase_client)));
        let user_app_repository = Arc::new(SupabaseUserAppRepository::new(
            Arc::clone(&supabase_client),
        ));

        /* Create Study repositories */
        let study_repos = crate::services::StudyRepositories {
            qcm_repo: Arc::new(SupabaseQcmRepository::new(Arc::clone(
                &supabase_client,
            ))),
            ai_qcm_repo: Arc::new(SupabaseQcmRepository::new(Arc::clone(
                &supabase_client,
            ))),
            open_question_repo: Arc::new(SupabaseOpenQuestionRepository::new(
                Arc::clone(&supabase_client),
            )),
            flashcard_repo: Arc::new(SupabaseFlashcardRepository::new(
                Arc::clone(&supabase_client),
            )),
            true_false_repo: Arc::new(SupabaseTrueOrFalseRepository::new(
                Arc::clone(&supabase_client),
            )),
            keywords_repo: Arc::new(SupabaseKeywordsRepository::new(
                Arc::clone(&supabase_client),
            )),
            order_phrase_repo: Arc::new(SupabaseOrderPhraseRepository::new(
                Arc::clone(&supabase_client),
            )),
            fill_blank_repo: Arc::new(SupabaseFillBlankRepository::new(
                Arc::clone(&supabase_client),
            )),
            course_repo: Arc::new(SupabaseCourseRepository::new(Arc::clone(
                &supabase_client,
            ))),
            ai_usage_repo: Arc::new(SupabaseAiUsageRepository::new(
                Arc::clone(&supabase_client),
            )),
            study_session_repo: Arc::new(SupabaseStudySessionRepository::new(
                Arc::clone(&supabase_client),
            )),
        };

        // Create caches
        let open_question_cache = Arc::new(OpenQuestionCache::new());

        // Create OpenRouter client (warn if no API key)
        let openrouter_api_key = match &cfg.openrouter_api_key {
            Some(key) => key.clone(),
            None => {
                tracing::warn!(
                    "OpenRouter API key not set"
                );
                String::new()
            }
        };
        let google_ai_key = cfg.google_ai_key.clone();

        // Create services with all dependencies injected (wrapped in Arc for cheap cloning)
        let collection_service =
            Arc::new(CollectionService::new(collection_repo, dvd_repo));
        let app_service =
            Arc::new(AppService::new(app_repository, user_app_repository));

        // OpenRouter AI client
        let openrouter_client = Arc::new(OpenRouterClient::with_google_key(
            openrouter_api_key,
            google_ai_key,
        ));

        // Build Study service
        let study_service = Arc::new(
            StudyService::builder()
                .with_repositories(study_repos)
                .with_openrouter(Arc::clone(&openrouter_client))
                .with_cache(open_question_cache)
                .build()
                .expect("StudyService must have all dependencies"),
        );

        info!(
            collection_app = %collection.name(),
            study_app = %study.name(),
            "Apps initialized with Supabase repositories"
        );

        Ok(Self {
            name: "LAPP".to_string(),
            version: "0.1.0".to_string(),
            config: cfg,
            auth,
            collection_service,
            app_service,
            study_service,
            openrouter_client,
            http_client: reqwest::Client::new(),
            collection,
            study,
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
            study_service: Arc::clone(&self.study_service),
            openrouter_client: Arc::clone(&self.openrouter_client),
            http_client: self.http_client.clone(),
            collection: self.collection.clone(),
            study: self.study.clone(),
        }
    }
}

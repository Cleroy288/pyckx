//! AI Usage Operations - Logging and cost tracking

use tracing::{info, instrument, warn};

use super::ai_usage_domain::{
    AggregatedUsageStats, AiUsageInput, AiUsageLog,
    CreateAiUsageLog,
};
use super::ai_usage_stats::aggregate_usage_stats;
use crate::infra::openrouter::calculate_cost;
use crate::services::error_domain::StudyError;
use crate::services::StudyService;

impl StudyService {
    // ** log_ai_usage **
    // ==> Logs AI usage with automatic cost calculation
    //
    // @ input : AiUsageInput with user, model, feature, and token counts
    // @ returns : The created AiUsageLog record
    // @ errors : StorageError if logging fails
    #[instrument(skip(self, input), fields(user_id = %input.user_id, model = %input.model_id, feature = %input.feature_type))]
    pub async fn log_ai_usage(
        &self,
        input: AiUsageInput<'_>,
    ) -> Result<AiUsageLog, StudyError> {
        // Step 1: Calculate cost using centralized model registry
        let cost = calculate_cost(
            input.model_id,
            input.input_tokens,
            input.output_tokens,
        );

        // Step 2: Build usage log entry
        let create = CreateAiUsageLog {
            user_id: input.user_id.to_string(),
            model_id: input.model_id.to_string(),
            feature_type: input.feature_type.to_string(),
            input_tokens: input.input_tokens as i32,
            output_tokens: input.output_tokens as i32,
            input_cost_usd: cost.input_cost_usd,
            output_cost_usd: cost.output_cost_usd,
            total_cost_usd: cost.total_cost_usd,
        };

        // Step 3: Persist usage log to database
        let log = self
            .ai_usage_repo
            .log_usage(create)
            .await
            .map_err(|err| StudyError::storage(err.to_string()))?;

        // Step 4: Log the operation
        info!(
            input_tokens = input.input_tokens,
            output_tokens = input.output_tokens,
            total_cost = cost.total_cost_usd,
            "AI usage logged"
        );

        // Step 5: Return the created log
        Ok(log)
    }

    // ** try_log_ai_usage **
    // ==> Fire-and-forget AI usage logging (doesn't fail)
    //
    // @ input : AiUsageInput with user, model, feature, and token counts
    pub async fn try_log_ai_usage(&self, input: AiUsageInput<'_>) {
        // Step 1: Attempt to log usage, suppress errors
        if let Err(err) = self.log_ai_usage(input).await {
            // Step 2: Log warning if logging failed
            warn!("Failed to log AI usage: {}", err);
        }
    }

    /// Retrieves all AI usage logs
    pub async fn get_all_usage(
        &self,
    ) -> Result<Vec<AiUsageLog>, StudyError> {
        self.ai_usage_repo
            .get_all_usage()
            .await
            .map_err(|err| StudyError::storage(err.to_string()))
    }

    /// Retrieves aggregated usage statistics
    pub async fn get_aggregated_usage(
        &self,
    ) -> Result<AggregatedUsageStats, StudyError> {
        let logs = self.get_all_usage().await?;
        Ok(aggregate_usage_stats(&logs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::openrouter::OpenRouterClient;
    use crate::infra::{
        AiUsageRepository, CourseRepository,
        StudySessionRepository,
    };
    use crate::infra::{
        GameSetRepository, QcmRepository,
        OpenQuestionRepository,
    };
    use crate::services::games::open_question::open_question_cache_service::OpenQuestionCache;
    use crate::services::study_service::{
        StudyRepositories, StudyService,
    };
    use crate::services::{
        FillBlankSet, FlashcardSet, KeywordSet,
        OpenQuestionSet, OrderPhraseSet, QcmSet,
        TrueOrFalseSet,
    };
    use crate::services::course::domain::{
        Course, ResourceSummary, UserResource,
    };
    use crate::services::study_session::study_session_domain::StudySession;
    use crate::shared::AppError;
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    // -- Stub AiUsageRepository --

    struct StubAiUsageRepo {
        logs: Mutex<Vec<AiUsageLog>>,
    }

    impl StubAiUsageRepo {
        fn new(logs: Vec<AiUsageLog>) -> Self {
            Self { logs: Mutex::new(logs) }
        }

        fn empty() -> Self {
            Self::new(vec![])
        }
    }

    #[async_trait]
    impl AiUsageRepository for StubAiUsageRepo {
        async fn log_usage(
            &self,
            input: CreateAiUsageLog,
        ) -> Result<AiUsageLog, AppError> {
            let log = AiUsageLog {
                id: "log-1".to_string(),
                user_id: input.user_id,
                model_id: input.model_id,
                feature_type: input.feature_type,
                input_tokens: input.input_tokens,
                output_tokens: input.output_tokens,
                input_cost_usd: input.input_cost_usd,
                output_cost_usd: input.output_cost_usd,
                total_cost_usd: input.total_cost_usd,
                created_at: "2024-01-01".to_string(),
            };
            self.logs.lock().unwrap().push(log.clone());
            Ok(log)
        }

        async fn get_user_usage(
            &self,
            user_id: &str,
        ) -> Result<Vec<AiUsageLog>, AppError> {
            Ok(self
                .logs
                .lock()
                .unwrap()
                .iter()
                .filter(|l| l.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn get_all_usage(
            &self,
        ) -> Result<Vec<AiUsageLog>, AppError> {
            Ok(self.logs.lock().unwrap().clone())
        }
    }

    // -- Panic stubs for unused repos --

    struct PanicQcmRepo;
    #[async_trait]
    impl QcmRepository for PanicQcmRepo {
        async fn insert(
            &self,
            _: &QcmSet,
        ) -> Result<QcmSet, StudyError> {
            unimplemented!()
        }
        async fn find_by_id(
            &self,
            _: &str,
            _: &str,
        ) -> Result<Option<QcmSet>, StudyError> {
            unimplemented!()
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<QcmSet>, StudyError> {
            unimplemented!()
        }
        async fn update(
            &self,
            _: &QcmSet,
        ) -> Result<bool, StudyError> {
            unimplemented!()
        }
        async fn delete(
            &self,
            _: &str,
            _: &str,
        ) -> Result<bool, StudyError> {
            unimplemented!()
        }
    }

    struct PanicOpenQuestionRepo;
    #[async_trait]
    impl OpenQuestionRepository
        for PanicOpenQuestionRepo
    {
        async fn insert(
            &self,
            _: &OpenQuestionSet,
        ) -> Result<OpenQuestionSet, StudyError> {
            unimplemented!()
        }
        async fn find_by_id(
            &self,
            _: &str,
            _: &str,
        ) -> Result<Option<OpenQuestionSet>, StudyError>
        {
            unimplemented!()
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<OpenQuestionSet>, StudyError> {
            unimplemented!()
        }
    }

    struct PanicFlashcardRepo;
    #[async_trait]
    impl GameSetRepository<FlashcardSet>
        for PanicFlashcardRepo
    {
        async fn insert(
            &self,
            _: &FlashcardSet,
        ) -> Result<FlashcardSet, StudyError> {
            unimplemented!()
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<FlashcardSet>, StudyError> {
            unimplemented!()
        }
    }

    struct PanicTrueOrFalseRepo;
    #[async_trait]
    impl GameSetRepository<TrueOrFalseSet>
        for PanicTrueOrFalseRepo
    {
        async fn insert(
            &self,
            _: &TrueOrFalseSet,
        ) -> Result<TrueOrFalseSet, StudyError> {
            unimplemented!()
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<TrueOrFalseSet>, StudyError> {
            unimplemented!()
        }
    }

    struct PanicKeywordsRepo;
    #[async_trait]
    impl GameSetRepository<KeywordSet>
        for PanicKeywordsRepo
    {
        async fn insert(
            &self,
            _: &KeywordSet,
        ) -> Result<KeywordSet, StudyError> {
            unimplemented!()
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<KeywordSet>, StudyError> {
            unimplemented!()
        }
    }

    struct PanicOrderPhraseRepo;
    #[async_trait]
    impl GameSetRepository<OrderPhraseSet>
        for PanicOrderPhraseRepo
    {
        async fn insert(
            &self,
            _: &OrderPhraseSet,
        ) -> Result<OrderPhraseSet, StudyError> {
            unimplemented!()
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<OrderPhraseSet>, StudyError> {
            unimplemented!()
        }
    }

    struct PanicFillBlankRepo;
    #[async_trait]
    impl GameSetRepository<FillBlankSet>
        for PanicFillBlankRepo
    {
        async fn insert(
            &self,
            _: &FillBlankSet,
        ) -> Result<FillBlankSet, StudyError> {
            unimplemented!()
        }
        async fn find_by_user(
            &self,
            _: &str,
        ) -> Result<Vec<FillBlankSet>, StudyError> {
            unimplemented!()
        }
    }

    struct PanicCourseRepo;
    #[async_trait]
    impl CourseRepository for PanicCourseRepo {
        async fn create_course(
            &self,
            _: &Course,
        ) -> Result<Course, AppError> {
            unimplemented!()
        }
        async fn get_user_courses(
            &self,
            _: &str,
        ) -> Result<Vec<Course>, AppError> {
            unimplemented!()
        }
        async fn get_course_resources(
            &self,
            _: &str,
        ) -> Result<Vec<UserResource>, AppError> {
            unimplemented!()
        }
        async fn fetch_resources(
            &self,
            _: &[String],
        ) -> Result<Vec<UserResource>, AppError> {
            unimplemented!()
        }
        async fn get_user_resources(
            &self,
            _: &str,
        ) -> Result<Vec<ResourceSummary>, AppError> {
            unimplemented!()
        }
        async fn get_resource_by_id(
            &self,
            _: &str,
        ) -> Result<Option<UserResource>, AppError> {
            unimplemented!()
        }
        async fn resource_exists(
            &self,
            _: &str,
            _: &str,
        ) -> Result<bool, AppError> {
            unimplemented!()
        }
        async fn create_user_resource(
            &self,
            _: &UserResource,
        ) -> Result<UserResource, AppError> {
            unimplemented!()
        }
        async fn link_resource_to_course(
            &self,
            _: &str,
            _: &str,
        ) -> Result<(), AppError> {
            unimplemented!()
        }
        async fn delete_course(
            &self,
            _: &str,
        ) -> Result<(), AppError> {
            unimplemented!()
        }
        async fn delete_resource_links(
            &self,
            _: &str,
        ) -> Result<(), AppError> {
            unimplemented!()
        }
        async fn delete_resource(
            &self,
            _: &str,
        ) -> Result<(), AppError> {
            unimplemented!()
        }
    }

    struct PanicStudySessionRepo;
    #[async_trait]
    impl StudySessionRepository
        for PanicStudySessionRepo
    {
        async fn create(
            &self,
            _: &StudySession,
        ) -> Result<StudySession, AppError> {
            unimplemented!()
        }
        async fn list_by_course(
            &self,
            _: &str,
        ) -> Result<Vec<StudySession>, AppError> {
            unimplemented!()
        }
        async fn get(
            &self,
            _: &str,
        ) -> Result<StudySession, AppError> {
            unimplemented!()
        }
        async fn save_session_content(
            &self,
            _: &str,
            _: &serde_json::Value,
        ) -> Result<(), AppError> {
            unimplemented!()
        }
        async fn delete_by_course(
            &self,
            _: &str,
        ) -> Result<(), AppError> {
            unimplemented!()
        }
        async fn delete(
            &self,
            _: &str,
        ) -> Result<(), AppError> {
            unimplemented!()
        }
    }

    /// Build an StudyService with a given AI usage stub
    fn make_service(
        ai_repo: StubAiUsageRepo,
    ) -> StudyService {
        let repos = StudyRepositories {
            qcm_repo: Arc::new(PanicQcmRepo),
            ai_qcm_repo: Arc::new(PanicQcmRepo),
            open_question_repo: Arc::new(
                PanicOpenQuestionRepo,
            ),
            flashcard_repo: Arc::new(
                PanicFlashcardRepo,
            ),
            true_false_repo: Arc::new(
                PanicTrueOrFalseRepo,
            ),
            keywords_repo: Arc::new(PanicKeywordsRepo),
            order_phrase_repo: Arc::new(
                PanicOrderPhraseRepo,
            ),
            fill_blank_repo: Arc::new(
                PanicFillBlankRepo,
            ),
            course_repo: Arc::new(PanicCourseRepo),
            ai_usage_repo: Arc::new(ai_repo),
            study_session_repo: Arc::new(
                PanicStudySessionRepo,
            ),
        };
        let client = Arc::new(
            OpenRouterClient::new("test-key".into()),
        );
        let cache = Arc::new(OpenQuestionCache::new());
        StudyService::from_repositories(
            repos, client, cache,
        )
    }

    /// Build a test AiUsageInput
    fn test_input() -> AiUsageInput<'static> {
        AiUsageInput {
            user_id: "usr-1",
            model_id: "google/gemini-3-flash-preview",
            feature_type: "qcm",
            input_tokens: 100,
            output_tokens: 50,
        }
    }

    /// Build a test AiUsageLog
    fn test_log(
        user_id: &str,
        feature: &str,
        cost: f64,
    ) -> AiUsageLog {
        AiUsageLog {
            id: "log-1".to_string(),
            user_id: user_id.to_string(),
            model_id: "gpt-4".to_string(),
            feature_type: feature.to_string(),
            input_tokens: 100,
            output_tokens: 50,
            input_cost_usd: cost * 0.6,
            output_cost_usd: cost * 0.4,
            total_cost_usd: cost,
            created_at: "2024-01-01".to_string(),
        }
    }

    // -- log_ai_usage tests --

    #[tokio::test]
    async fn test_log_ai_usage_success() {
        // arrange
        let svc = make_service(StubAiUsageRepo::empty());
        let input = test_input();

        // act
        let log = svc.log_ai_usage(input).await.unwrap();

        // assert
        assert_eq!(log.user_id, "usr-1");
        assert_eq!(log.feature_type, "qcm");
    }

    #[tokio::test]
    async fn test_log_ai_usage_calculates_cost() {
        // arrange
        let svc = make_service(StubAiUsageRepo::empty());
        let input = test_input();

        // act
        let log = svc.log_ai_usage(input).await.unwrap();

        // assert - cost should be > 0 for known model
        assert!(log.total_cost_usd > 0.0);
    }

    // -- try_log_ai_usage tests --

    #[tokio::test]
    async fn test_try_log_ai_usage_does_not_panic() {
        // arrange
        let svc = make_service(StubAiUsageRepo::empty());
        let input = test_input();

        // act - should not panic or return error
        svc.try_log_ai_usage(input).await;
    }

    // -- get_all_usage tests --

    #[tokio::test]
    async fn test_get_all_usage_returns_all() {
        // arrange
        let logs = vec![
            test_log("usr-1", "qcm", 0.01),
            test_log("usr-2", "flashcard", 0.02),
        ];
        let svc =
            make_service(StubAiUsageRepo::new(logs));

        // act
        let result = svc.get_all_usage().await.unwrap();

        // assert
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_all_usage_empty() {
        // arrange
        let svc = make_service(StubAiUsageRepo::empty());

        // act
        let result = svc.get_all_usage().await.unwrap();

        // assert
        assert!(result.is_empty());
    }

    // -- get_aggregated_usage tests --

    #[tokio::test]
    async fn test_get_aggregated_usage_with_data() {
        // arrange
        let logs = vec![
            test_log("usr-1", "qcm", 0.01),
            test_log("usr-1", "flashcard", 0.02),
        ];
        let svc =
            make_service(StubAiUsageRepo::new(logs));

        // act
        let stats =
            svc.get_aggregated_usage().await.unwrap();

        // assert
        assert_eq!(stats.total_requests, 2);
    }

    #[tokio::test]
    async fn test_get_aggregated_usage_empty() {
        // arrange
        let svc = make_service(StubAiUsageRepo::empty());

        // act
        let stats =
            svc.get_aggregated_usage().await.unwrap();

        // assert
        assert_eq!(stats.total_requests, 0);
    }
}

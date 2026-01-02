//! Open question operations

use super::types_domain::{
    AnswerGrade, CheckAnswersInput, GenerateContentInput, GradingResult, IntelloService,
};
use crate::services::intello::ai_usage_domain::feature_type;
use crate::services::intello::open_question_domain::OpenQuestionSet;
use crate::services::intello::SetId;
use crate::services::intello::error_domain::IntelloError;
use crate::infra::openrouter::DEFAULT_MODEL;
use super::prompt_builder_service::{AnswerToGrade, OpenQuestionPromptInput, VerificationPromptInput};
use tracing::{info, instrument};

impl IntelloService {
    /// Get all open question sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_open_question_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        let sets = self.open_question_repo.find_by_user(user_id).await?;
        info!(count = sets.len(), "Retrieved user open question sets");
        Ok(sets)
    }

    /// Generate AI open questions and store them
    #[instrument(skip(self, input, source_content), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_open_questions(
        &self,
        user_id: &str,
        input: GenerateContentInput,
        source_content: String,
    ) -> Result<OpenQuestionSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            "Generating AI open questions"
        );

        // Build prompt input
        let prompt_input = OpenQuestionPromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_questions: input.num_questions,
            documents: input
                .documents
                .iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Build prompt and send request
        let prompt = super::prompt_builder_service::build_open_question_prompt(&prompt_input);
        let ai_result = self.openrouter_client.send_chat_request(&prompt, None).await?;

        // Parse the response
        let questions = crate::services::intello::ai_parsing_service::parse_open_question_response(&ai_result.content)?;

        // Log AI usage (fire-and-forget)
        if let Some(usage) = ai_result.usage {
            self.try_log_ai_usage(
                user_id,
                DEFAULT_MODEL,
                feature_type::OPEN_QUESTION,
                usage.prompt_tokens,
                usage.completion_tokens,
            )
            .await;
        }

        info!(generated = questions.len(), "AI open questions generated");

        // Build and store the set
        let set_id = SetId::new();
        let open_question_set = OpenQuestionSet {
            id: set_id.clone(),
            user_id: user_id.into(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        };

        let created = self.open_question_repo.insert(&open_question_set).await?;

        // Store source content in cache for later grading
        self.open_question_cache
            .store(user_id, &set_id, source_content);

        info!(set_id = %created.id, "AI open question set stored");
        Ok(created)
    }

    /// Check/grade user answers for open questions
    #[instrument(skip(self, input), fields(user_id = %user_id, set_id = %input.set_id))]
    pub async fn check_open_question_answers(
        &self,
        user_id: &str,
        input: CheckAnswersInput,
    ) -> Result<Vec<GradingResult>, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(&input.set_id)?;

        // Get the question set and verify ownership
        let set = self
            .open_question_repo
            .find_by_id(&input.set_id, user_id)
            .await?;
        let set =
            set.ok_or_else(|| IntelloError::game_not_found("open_question_set", &input.set_id))?;

        // Get source content from cache
        let source_content = self
            .open_question_cache
            .get(user_id, &input.set_id)
            .unwrap_or_else(|| "Source content not available.".to_string());

        // Build answers to grade
        let mut answers_to_grade: Vec<AnswerToGrade> = Vec::new();
        for user_answer in &input.answers {
            if let Some(question) = set
                .questions
                .iter()
                .find(|q| q.id.as_str() == user_answer.question_id)
            {
                answers_to_grade.push(AnswerToGrade {
                    question_id: question.id.to_string(),
                    question: question.question.clone(),
                    hint: question.hint.clone().unwrap_or_default(),
                    expected_answer: question.expected_answer.clone().unwrap_or_default(),
                    user_answer: user_answer.user_answer.clone(),
                });
            }
        }

        if answers_to_grade.is_empty() {
            return Err(IntelloError::validation(
                "answers",
                "No valid answers to grade",
            ));
        }

        info!(
            count = answers_to_grade.len(),
            "Grading open question answers"
        );

        // Build verification input and call AI
        let verification_input = VerificationPromptInput {
            language: set.language.clone(),
            source_content,
            answers: answers_to_grade,
        };

        // Build prompt and send request for verification
        let prompt = super::prompt_builder_service::build_verification_prompt(&verification_input);
        let ai_result = self.openrouter_client.send_chat_request(&prompt, None).await?;

        // Parse the verification response
        let grades = crate::services::intello::ai_parsing_service::parse_verification_response(&ai_result.content)?;

        // Convert to our result type
        let results: Vec<GradingResult> = grades
            .into_iter()
            .map(|g| GradingResult {
                question_id: g.question_id,
                grade: match g.grade {
                    crate::services::intello::ai_parsing_service::AnswerGrade::Right => AnswerGrade::Right,
                    crate::services::intello::ai_parsing_service::AnswerGrade::Medium => AnswerGrade::Medium,
                    crate::services::intello::ai_parsing_service::AnswerGrade::Error => AnswerGrade::Error,
                },
                feedback: g.feedback,
            })
            .collect();

        info!(graded = results.len(), "Open question answers graded");
        Ok(results)
    }
}

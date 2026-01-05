//! Open question operations


use tracing::{info, instrument};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::intello::ai_usage::ai_usage_domain::feature_type;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::types_domain::{
    AnswerGrade, CheckAnswersInput, GenerateContentInput, GradingResult, IntelloService,
};
use crate::services::intello::SetId;

use super::domain::OpenQuestionSet;
use super::parser;
use super::prompt::{build_open_question_prompt, OpenQuestionPromptInput};

// Verification submodule
use super::verification::prompt::{
    build_verification_prompt, AnswerToGrade, VerificationPromptInput,
};
use crate::services::intello::games::shared::types::AnswerGrade as SharedAnswerGrade;

impl IntelloService {
    // ** get_user_open_question_sets **
    // ==> Retrieves all open question sets for a user
    //
    // @ user_id : The user ID to query sets for
    // @ returns : Vector of all OpenQuestionSets owned by the user
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_open_question_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        // Step 1: Validate user ID
        self.validate_user_id(user_id)?;
        
        // Step 2: Query repository for all user sets
        let sets = self.open_question_repo.find_by_user(user_id).await?;
        
        // Step 3: Log retrieval and return sets
        info!(count = sets.len(), "Retrieved user open question sets");
        Ok(sets)
    }

    // ** generate_ai_open_questions **
    // ==> Generates open questions using AI and stores them with source content
    //
    // @ user_id : The user requesting generation
    // @ input : Generation parameters (name, level, documents, etc.)
    // @ source_content : Source content for later answer grading
    // @ returns : The created OpenQuestionSet with AI-generated questions
    // @ errors : ValidationFailed if inputs invalid, ExternalServiceError if AI fails
    #[instrument(skip(self, input, source_content), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_open_questions(
        &self,
        user_id: &str,
        input: GenerateContentInput,
        source_content: String,
    ) -> Result<OpenQuestionSet, IntelloError> {
        // Step 1: Validate user ID and generation input
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;
        info!(num_questions = input.num_questions, "Generating AI open questions");

        // Step 2: Build prompt input from generation parameters
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

        // Step 3: Build prompt and send request to AI service
        let prompt = build_open_question_prompt(&prompt_input);
        let ai_result = self.openrouter_client.send_chat_request(&prompt, None).await?;

        // Step 4: Parse AI response into open questions
        let questions = parser::parse_open_question_response(&ai_result.content)?;

        // Step 5: Log AI usage (fire-and-forget)
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

        // Step 6: Build and store the question set
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

        // Step 7: Cache source content for later grading
        self.open_question_cache.store(user_id, &set_id, source_content);
        info!(set_id = %created.id, "AI open question set stored");
        Ok(created)
    }

    // ** check_open_question_answers **
    // ==> Grades user answers for open questions using AI verification
    //
    // @ user_id : The user requesting answer checking
    // @ input : Contains set_id and user answers to grade
    // @ returns : Vector of GradingResult with feedback for each answer
    // @ errors : ValidationFailed, GameNotFound, ExternalServiceError
    #[instrument(skip(self, input), fields(user_id = %user_id, set_id = %input.set_id))]
    pub async fn check_open_question_answers(
        &self,
        user_id: &str,
        input: CheckAnswersInput,
    ) -> Result<Vec<GradingResult>, IntelloError> {
        // Step 1: Validate user ID and set ID
        self.validate_user_id(user_id)?;
        self.validate_set_id(&input.set_id)?;

        // Step 2: Retrieve question set and verify ownership
        let set = self
            .open_question_repo
            .find_by_id(&input.set_id, user_id)
            .await?;
        let set =
            set.ok_or_else(|| IntelloError::game_not_found("open_question_set", &input.set_id))?;

        // Step 3: Retrieve source content from cache for grading context
        let source_content = self
            .open_question_cache
            .get(user_id, &input.set_id)
            .unwrap_or_else(|| "Source content not available.".to_string());

        // Step 4: Build answers to grade by matching user answers with questions
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

        // Step 5: Validate that we have answers to grade
        if answers_to_grade.is_empty() {
            return Err(IntelloError::validation(
                "answers",
                "No valid answers to grade",
            ));
        }
        info!(count = answers_to_grade.len(), "Grading open question answers");

        // Step 6: Build verification prompt with source content and answers
        let verification_input = VerificationPromptInput {
            language: set.language.clone(),
            source_content,
            answers: answers_to_grade,
        };
        let prompt = build_verification_prompt(&verification_input);

        // Step 7: Send verification request to AI service
        let ai_result = self.openrouter_client.send_chat_request(&prompt, None).await?;

        // Step 8: Parse AI verification response into grades
        let grades = super::verification::parser::parse_verification_response(&ai_result.content)?;

        // Step 9: Convert to GradingResult format
        let results: Vec<GradingResult> = grades
            .into_iter()
            .map(|g| GradingResult {
                question_id: g.question_id,
                grade: match g.grade {
                    SharedAnswerGrade::Right => AnswerGrade::Right,
                    SharedAnswerGrade::Medium => AnswerGrade::Medium,
                    SharedAnswerGrade::Error => AnswerGrade::Error,
                },
                feedback: g.feedback,
            })
            .collect();

        // Step 10: Log results and return grading feedback
        info!(graded = results.len(), "Open question answers graded");
        Ok(results)
    }
}

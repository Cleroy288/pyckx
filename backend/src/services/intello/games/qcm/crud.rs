//! QCM CRUD operations

use tracing::{debug, info, instrument, warn};

use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::types_domain::IntelloService;

use super::domain::QcmSet;

impl IntelloService {
    /// Creates a new manual QCM set and stores it
    #[instrument(
        skip(self, qcm_set),
        fields(
            user_id = %qcm_set.user_id,
            set_id = %qcm_set.id,
        )
    )]
    pub async fn create_qcm_set(
        &self,
        qcm_set: QcmSet,
    ) -> Result<QcmSet, IntelloError> {
        // Validate QCM set structure and content
        self.validate_qcm_set(&qcm_set)?;

        // Insert into repository
        let created =
            self.qcm_repo.insert(&qcm_set).await?;

        info!(set_id = %created.id, "QCM set created");
        Ok(created)
    }

    /// Retrieves all QCM sets for a user
    #[instrument(
        skip(self),
        fields(user_id = %user_id)
    )]
    pub async fn get_user_qcm_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<QcmSet>, IntelloError> {
        self.validate_user_id(user_id)?;

        let all_sets =
            self.qcm_repo.find_by_user(user_id).await?;

        info!(
            count = all_sets.len(),
            "Retrieved user QCM sets"
        );
        Ok(all_sets)
    }

    /// Retrieves a specific QCM set by ID
    #[instrument(
        skip(self),
        fields(
            user_id = %user_id,
            set_id = %set_id,
        )
    )]
    pub async fn get_qcm_set(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<QcmSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        let set = self
            .qcm_repo
            .find_by_id(set_id, user_id)
            .await?;

        if set.is_some() {
            debug!(set_id = %set_id, "QCM set found");
        }
        Ok(set)
    }

    /// Updates an existing QCM set
    #[instrument(
        skip(self, qcm_set),
        fields(
            user_id = %qcm_set.user_id,
            set_id = %qcm_set.id,
        )
    )]
    pub async fn update_qcm_set(
        &self,
        qcm_set: QcmSet,
    ) -> Result<bool, IntelloError> {
        self.validate_qcm_set(&qcm_set)?;

        // Verify user owns the set
        let existing = self
            .qcm_repo
            .find_by_id(&qcm_set.id, &qcm_set.user_id)
            .await?;
        if existing.is_none() {
            warn!(
                set_id = %qcm_set.id,
                "QCM set not found or not owned"
            );
            return Ok(false);
        }

        let updated =
            self.qcm_repo.update(&qcm_set).await?;

        if updated {
            info!(
                set_id = %qcm_set.id,
                "QCM set updated"
            );
        }
        Ok(updated)
    }

    /// Deletes a QCM set with ownership check
    #[instrument(
        skip(self),
        fields(
            user_id = %user_id,
            set_id = %set_id,
        )
    )]
    pub async fn delete_qcm_set(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<bool, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        // Verify user owns the set
        let existing = self
            .qcm_repo
            .find_by_id(set_id, user_id)
            .await?;
        if existing.is_none() {
            warn!(
                set_id = %set_id,
                "QCM set not found or not owned"
            );
            return Ok(false);
        }

        let deleted = self
            .qcm_repo
            .delete(set_id, user_id)
            .await?;

        if deleted {
            info!(
                set_id = %set_id,
                "QCM set deleted"
            );
        }
        Ok(deleted)
    }
}

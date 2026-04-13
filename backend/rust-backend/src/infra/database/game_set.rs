//! Game Set Repository Trait - Generic abstraction for game set persistence
//!
//! This trait provides a unified interface for all Study game set repositories.
//! Each game type (QCM, Flashcard, Keywords, etc.) implements this trait.

use crate::services::domain::error_domain::StudyError;
use async_trait::async_trait;

/// Generic repository trait for game set persistence operations.
///
/// This trait defines the common CRUD operations for all game set types.
/// Type `T` must be Clone (for insert return) and Send + Sync for async safety.
///
/// # Examples
///
/// ```ignore
/// use crate::services::domain::QcmSet;
/// use crate::infra::database::GameSetRepository;
///
/// struct JsonQcmRepository { /* ... */ }
///
/// #[async_trait]
/// impl GameSetRepository<QcmSet> for JsonQcmRepository {
///     async fn insert(&self, set: &QcmSet) -> Result<QcmSet, StudyError> { /* ... */ }
///     // ...
/// }
/// ```
#[async_trait]
pub trait GameSetRepository<T: Clone + Send + Sync>: Send + Sync {
    /// Insert a new game set into the repository.
    async fn insert(&self, set: &T) -> Result<T, StudyError>;

    /// Find all game sets for a specific user.
    async fn find_by_user(&self, user_id: &str)
        -> Result<Vec<T>, StudyError>;
}

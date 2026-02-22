//! Session management - Server-side session storage with Supabase persistence
//!
//! Sessions are stored in-memory for fast lookup but persisted to Supabase
//! for durability across server restarts.

use crate::infra::supabase::session::SupabaseSessionRepository;
use crate::infra::supabase::shared::SupabaseError;
use crate::infra::user::User;
use crate::infra::user::UserId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{info, warn};
use uuid::Uuid;

/// Type alias for session IDs
pub type SessionId = String;

/// Session - Links a session ID to a user with their tokens
#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub user: User,
}

impl Session {
    /// Create a new session for a user
    pub fn new(user: User) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user,
        }
    }
}

/// Thread-safe map of session IDs to sessions
type SessionMap = Arc<RwLock<HashMap<SessionId, Session>>>;
/// Thread-safe map of user IDs to their session IDs
type UserSessionMap = Arc<RwLock<HashMap<UserId, SessionId>>>;

/// SessionStore - In-memory + Supabase persistence
/// Keyed by session_id for fast lookup from cookie
#[derive(Clone)]
pub struct SessionStore {
    // session_id -> Session
    by_session: SessionMap,
    // user_id -> session_id (for lookup by Supabase ID)
    user_to_session: UserSessionMap,
    // Supabase repository for persistence
    repository: Arc<SupabaseSessionRepository>,
}

impl std::fmt::Debug for SessionStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionStore")
            .field(
                "session_count",
                &self.by_session.read().map(|s| s.len()).unwrap_or(0),
            )
            .finish()
    }
}

impl SessionStore {
    /// Create new store with Supabase repository
    pub fn new(repository: Arc<SupabaseSessionRepository>) -> Self {
        Self {
            by_session: Arc::new(RwLock::new(HashMap::new())),
            user_to_session: Arc::new(RwLock::new(HashMap::new())),
            repository,
        }
    }

    /// Initialize by loading existing sessions from Supabase
    /// Call this on server startup
    pub async fn initialize(&self) -> Result<(), SupabaseError> {
        info!("Loading sessions from Supabase...");

        match self.repository.get_all().await {
            Ok(rows) => {
                let mut sessions = self.by_session.write().unwrap();
                let mut user_map = self.user_to_session.write().unwrap();

                for row in rows {
                    let user = User {
                        id: row.user_id.into(),
                        email: row.email,
                        username: row.username,
                        role: row.role,
                        access_token: row.access_token,
                        refresh_token: row.refresh_token,
                        expires_at: row.expires_at as u64,
                    };

                    let session = Session {
                        id: row.session_id.clone(),
                        user,
                    };

                    user_map
                        .insert(session.user.id.clone(), session.id.clone());
                    sessions.insert(session.id.clone(), session);
                }

                info!(count = sessions.len(), "Sessions loaded from Supabase");
                Ok(())
            }
            Err(err) => {
                warn!(error = %err, "Failed to load sessions from Supabase, starting fresh");
                // Don't fail startup if we can't load sessions
                Ok(())
            }
        }
    }

    /// Find an existing valid (non-expired) session for a user
    fn find_valid_session(&self, user_id: &UserId) -> Option<SessionId> {
        let sessions = self.by_session.read().unwrap();
        let user_map = self.user_to_session.read().unwrap();

        let session_id = user_map.get(user_id)?;
        let session = sessions.get(session_id)?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if session.user.expires_at > now {
            Some(session_id.clone())
        } else {
            None
        }
    }

    /// Spawn a background task to delete a session from Supabase
    fn spawn_delete_session(&self, session_id: String) {
        let repo = Arc::clone(&self.repository);
        tokio::spawn(async move {
            if let Err(err) = repo.delete(&session_id).await {
                warn!(
                    error = %err,
                    "Failed to delete session from Supabase"
                );
            }
        });
    }

    /// Spawn a background task to persist a session to Supabase
    fn spawn_persist_session(&self, session: Session) {
        use crate::infra::supabase::session::SessionRow;

        let repo = Arc::clone(&self.repository);
        let row = SessionRow {
            session_id: session.id,
            user_id: session.user.id.to_string(),
            email: session.user.email,
            username: session.user.username,
            role: session.user.role,
            access_token: session.user.access_token,
            refresh_token: session.user.refresh_token,
            expires_at: session.user.expires_at as i64,
        };

        tokio::spawn(async move {
            if let Err(err) = repo.upsert(row).await {
                warn!(
                    error = %err,
                    "Failed to persist session to Supabase"
                );
            }
        });
    }

    /// Get or create session for user
    /// If user already has a valid session, returns existing session_id
    /// Otherwise creates a new session
    pub fn create_session(&self, user: User) -> SessionId {
        // Check if user already has a valid session
        if let Some(sid) = self.find_valid_session(&user.id) {
            info!(
                session_id = %sid,
                user_id = %user.id,
                "Reusing existing valid session"
            );
            return sid;
        }

        // No valid session exists, create new one
        let session = Session::new(user);
        let session_id = session.id.clone();
        let session_for_persist = session.clone();

        {
            let mut sessions = self.by_session.write().unwrap();
            let mut user_map = self.user_to_session.write().unwrap();

            // Remove old expired session if exists
            if let Some(old_id) = user_map.get(&session.user.id) {
                let old_id = old_id.clone();
                sessions.remove(&old_id);
                self.spawn_delete_session(old_id);
            }

            user_map.insert(session.user.id.clone(), session_id.clone());
            sessions.insert(session_id.clone(), session);
        }

        // Persist new session to Supabase in background
        self.spawn_persist_session(session_for_persist);

        info!(session_id = %session_id, "New session created");
        session_id
    }

    /// Get user by session_id (from cookie)
    pub fn get_user(&self, session_id: &str) -> Option<User> {
        let sessions = self.by_session.read().unwrap();
        sessions.get(session_id).map(|s| s.user.clone())
    }

    /// Remove session (logout)
    pub fn delete_session(&self, session_id: &str) -> Option<User> {
        let user = {
            let mut sessions = self.by_session.write().unwrap();
            let mut user_map = self.user_to_session.write().unwrap();

            if let Some(session) = sessions.remove(session_id) {
                user_map.remove(&session.user.id);
                Some(session.user)
            } else {
                None
            }
        };

        if user.is_some() {
            self.spawn_delete_session(session_id.to_string());
            info!(session_id = %session_id, "Session deleted");
        }
        user
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a test user with known fields
    fn test_user() -> User {
        User {
            id: UserId::from_string("user-42".to_string()),
            email: "alice@test.com".to_string(),
            username: "alice".to_string(),
            role: "student".to_string(),
            access_token: "tok-access".to_string(),
            refresh_token: "tok-refresh".to_string(),
            expires_at: 9_999_999_999,
        }
    }

    #[test]
    fn test_session_new_generates_uuid_id() {
        // arrange
        let user = test_user();

        // act
        let session = Session::new(user);

        // assert - UUID v4 format: 36 chars
        assert_eq!(session.id.len(), 36);
    }

    #[test]
    fn test_session_new_preserves_user() {
        // arrange
        let user = test_user();

        // act
        let session = Session::new(user.clone());

        // assert
        assert_eq!(session.user.email, "alice@test.com");
        assert_eq!(session.user.username, "alice");
    }

    #[test]
    fn test_session_new_generates_unique_ids() {
        // arrange
        let user1 = test_user();
        let user2 = test_user();

        // act
        let session1 = Session::new(user1);
        let session2 = Session::new(user2);

        // assert
        assert_ne!(session1.id, session2.id);
    }
}

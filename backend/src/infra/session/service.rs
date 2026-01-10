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

/// SessionStore - In-memory + Supabase persistence
/// Keyed by session_id for fast lookup from cookie
#[derive(Clone)]
pub struct SessionStore {
    // session_id -> Session
    by_session: Arc<RwLock<HashMap<SessionId, Session>>>,
    // user_id -> session_id (for lookup by Supabase ID)
    user_to_session: Arc<RwLock<HashMap<UserId, SessionId>>>,
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

                    user_map.insert(session.user.id.clone(), session.id.clone());
                    sessions.insert(session.id.clone(), session);
                }

                info!(count = sessions.len(), "Sessions loaded from Supabase");
                Ok(())
            }
            Err(e) => {
                warn!(error = %e, "Failed to load sessions from Supabase, starting fresh");
                // Don't fail startup if we can't load sessions
                Ok(())
            }
        }
    }

    /// Get or create session for user
    /// If user already has a valid session, returns existing session_id
    /// Otherwise creates a new session
    pub fn create_session(&self, user: User) -> SessionId {
        // Check if user already has a valid session
        {
            let sessions = self.by_session.read().unwrap();
            let user_map = self.user_to_session.read().unwrap();

            if let Some(existing_session_id) = user_map.get(&user.id) {
                if let Some(existing_session) = sessions.get(existing_session_id) {
                    // Check if session is still valid (not expired)
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if existing_session.user.expires_at > now {
                        info!(session_id = %existing_session_id, user_id = %user.id, "Reusing existing valid session");
                        return existing_session_id.clone();
                    }
                }
            }
        }

        // No valid session exists, create new one
        let session = Session::new(user);
        let session_id = session.id.clone();
        let session_for_persist = session.clone();

        {
            let mut sessions = self.by_session.write().unwrap();
            let mut user_map = self.user_to_session.write().unwrap();

            // Remove old expired session if exists
            if let Some(old_session_id) = user_map.get(&session.user.id) {
                let old_id = old_session_id.clone();
                sessions.remove(&old_id);

                // Delete old session from Supabase in background
                let repo: Arc<SupabaseSessionRepository> = Arc::clone(&self.repository);
                tokio::spawn(async move {
                    if let Err(e) = repo.delete(&old_id).await {
                        warn!(error = %e, "Failed to delete old session from Supabase");
                    }
                });
            }

            user_map.insert(session.user.id.clone(), session_id.clone());
            sessions.insert(session_id.clone(), session);
        }

        // Persist new session to Supabase in background
        let repo: Arc<SupabaseSessionRepository> = Arc::clone(&self.repository);
        tokio::spawn(async move {
            use crate::infra::supabase::session::SessionRow;

            let row = SessionRow {
                session_id: session_for_persist.id,
                user_id: session_for_persist.user.id.to_string(),
                email: session_for_persist.user.email,
                username: session_for_persist.user.username,
                role: session_for_persist.user.role,
                access_token: session_for_persist.user.access_token,
                refresh_token: session_for_persist.user.refresh_token,
                expires_at: session_for_persist.user.expires_at as i64,
            };

            if let Err(e) = repo.upsert(row).await {
                warn!(error = %e, "Failed to persist session to Supabase");
            }
        });

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
            // Delete from Supabase in background
            let repo: Arc<SupabaseSessionRepository> = Arc::clone(&self.repository);
            let sid = session_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = repo.delete(&sid).await {
                    warn!(error = %e, "Failed to delete session from Supabase");
                }
            });
            info!(session_id = %session_id, "Session deleted");
        }
        user
    }
}

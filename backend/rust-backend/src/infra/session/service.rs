//! Session management - Redis-backed session storage
//!
//! Sessions are stored in Redis as JSON. Both the Rust
//! backend and the auth microservice (Hono) read/write
//! the same keys.

use crate::infra::user::User;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use tracing::{info, warn};
use uuid::Uuid;

/// Type alias for session IDs
pub type SessionId = String;

/// Redis key prefix for sessions
const SESSION_PREFIX: &str = "session:";
/// Redis key prefix for user->session index
const USER_SESSION_PREFIX: &str = "user_session:";
/// Fallback TTL when token is already expired
const FALLBACK_TTL_SECS: u64 = 3600;

/// SessionStore - Redis-backed session storage
#[derive(Clone)]
pub struct SessionStore {
    redis: ConnectionManager,
}

impl std::fmt::Debug for SessionStore {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("SessionStore")
            .field("backend", &"redis")
            .finish()
    }
}

impl SessionStore {
    /// Create new store with Redis connection
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Get user by session_id (from cookie)
    pub async fn get_user(
        &self,
        session_id: &str,
    ) -> Option<User> {
        let key =
            format!("{SESSION_PREFIX}{session_id}");
        let mut conn = self.redis.clone();

        let json: Option<String> =
            match conn.get(&key).await {
                Ok(val) => val,
                Err(err) => {
                    warn!(
                        error = %err,
                        "Redis GET failed for session"
                    );
                    return None;
                }
            };

        let raw = json?;
        match serde_json::from_str(raw.as_str()) {
            Ok(user) => Some(user),
            Err(err) => {
                warn!(
                    error = %err,
                    "Failed to deserialize session"
                );
                None
            }
        }
    }

    /// Create session -- always overwrites (fresh token)
    pub async fn create_session(
        &self,
        user: User,
    ) -> SessionId {
        let session_id = Uuid::new_v4().to_string();
        let ttl = compute_ttl(&user);

        if let Err(err) = self
            .store_session(&session_id, &user, ttl)
            .await
        {
            warn!(
                error = %err,
                "Failed to store session"
            );
        }

        info!(
            session_id = %session_id,
            "Session created"
        );
        session_id
    }

    /// Remove session (logout)
    pub async fn delete_session(
        &self,
        session_id: &str,
    ) -> Option<User> {
        let user = self.get_user(session_id).await?;
        let mut conn = self.redis.clone();

        let session_key =
            format!("{SESSION_PREFIX}{session_id}");
        let user_key = format!(
            "{USER_SESSION_PREFIX}{}",
            user.id
        );

        let result: Result<(), _> = redis::pipe()
            .del(&session_key)
            .del(&user_key)
            .query_async(&mut conn)
            .await;

        if let Err(err) = result {
            warn!(
                error = %err,
                "Redis DEL failed during logout"
            );
        }

        info!(
            session_id = %session_id,
            "Session deleted"
        );
        Some(user)
    }

    /// Store session + user->session index in Redis
    async fn store_session(
        &self,
        session_id: &str,
        user: &User,
        ttl_secs: u64,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        let session_key =
            format!("{SESSION_PREFIX}{session_id}");
        let user_key = format!(
            "{USER_SESSION_PREFIX}{}",
            user.id
        );

        let json = serde_json::to_string(user)
            .map_err(|e| {
                redis::RedisError::from(
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        e,
                    ),
                )
            })?;

        redis::pipe()
            .cmd("SET")
            .arg(&session_key)
            .arg(&json)
            .arg("EX")
            .arg(ttl_secs)
            .cmd("SET")
            .arg(&user_key)
            .arg(session_id)
            .arg("EX")
            .arg(ttl_secs)
            .query_async(&mut conn)
            .await
    }
}

/// Compute TTL from user token expiry
fn compute_ttl(user: &User) -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_secs();

    if user.expires_at > now {
        return user.expires_at - now;
    }

    warn!(
        expires_at = user.expires_at,
        now = now,
        "Token already expired, using fallback"
    );
    FALLBACK_TTL_SECS
}

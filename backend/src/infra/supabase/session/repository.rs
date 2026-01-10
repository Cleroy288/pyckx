//! Supabase session repository - CRUD operations for sessions table
//!
//! Handles persistence of user sessions to Supabase.
//! Used to load sessions on server startup and persist changes.

use crate::infra::supabase::shared::SupabaseError;
use crate::infra::supabase::SupabaseHttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, warn};

const TABLE: &str = "sessions";

/// Session row from Supabase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRow {
    pub session_id: String,
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

/// Insert body for creating a session
#[derive(Debug, Clone, Serialize)]
struct InsertSession {
    session_id: String,
    user_id: String,
    email: String,
    username: String,
    role: String,
    access_token: String,
    refresh_token: String,
    expires_at: i64,
}

/// Supabase session repository
#[derive(Clone, Debug)]
pub struct SupabaseSessionRepository {
    client: Arc<SupabaseHttpClient>,
}

impl SupabaseSessionRepository {
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        Self { client }
    }

    /// Load all sessions from Supabase
    pub async fn get_all(&self) -> Result<Vec<SessionRow>, SupabaseError> {
        let url = self.client.rest_url_with_query(TABLE, "select=*");
        debug!(url = %url, "Loading all sessions from Supabase");

        match self.client.get::<Vec<SessionRow>>(&url).await {
            Ok(sessions) => {
                info!(count = sessions.len(), "Loaded sessions from Supabase");
                Ok(sessions)
            }
            Err(e) => {
                warn!(error = %e, "Failed to load sessions from Supabase");
                Err(e)
            }
        }
    }

    /// Insert or update a session (upsert on session_id)
    pub async fn upsert(&self, session: SessionRow) -> Result<(), SupabaseError> {
        let url = self
            .client
            .rest_url_with_query(TABLE, "on_conflict=session_id");
        debug!(session_id = %session.session_id, "Upserting session to Supabase");

        let body = InsertSession {
            session_id: session.session_id.clone(),
            user_id: session.user_id,
            email: session.email,
            username: session.username,
            role: session.role,
            access_token: session.access_token,
            refresh_token: session.refresh_token,
            expires_at: session.expires_at,
        };

        let _: Vec<SessionRow> = self.client.post(&url, &body).await?;
        info!(session_id = %session.session_id, "Session upserted to Supabase");
        Ok(())
    }

    /// Delete a session by session_id
    pub async fn delete(&self, session_id: &str) -> Result<(), SupabaseError> {
        let url = self
            .client
            .rest_url_with_query(TABLE, &format!("session_id=eq.{}", session_id));
        debug!(session_id = %session_id, "Deleting session from Supabase");

        self.client.delete(&url).await?;
        info!(session_id = %session_id, "Session deleted from Supabase");
        Ok(())
    }

    /// Delete all sessions for a user (cleanup old sessions)
    pub async fn delete_by_user_id(&self, user_id: &str) -> Result<(), SupabaseError> {
        let url = self
            .client
            .rest_url_with_query(TABLE, &format!("user_id=eq.{}", user_id));
        debug!(user_id = %user_id, "Deleting user sessions from Supabase");

        self.client.delete(&url).await?;
        Ok(())
    }
}

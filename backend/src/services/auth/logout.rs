//! Logout and session operations

use super::AuthService;
use crate::domain::SessionStore;
use tracing::{info, instrument};

impl AuthService {
    /// Logout user - invalidates session locally and notifies Supabase
    #[instrument(skip(self, session_id))]
    pub async fn logout(&self, session_id: &str) -> bool {
        // Remove from local store and get user data (contains access_token)
        let user = self.sessions.delete_session(session_id);

        if let Some(user) = user {
            // Notify Supabase to invalidate the token (best-effort)
            self.supabase.logout(&user.access_token).await;
            info!(session_id = %session_id, "User logged out");
            true
        } else {
            info!(session_id = %session_id, "Logout attempted but session not found");
            false
        }
    }

    /// Get session store reference (for user lookups)
    pub fn sessions(&self) -> &SessionStore {
        &self.sessions
    }
}

//! Session management - Server-side session storage with CSV persistence

use crate::domain::user::{User, UserId};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Arc, RwLock};
use tracing::{info, warn};
use uuid::Uuid;

const SESSION_FILE: &str = "data/sessions.csv";

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

/// SessionStore - In-memory + CSV persistence
/// Keyed by session_id for fast lookup from cookie
#[derive(Debug, Clone)]
pub struct SessionStore {
    // session_id -> Session
    by_session: Arc<RwLock<HashMap<SessionId, Session>>>,
    // user_id -> session_id (for lookup by Supabase ID)
    user_to_session: Arc<RwLock<HashMap<UserId, SessionId>>>,
}

impl SessionStore {
    /// Create new store and load existing sessions from CSV
    pub fn new() -> Self {
        let store = Self {
            by_session: Arc::new(RwLock::new(HashMap::new())),
            user_to_session: Arc::new(RwLock::new(HashMap::new())),
        };
        store.load_from_csv();
        store
    }


    /// Insert user with new session, persist to CSV
    pub fn create_session(&self, user: User) -> SessionId {
        let session = Session::new(user);
        let session_id = session.id.clone();
        let user_id = session.user.id.clone();

        {
            let mut sessions = self.by_session.write().unwrap();
            let mut user_map = self.user_to_session.write().unwrap();

            // Remove old session if user already logged in
            if let Some(old_session) = user_map.get(&user_id) {
                sessions.remove(old_session);
            }

            user_map.insert(user_id, session_id.clone());
            sessions.insert(session_id.clone(), session);
        }

        self.save_to_csv();
        info!(session_id = %session_id, "Session created");
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
            self.save_to_csv();
            info!(session_id = %session_id, "Session deleted");
        }
        user
    }

    /// Load sessions from CSV file
    fn load_from_csv(&self) {
        let path = Path::new(SESSION_FILE);
        if !path.exists() {
            info!("No session file found, starting fresh");
            return;
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                warn!(error = %e, "Failed to open session file");
                return;
            }
        };

        let reader = BufReader::new(file);
        let mut sessions = self.by_session.write().unwrap();
        let mut user_map = self.user_to_session.write().unwrap();
        let mut count = 0;

        for (i, line) in reader.lines().enumerate() {
            if i == 0 {
                continue; // Skip header
            }

            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 8 {
                continue;
            }

            let user = User {
                id: parts[1].to_string(),
                email: parts[2].to_string(),
                username: parts[3].to_string(),
                role: parts[4].to_string(),
                access_token: parts[5].to_string(),
                refresh_token: parts[6].to_string(),
                expires_at: parts[7].parse().unwrap_or(0),
            };

            let session = Session {
                id: parts[0].to_string(),
                user,
            };

            user_map.insert(session.user.id.clone(), session.id.clone());
            sessions.insert(session.id.clone(), session);
            count += 1;
        }

        info!(count = count, "Loaded sessions from CSV");
    }

    /// Save all sessions to CSV file
    fn save_to_csv(&self) {
        if let Err(e) = std::fs::create_dir_all("data") {
            warn!(error = %e, "Failed to create data directory");
            return;
        }

        let file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(SESSION_FILE)
        {
            Ok(f) => f,
            Err(e) => {
                warn!(error = %e, "Failed to open session file for writing");
                return;
            }
        };

        let mut writer = std::io::BufWriter::new(file);
        let sessions = self.by_session.read().unwrap();

        // Write header
        let _ = writeln!(
            writer,
            "session_id,user_id,email,username,role,access_token,refresh_token,expires_at"
        );

        // Write each session
        for session in sessions.values() {
            let _ = writeln!(
                writer,
                "{},{},{},{},{},{},{},{}",
                session.id,
                session.user.id,
                session.user.email,
                session.user.username,
                session.user.role,
                session.user.access_token,
                session.user.refresh_token,
                session.user.expires_at
            );
        }

        let _ = writer.flush();
        info!(count = sessions.len(), "Saved sessions to CSV");
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

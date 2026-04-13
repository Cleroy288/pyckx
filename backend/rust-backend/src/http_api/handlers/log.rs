//! Frontend log ingestion endpoint

use actix_web::{web, HttpResponse};
use serde::Deserialize;

/// Max log entries per request (prevent abuse)
const MAX_ENTRIES: usize = 20;
/// Max message length per entry
const MAX_MSG_LEN: usize = 2000;

/// A single log entry from the frontend
#[derive(Debug, Deserialize)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
    pub context: Option<String>,
}

/// Batch of log entries from the frontend
#[derive(Debug, Deserialize)]
pub struct LogBatch {
    pub entries: Vec<LogEntry>,
}

/// POST /api/log — receive frontend logs
pub async fn receive_logs(
    body: web::Json<LogBatch>,
) -> HttpResponse {
    let entries = &body.entries;
    let count = entries.len().min(MAX_ENTRIES);

    for entry in entries.iter().take(count) {
        let msg = truncate(&entry.message, MAX_MSG_LEN);
        let ctx = entry
            .context
            .as_deref()
            .unwrap_or("unknown");
        emit_log(&entry.level, msg, ctx);
    }

    HttpResponse::NoContent().finish()
}

/// Emit a tracing event at the correct level
fn emit_log(level: &str, msg: &str, ctx: &str) {
    match level {
        "error" => tracing::error!(
            target: "frontend",
            context = ctx,
            "{msg}"
        ),
        "warn" => tracing::warn!(
            target: "frontend",
            context = ctx,
            "{msg}"
        ),
        _ => tracing::info!(
            target: "frontend",
            context = ctx,
            "{msg}"
        ),
    }
}

/// Truncate a string to max bytes on char boundary
fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    match s.get(..max) {
        Some(slice) => slice,
        None => &s[..s.floor_char_boundary(max)],
    }
}

/// Register routes
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/log")
            .route(web::post().to(receive_logs)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_long_string() {
        let long = "a".repeat(3000);
        assert_eq!(truncate(&long, 2000).len(), 2000);
    }

    #[test]
    fn test_truncate_unicode_boundary() {
        // 3-byte UTF-8 char: won't panic on boundary
        let s = "aaé";
        let result = truncate(s, 3);
        assert!(result.len() <= 3);
    }
}

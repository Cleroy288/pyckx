//! Frontend log forwarder — sends logs to backend
//! and mirrors to browser console.

use gloo_net::http::Request;
use serde::Serialize;

/// A single log entry
#[derive(Serialize)]
struct LogEntry {
    level: &'static str,
    message: String,
    context: Option<String>,
}

/// Batch wrapper
#[derive(Serialize)]
struct LogBatch {
    entries: Vec<LogEntry>,
}

/// Log an error with context, forward to backend
pub fn log_error(msg: &str, context: &str) {
    console_error(msg);
    send_log("error", msg, context);
}

/// Log a warning with context, forward to backend
pub fn log_warn(msg: &str, context: &str) {
    console_warn(msg);
    send_log("warn", msg, context);
}

/// Fire-and-forget POST to /api/log
fn send_log(
    level: &'static str,
    msg: &str,
    context: &str,
) {
    let batch = LogBatch {
        entries: vec![LogEntry {
            level,
            message: msg.to_string(),
            context: Some(context.to_string()),
        }],
    };
    let Ok(body) = serde_json::to_string(&batch)
    else {
        return;
    };
    // Fire-and-forget — don't await, don't block
    let _ = Request::post("/api/log")
        .header("Content-Type", "application/json")
        .body(body)
        .map(|req| {
            wasm_bindgen_futures::spawn_local(
                async move {
                    let _ = req.send().await;
                },
            );
        });
}

/// Browser console.error
fn console_error(msg: &str) {
    web_sys::console::error_1(
        &wasm_bindgen::JsValue::from_str(msg),
    );
}

/// Browser console.warn
fn console_warn(msg: &str) {
    web_sys::console::warn_1(
        &wasm_bindgen::JsValue::from_str(msg),
    );
}

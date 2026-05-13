use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Shared atomic counters for request and upstream-call tracking.
///
/// Stored in [`AppState`] and incremented in the service layer.
/// Exposed via `GET /status` and the `status` MCP action.
#[derive(Debug, Default)]
pub struct Counters {
    /// Total tool calls received (all actions).
    pub requests_total: AtomicU64,
    /// Total tool calls that returned an error.
    pub errors_total: AtomicU64,
    /// Total HTTP requests sent to the UniFi controller.
    pub upstream_calls: AtomicU64,
    /// Total upstream HTTP requests that failed.
    pub upstream_errors: AtomicU64,
}

impl Counters {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Increment `requests_total` by 1.
    pub fn request(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment `errors_total` by 1.
    pub fn error(&self) {
        self.errors_total.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment `upstream_calls` by 1.
    pub fn upstream_call(&self) {
        self.upstream_calls.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment `upstream_errors` by 1.
    pub fn upstream_error(&self) {
        self.upstream_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Snapshot all counters as a JSON-serializable struct.
    pub fn snapshot(&self) -> CounterSnapshot {
        CounterSnapshot {
            requests_total: self.requests_total.load(Ordering::Relaxed),
            errors_total: self.errors_total.load(Ordering::Relaxed),
            upstream_calls: self.upstream_calls.load(Ordering::Relaxed),
            upstream_errors: self.upstream_errors.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CounterSnapshot {
    pub requests_total: u64,
    pub errors_total: u64,
    pub upstream_calls: u64,
    pub upstream_errors: u64,
}

/// Server start time, stored as a process-global for uptime calculation.
static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

/// Record the server start time. Call once at startup.
pub fn record_start_time() {
    START_TIME.get_or_init(Instant::now);
}

/// Return elapsed seconds since [`record_start_time`] was called.
/// Returns 0 if never called.
pub fn uptime_secs() -> u64 {
    START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0)
}

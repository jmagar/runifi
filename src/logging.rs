//! Dual-output logging: pretty aurora console (stderr) + structured JSON file.
//!
//! File: `{data_dir}/logs/unifi.log`, 10 MB cap (truncated on startup when over limit).
//! Console: colored, human-readable, aurora palette, stderr only.

pub mod aurora;

use std::io::IsTerminal;
use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Initialize tracing with dual outputs.
///
/// **Must** be called before any `tracing` macro. The returned [`WorkerGuard`]
/// must be kept alive for the duration of `main()` — dropping it early silently
/// discards buffered file writes.
///
/// # Log file
/// Located at `{data_dir}/logs/unifi.log`. Truncated to zero on startup when
/// the file already exceeds 10 MB (§42: exactly one file, never more).
///
/// # Environment
/// - `RUST_LOG` — log filter directive (overrides the default `"info"`)
/// - `NO_COLOR`  — disable ANSI colors on console
/// - `FORCE_COLOR` — force ANSI colors even when stderr is not a TTY (useful
///   inside Docker when piped to `docker compose logs`)
pub fn init_logging(data_dir: &Path, service_name: &str) -> anyhow::Result<WorkerGuard> {
    // ── log file ──────────────────────────────────────────────────────────────

    let log_dir = data_dir.join("logs");
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| anyhow::anyhow!("failed to create log dir {}: {e}", log_dir.display()))?;

    let log_path = log_dir.join(format!("{service_name}.log"));

    // Truncate on startup if over 10 MB (§42: 1 file, 10 MB cap).
    const LOG_CAP_BYTES: u64 = 10 * 1024 * 1024;
    if log_path.exists() {
        if let Ok(meta) = log_path.metadata() {
            if meta.len() >= LOG_CAP_BYTES {
                std::fs::write(&log_path, b"").map_err(|e| {
                    anyhow::anyhow!("failed to truncate log file {}: {e}", log_path.display())
                })?;
            }
        }
    }

    let file_writer = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| anyhow::anyhow!("failed to open log file {}: {e}", log_path.display()))?;

    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_writer);

    // ── filter ────────────────────────────────────────────────────────────────

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // ── console layer ─────────────────────────────────────────────────────────

    let console_layer = fmt::layer()
        .with_ansi(should_colorize())
        .with_target(true)
        .with_span_events(FmtSpan::NONE)
        .with_writer(std::io::stderr);

    // ── file layer (JSON, no ANSI) ────────────────────────────────────────────

    let file_layer = fmt::layer()
        .json()
        .with_ansi(false)
        .with_span_events(FmtSpan::NONE)
        .with_writer(non_blocking_file);

    // ── assemble ──────────────────────────────────────────────────────────────

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(guard)
}

/// Returns true when the console log output should include ANSI color codes.
///
/// Priority:
/// 1. `NO_COLOR` env var (https://no-color.org) — always disables color.
/// 2. `FORCE_COLOR` env var — always enables color (useful in Docker).
/// 3. Whether stderr is a TTY.
fn should_colorize() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if std::env::var_os("FORCE_COLOR").is_some() {
        return true;
    }
    std::io::stderr().is_terminal()
}

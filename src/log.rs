//! Centralized logging setup for the arbitrage bot.
//!
//! Use `RUST_LOG` to control level (e.g. `RUST_LOG=info`, `RUST_LOG=debug,solana_client=warn`).
//! If `log_level` is set in `[bot]` in config.toml, it is used as the default when `RUST_LOG` is not set.

use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, FmtSubscriber};

/// Initialize global tracing subscriber with timestamps, target (module path), and env filter.
/// Call once at startup (e.g. in `main`).
///
/// - `default_level`: used as default when `RUST_LOG` is not set (e.g. `"info"`).
pub fn init_logging(default_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default_level))
        .unwrap_or_else(|_| EnvFilter::new(default_level));

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_span_events(FmtSpan::CLOSE)
        .with_file(false)
        .with_line_number(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default tracing subscriber");
}

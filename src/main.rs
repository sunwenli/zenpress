mod bot;
mod config;
mod constants;
mod dex;
mod log;
mod pools;
mod refresh;
mod transaction;

use clap::{App, Arg};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Solana Onchain Arbitrage Bot")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Cetipo")
        .about("Multi-DEX onchain arbitrage bot with configurable logging and pool refresh")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .default_value("config.toml"),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap();

    // Load config early so we can apply optional log_level (RUST_LOG still overrides).
    let default_log_level = match config::Config::load(config_path) {
        Ok(c) => c.bot.log_level.as_deref().unwrap_or("info"),
        Err(_) => "info",
    };
    log::init_logging(default_log_level);

    info!(config = %config_path, "Starting Solana Onchain Arbitrage Bot");

    bot::run_bot(config_path).await?;

    Ok(())
}

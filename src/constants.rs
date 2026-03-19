//! Shared constants for the arbitrage bot (e.g. SOL mint).

use solana_program::pubkey::Pubkey;
use std::str::FromStr;

pub const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

pub fn sol_mint() -> Pubkey {
    Pubkey::from_str(SOL_MINT).unwrap()
}

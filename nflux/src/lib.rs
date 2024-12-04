mod config;
mod core;
mod logger;
mod utils;

// Dependencies
pub use config::{FirewallConfig, IcmpRules, Protocol, Rules};
pub use core::set_mem_limit;

/// RXH version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod config;
mod core;
mod logger;
mod utils;

// Dependencies
pub use config::Action;
pub use config::{FirewallConfig, FirewallGlobalConfig, IcmpRules, Protocol};
pub use core::set_mem_limit;

/// RXH version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

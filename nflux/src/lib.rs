pub mod config;
pub mod core;
mod logger;
mod utils;

// Dependencies
pub use config::{Config, FirewallConfig, LoggingConfig, Nflux};
pub use core::set_mem_limit;

/// RXH version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

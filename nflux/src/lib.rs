mod config;
mod core;
mod logger;
mod utils;

// Dependencies
pub use config::{NfluxConfig, Rules, Nflux, Action, Protocol};
pub use core::set_mem_limit;
pub use utils::{is_root_user, wait_for_shutdown};

/// RXH version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

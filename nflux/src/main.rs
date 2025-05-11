use tracing::error;

mod cli;
mod logger;
mod utils;
mod tc_event;
mod try_netrace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start CLI
    match cli::start_cli().await {
        Ok(_) => {} // Do nothing
        Err(err) => {
            error!("Error starting the cli {}", err)
        }
    }

    Ok(())
}

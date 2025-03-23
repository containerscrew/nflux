use tracing::error;

mod cli;
mod logger;
mod netrace;
mod tlstrace;
mod utils;

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

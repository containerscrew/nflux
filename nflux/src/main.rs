use tracing::error;

mod cli;
mod logger;
mod netrace;
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

    // // Spawn a task to process events
    // tokio::spawn(async move { process_event(ring_buf).await });

    // // Wait for shutdown
    // let _ = wait_for_shutdown().await;

    Ok(())
}

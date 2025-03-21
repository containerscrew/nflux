use tracing::error;

mod cli;
mod netrace;
mod custom_logger;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start CLI
    match cli::start_cli() {
        Ok(_) => {
        } // Do nothing
        Err(err) => {
            error!("Error starting the cli {}", err)
        }
    }

    // // Traffic control event ring buffer
    // let tc_event_ring_map = bpf
    //     .take_map("TC_EVENT")
    //     .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer TC_EVENT map"))?;

    // let ring_buf = RingBuf::try_from(tc_event_ring_map)?;

    // // Spawn a task to process events
    // tokio::spawn(async move { process_event(ring_buf).await });

    // // Wait for shutdown
    // let _ = wait_for_shutdown().await;

    Ok(())
}

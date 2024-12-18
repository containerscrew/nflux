use tracing_subscriber::fmt::format::FmtSpan;
use tracing::Level;

pub fn setup_logger(log_level: &str, format: &str) {
    // Match the log level
    let log_level = match log_level {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    // Create a subscriber with stdout as the target
    let base_subscriber = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_thread_names(false)
        .with_span_events(FmtSpan::FULL)
        .with_file(false)
        .with_target(false);

    // JSON or text format (default to JSON for container environments)
    if format == "json" {
        base_subscriber.json().init();
    } else {
        base_subscriber.init();
    }
}

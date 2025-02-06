use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

pub enum LogFormat {
    #[allow(dead_code)]
    Json,
    Text,
}

pub fn setup_logger(log_level: &str, log_format: LogFormat) {
    // Match the log level
    // TODO: validate log level
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
    match log_format {
        LogFormat::Json => base_subscriber.json().init(),
        _ => base_subscriber.init(),
    }
}

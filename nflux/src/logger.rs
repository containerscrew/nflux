use tracing_subscriber::fmt::format::FmtSpan;

pub fn setup_logger(log_level: &String, format: &String) {
    let log_level = match log_level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    // Create a base subscriber with common settings
    let base_subscriber = tracing_subscriber::fmt()
        .with_thread_names(false)
        .with_max_level(log_level)
        .with_span_events(FmtSpan::FULL)
        .with_file(false)
        .with_target(false);

    // Apply JSON or text formatting based on the format parameter
    match format.as_str() {
        "json" => base_subscriber.json().init(),
        "text" => base_subscriber.init(),
        _ => base_subscriber.init(),
    };
}

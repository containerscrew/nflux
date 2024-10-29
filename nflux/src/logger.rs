use tracing_subscriber::fmt::format::FmtSpan;

pub fn setup_logger(log_level: &String) {
    let log_level = match log_level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warning" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_thread_names(false)
        .with_max_level(log_level)
        .with_span_events(FmtSpan::FULL)
        .with_file(false)
        .with_target(false)
        // .with_timer(CustomTimeFormatter)
        .init();
}

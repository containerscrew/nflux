use chrono::Local;
use tracing::Level;
use tracing_subscriber::fmt::{
    format::{FmtSpan, Writer},
    time::FormatTime,
};

/// MyTimer is a custom timer for the logs.
pub struct MyTimer;

/// Implement the FormatTime trait for MyTimer.
impl FormatTime for MyTimer {
    fn format_time(
        &self,
        w: &mut Writer<'_>,
    ) -> std::fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub struct LoggerConfig {
    pub level: String,
    pub format: String,
    pub with_timer: bool,
}

/// setup_logger initializes the logger with the given log level and format.
pub fn init_logger(logger_config: LoggerConfig) {
    let log_level = match logger_config.level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            eprintln!(
                "Invalid log level: {}. Defaulting to info",
                logger_config.level
            );
            Level::INFO
        }
    };

    let base_subscriber = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_thread_names(false)
        .with_span_events(FmtSpan::FULL)
        .with_file(false)
        .with_target(false);

    match (logger_config.format.as_str(), logger_config.with_timer) {
        ("text", false) => base_subscriber.without_time().init(),
        ("json", false) => base_subscriber
            .without_time()
            .json()
            .flatten_event(true)
            .init(),
        ("text", true) => base_subscriber.with_timer(MyTimer).init(),
        ("json", true) => base_subscriber.json().flatten_event(true).init(),
        _ => base_subscriber.init(), // Defaults to text
    }
}

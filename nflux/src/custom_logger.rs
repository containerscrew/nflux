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
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d-%H:%M:%S"))
    }
}

// /// LogFormat represents the format of the logs.
// pub enum LogFormat {
//     #[allow(dead_code)]
//     Json,
//     Text,
// }

/// setup_logger initializes the logger with the given log level and format.
pub fn setup_logger(log_level: &str, log_format: &str) {
    let log_level = match log_level {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            eprintln!("Invalid log level: {}. Defaulting to info", log_level);
            Level::INFO
        }
    };

    let base_subscriber = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_thread_names(false)
        .with_span_events(FmtSpan::FULL)
        .with_file(false)
        .with_target(false)
        .with_timer(MyTimer);

    match log_format {
        "json" => base_subscriber.json().init(),
        _ => base_subscriber.init(), // Defaults to text
    }
}

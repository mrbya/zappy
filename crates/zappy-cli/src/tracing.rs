use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initializes CLI logging from verbosity level.
///
/// Verbosity mapping:
///
/// ```text
/// 0 => warn
/// 1 => info
/// 2 => debug
/// 3+ => trace
/// ```
pub fn init_trace(verbosity: u8) {
    let filter = EnvFilter::new(default_filter(verbosity));

    let format_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_target(false)
        .without_time();

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(format_layer);

    // This call can fail in tests if a subscriber was already installed.
    // Logging is best-effort; command execution should not fail because of it.
    if subscriber.try_init().is_err() {
        // Intentionally ignored.
    }
}

/// Builds default tracing filter.
pub fn default_filter(verbosity: u8) -> String {
    let level = match verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    format!(
        "zappy={level},zappy_cli={level},zappy_core={level},zappy_fs={level},zappy_hooks={level},\
         zappy_templates={level}"
    )
}

//! Dev deps: some useful macros.

#[cfg(feature = "macros-base64")]
pub mod base64;
#[cfg(feature = "macros-hash")]
pub mod hash;
#[cfg(feature = "macros-random")]
pub mod random;
#[cfg(feature = "macros-string")]
pub mod string;

#[macro_export]
/// Faster way to get current timestamp other than
/// `chrono::Local::now().timestamp()`, 12x faster on my machine.
///
/// # Example
///
/// ```rust
/// # use macro_toolset::now;
/// let now_ts_sec = now!().as_secs(); // Seconds since UNIX_EPOCH
/// let now_ts_millis = now!().as_millis(); // Milliseconds since UNIX_EPOCH
/// ```
///
/// See [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html) for more details.
macro_rules! now {
    () => {{
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(t) => t,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }};
}

#[macro_export]
/// Init `tracing_subscriber` with default settings.
///
/// This is useful when running tests.
macro_rules! init_tracing_simple {
    () => {{
        use tracing::level_filters::LevelFilter;
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

        let fmt_layer = tracing_subscriber::fmt::layer().with_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy()
                .add_directive("otel::tracing=trace".parse().unwrap())
                .add_directive("h2=error".parse().unwrap())
                .add_directive("tower=error".parse().unwrap())
                .add_directive("hyper=error".parse().unwrap()),
        );

        tracing_subscriber::registry().with(fmt_layer).init();
    }};
}

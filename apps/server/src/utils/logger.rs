use tracing_subscriber::{fmt::time::ChronoLocal, prelude::*, util::SubscriberInitExt};

pub async fn init_tracing() {
    let appender = tracing_appender::rolling::daily("./logs", "gd.log");

    // use native time format
    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_timer(timer.clone())
                .with_writer(appender),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_timer(timer)
                .pretty(),
        )
        .init();
}

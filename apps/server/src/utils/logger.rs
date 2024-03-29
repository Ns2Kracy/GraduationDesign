use tracing_subscriber::{fmt::time::ChronoUtc, prelude::*, util::SubscriberInitExt};

pub async fn init_tracing() {
    let appender = tracing_appender::rolling::daily("./logs", "gd.log");

    let timer = ChronoUtc::new("%F %T".to_string());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_timer(timer.clone())
                .pretty()
                .with_writer(appender)
                .with_file(false)
                .with_line_number(false),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_timer(timer.clone())
                .pretty()
                .with_file(false)
                .with_line_number(false),
        )
        .init();
}

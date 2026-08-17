use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() -> anyhow::Result<WorkerGuard> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum_ecommerce=debug,tower_http=debug,axum=debug,sqlx=warn".into());

    let is_production = std::env::var("APP_ENV")
        .map(|v| v.eq_ignore_ascii_case("production"))
        .unwrap_or(false);

    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
    let registry = tracing_subscriber::registry().with(env_filter);

    if is_production {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(non_blocking)
                    .with_target(true)
                    .with_current_span(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(false),
            )
            .try_init()?;
    } else {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_writer(non_blocking)
                    .with_target(true)
                    .with_file(false)
                    .with_line_number(true),
            )
            .try_init()?;
    }

    Ok(guard)
}

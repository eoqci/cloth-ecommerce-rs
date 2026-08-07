mod app;
mod app_state;
mod config;
mod error;
mod errors;
mod infra;
mod modules;
mod shared;
mod telemetry;

use app_state::AppState;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ===================================================
    // ===============| CONFIG INIT |=====================
    // ===================================================
    let config = Config::init()?;

    // ===================================================
    // ===============| LOGGER INIT |=====================
    // ===================================================
    telemetry::init();
    tracing::info!("Starting Server...");

    // ===================================================
    // =============| DATABASE CONNECTION |===============
    // ===================================================
    let pool = infra::db::create_pool(&config.database_url)
        .await
        .expect("Failed to coneect to database");
    tracing::info!("Successfully connected to Database!");

    // ===================================================
    // =================| APP STATE |=====================
    // ===================================================
    let state = AppState::new(config, pool);
    let addr = format!("{}:{}", state.config.server_host, state.config.server_port);
    let app = app::create_router(state);
    // Email service inside appstate
    tracing::info!("Email service initialized");

    // ===================================================
    // =============| AXUM SERVE - LISTENER |=============
    // ===================================================
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening at: {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Shutting down server...");
}

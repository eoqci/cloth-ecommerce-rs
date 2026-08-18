use std::sync::Arc;

use axum_ecommerce::app_state::AppState;
use axum_ecommerce::config::Config;
use axum_ecommerce::infra::db;
use axum_ecommerce::{app, telemetry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ===================================================
    // ===============| CONFIG INIT |=====================
    // ===================================================
    let config = Arc::new(Config::init()?);

    // ===================================================
    // ===============| LOGGER INIT |=====================
    // ===================================================
    let _guard = telemetry::init()?;
    tracing::info!("Starting Server...");

    // ===================================================
    // =============| DATABASE CONNECTION |===============
    // ===================================================
    let pool = db::create_pool(&config.database_url.clone())
        .await
        .expect("Failed to coneect to database");
    tracing::info!("Successfully connected to Database!");

    // ===================================================
    // =================| HTTP CLIENT |===================
    // ===================================================

    // for some dog sh1t reason or I can't read, http client doesn't work with googleplay oauth
    // so, the way i will handle this is make two client for this
    // the fn receive 2 params (oauth_http_client, http_client)
    let oauth_http_client = oauth2::reqwest::ClientBuilder::new()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .expect("failed to build oauth http client");
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("failed to build http client");

    // ===================================================
    // =================| APP STATE |=====================
    // ===================================================
    let state = AppState::new(
        Arc::clone(&config),
        pool,
        oauth_http_client.clone(),
        http_client.clone(),
    )?;

    let addr = format!("{}:{}", state.config.server_host, state.config.server_port);
    let app = app::create_router(state.clone());
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
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down server...");
}

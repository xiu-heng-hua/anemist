use anemist::{AppState, Storage, router};
use anyhow::{Context, Result, anyhow};
use std::sync::Arc;
use tokio::sync::RwLock;

// Configuration module with constants for database path and server address
mod config {
    // Path to the JSON file used for data storage
    pub const DATABASE_PATH: &str = "anemist.json";
    // Address and port where the server will listen
    pub const LISTEN_ADDRESS: &str = "127.0.0.1:3000";
}

// Entry point for the asynchronous web server using Tokio runtime
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize storage and load entries from the data file
    let mut storage = Storage::new(config::DATABASE_PATH);
    storage.load().await.with_context(|| {
        format!(
            "Failed to load entries from data file: {}",
            config::DATABASE_PATH
        )
    })?;

    // Initialize app state from storage for thread-safe sharing
    let state = AppState::new(Arc::new(RwLock::new(storage)));

    // Create Axum router with defined routes
    let router = router::create(state);

    // Bind TCP listener to the configured address
    let listener = tokio::net::TcpListener::bind(config::LISTEN_ADDRESS)
        .await
        .with_context(|| {
            format!(
                "Failed to bind listener to address: {}",
                config::LISTEN_ADDRESS
            )
        })?;

    // Start the Anemist server to handle HTTP requests
    axum::serve(listener, router)
        .await
        .map_err(|e| anyhow!("Failed to serve the service: {}", e))
}

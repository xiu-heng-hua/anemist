use crate::AppState;
use axum::{extract::State, http::StatusCode, response::Redirect};

/// Handles `POST /load` to fetch entries from the data file to the storage.
///
/// # Arguments
/// * `state` - The application state containing the shared storage.
///
/// # Returns
/// * `Ok(Redirect)` - Redirects to the index page on success.
/// * `Err(StatusCode)` - If loading fails.
pub async fn handle(State(state): State<AppState>) -> Result<Redirect, StatusCode> {
    let mut storage = state.storage.write().await;
    storage
        .save()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/"))
}

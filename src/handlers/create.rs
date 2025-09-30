use crate::AppState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::Redirect,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateForm {
    id: usize,
}

/// Handles `POST /create` to add a new anime entry.
///
/// # Arguments
/// * `state` - The application state containing the shared storage.
/// * `form` - Form data containing the MyAnimeList ID.
///
/// # Returns
/// * `Ok(Redirect)` - Redirects to the index page on success.
/// * `Err(StatusCode)` - If fetching or storing fails.
pub async fn handle(
    State(state): State<AppState>,
    Form(form): Form<CreateForm>,
) -> Result<Redirect, StatusCode> {
    let mut storage = state.storage.write().await;
    storage
        .create_entry(form.id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Redirect::to("/"))
}

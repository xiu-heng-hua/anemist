use crate::AppState;
use axum::{
    extract::{Form, State},
    response::Redirect,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteForm {
    id: usize,
}

/// Handles `DELETE /delete/:id` to remove anime entries with the given ID.
///
/// # Arguments
/// * `state` - The application state containing the shared storage.
/// * `form` - Form data containing the MyAnimeList ID.
///
/// # Returns
/// * Redirect - Redirects to the index page.
pub async fn handle(State(state): State<AppState>, Form(form): Form<DeleteForm>) -> Redirect {
    let mut storage = state.storage.write().await;
    storage.delete_entry(form.id);
    Redirect::to("/")
}

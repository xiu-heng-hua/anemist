use crate::{AppState, handlers};
use axum::{
    Router,
    routing::{get, post},
};

/// Creates an Axum router for Anemist.
///
/// # Arguments
/// * `state` - The application state, containing shared data like the storage backend.
///
/// # Returns
/// An Axum `Router` configured with routes for the index page, anime entry management, and image serving.
pub fn create(state: AppState) -> Router {
    Router::new()
        // Serve the index page with a list of anime entries
        .route("/", get(handlers::index::handle))
        // Create a new anime entry by ID
        .route("/create", post(handlers::create::handle))
        // Delete an anime entry by ID
        .route("/delete", post(handlers::delete::handle))
        // Save anime entries to persistent storage
        .route("/save", post(handlers::save::handle))
        // Load anime entries from persistent storage
        .route("/load", post(handlers::load::handle))
        // Serve the cover image for an anime by ID
        .route("/image/{id}", get(handlers::image::handle))
        .with_state(state)
}

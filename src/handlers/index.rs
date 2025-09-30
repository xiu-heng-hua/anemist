use crate::{Anime, AppState};
use askama::Template;
use axum::{extract::State, http::StatusCode, response::Html};

#[derive(Template)]
#[template(path = "index.html")]
struct AnemistTemplate<'a> {
    entries: &'a [Anime],
}

/// Handles `GET /` to render the index page.
///
/// # Arguments
/// * `state` - The application state containing the shared storage.
///
/// # Returns
/// * `Ok(Html<String>)` - The rendered index page.
/// * `Err(StatusCode)` - If template rendering fails.
pub async fn handle(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let storage = state.storage.read().await;
    let template = AnemistTemplate {
        entries: &storage.data,
    };
    let html = template
        .render()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(html))
}

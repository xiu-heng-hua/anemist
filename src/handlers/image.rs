use crate::{AppState, myanimelist};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
};
use tokio::fs;

/// Handles `GET /image/{id}` to fetch and serve an anime's cover image.
///
/// Checks if `/cache/{id}.jpg` exists; if so, returns it. Otherwise, downloads
/// the image from the anime's URL, saves it to `/images/{id}.jpg`, and returns it.
///
/// # Arguments
/// * `state` - The application state containing the shared storage.
/// * `id` - The MyAnimeList ID of the anime.
///
/// # Returns
/// * `Ok(([(&str, &str); 1], Bytes))` - The image data with content type.
/// * `Err(StatusCode)` - If the entry, file, or image fetch fails.
pub async fn handle(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Result<([(&'static str, &'static str); 1], Bytes), StatusCode> {
    // Construct file path for cached image
    let file_path = format!("cache/{}.jpg", id);

    // Check if image exists in `/image/` directory
    if fs::metadata(&file_path).await.is_ok() {
        // Read cached image
        let bytes = fs::read(&file_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into();
        return Ok(([("Content-Type", "image/jpeg")], bytes));
    }

    // Acquire read lock on storage
    let storage = state.storage.read().await;

    // Find anime entry in storage or fetch from MyAnimeList
    let entry = match storage.data.iter().find(|entry| entry.id() == id) {
        Some(entry) => entry,
        None => {
            // Fallback to fetching from MyAnimeList
            &myanimelist::fetch_anime(id)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?
        }
    };

    // Fetch image from URL
    let bytes = reqwest::get(entry.image().as_str())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .bytes()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Save image to `/image/` directory
    fs::write(&file_path, &bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(([("Content-Type", "image/jpeg")], bytes))
}

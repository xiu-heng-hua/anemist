use crate::Anime;
use anyhow::{Context, Result, anyhow};
use scraper::{Html, Selector};

/// Base URL for MyAnimeList anime pages.
const MAL_ANIME_BASE_URL: &str = "https://myanimelist.net/anime";

/// CSS selector for the anime title in MyAnimeList's Open Graph metadata.
const MAL_OG_TITLE_SELECTOR: &str = r#"meta[property="og:title"]"#;

/// CSS selector for the anime cover image URL in MyAnimeList's Open Graph metadata.
const MAL_OG_IMAGE_SELECTOR: &str = r#"meta[property="og:image"]"#;

/// Extracts the `content` attribute from the first HTML element matching the given selector.
fn extract_content_attr(document: &Html, selector: &Selector) -> Result<String> {
    let element = document
        .select(selector)
        .next()
        .ok_or_else(|| anyhow!("No matching element"))?;
    let content = element
        .attr("content")
        .ok_or_else(|| anyhow!("No content attribute"))?;
    Ok(content.to_owned())
}

/// Fetches an anime entry from MyAnimeList by ID.
///
/// # Arguments
/// * `id` - The MyAnimeList ID of the anime.
///
/// # Returns
/// * `Ok(Anime)` - An `Anime` instance with the scraped title and image URL.
/// * `Err` - If the HTTP request fails, HTML parsing fails, or the `Anime` creation fails.
///
/// # Errors
/// Returns an error if:
/// - The HTTP request to MyAnimeList fails.
/// - The HTML response lacks expected metadata (title or image).
/// - The `Anime::new` constructor fails (e.g., invalid image URL).
pub async fn fetch_anime(id: usize) -> Result<Anime> {
    let url = format!("{MAL_ANIME_BASE_URL}/{id}");

    // Fetch and decode HTTP response
    let text = reqwest::get(&url)
        .await
        .with_context(|| format!("Failed to fetch URL: {}", &url))?
        .text()
        .await
        .with_context(|| format!("Failed to decode response text for ID {}", id))?;

    // Parse HTML document
    let html = Html::parse_document(&text);

    // Parse CSS selectors
    let title_selector: Selector = Selector::parse(MAL_OG_TITLE_SELECTOR)
        .map_err(|e| anyhow!("Failed to parse title selector: {:?}", e))?;
    let image_selector = Selector::parse(MAL_OG_IMAGE_SELECTOR)
        .map_err(|e| anyhow!("Failed to parse image selector: {:?}", e))?;

    // Extract entry fields
    let title = extract_content_attr(&html, &title_selector)
        .with_context(|| format!("Failed to extract the title for ID {}", id))?;
    let image = extract_content_attr(&html, &image_selector)
        .with_context(|| format!("Failed to extract the image for ID {}", id))?;

    // Create Anime instance
    Anime::new(id, title, image)
        .with_context(|| format!("Failed to create Anime instance for ID {}", id))
}

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use url::Url;

/// Represents an anime entry fetched from MyAnimeList.
#[derive(Deserialize, Serialize)]
pub struct Anime {
    /// The unique ID of the anime on MyAnimeList.
    id: usize,
    /// The title of the anime.
    title: String,
    /// The URL of the anime's cover image.
    image: Url,
}

impl Anime {
    /// Creates a new `Anime` instance with validated fields.
    ///
    /// # Arguments
    /// * `id` - The MyAnimeList ID of the anime.
    /// * `title` - The title of the anime (must not be empty).
    /// * `image` - The URL of the anime's cover image (must be a valid URL).
    ///
    /// # Returns
    /// * `Ok(Anime)` - A new `Anime` instance.
    /// * `Err` - If the title is empty or the image URL is invalid.
    pub fn new(id: usize, title: impl Into<String>, image: impl AsRef<str>) -> Result<Self> {
        let title = title.into();
        if title.is_empty() {
            bail!("The provided title is empty")
        }
        let image = Url::parse(image.as_ref())
            .with_context(|| format!("The provided image is invalid: {}", image.as_ref()))?;
        Ok(Anime { id, title, image })
    }

    /// Returns the anime's ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the anime's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the anime's image URL.
    pub fn image(&self) -> &Url {
        &self.image
    }
}

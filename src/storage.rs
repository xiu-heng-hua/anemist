use crate::{Anime, myanimelist};
use anyhow::{Ok, Result};
use std::sync::Arc;
use tokio::{fs, sync::RwLock};

/// Manages a collection of anime entries, persisted to a data file.
pub struct Storage {
    /// In-memory list of anime entries.
    pub data: Vec<Anime>,
    /// Path to the data file for persistence.
    pub db_path: String,
}

impl Storage {
    /// Creates a new `Storage` with an empty list of entries and a file path.
    pub fn new(db_path: impl Into<String>) -> Self {
        Storage {
            data: vec![],
            db_path: db_path.into(),
        }
    }

    /// Adds or updates an anime entry by fetching it from MyAnimeList.
    ///
    /// # Arguments
    /// * `id` - The MyAnimeList ID of the anime.
    ///
    /// # Returns
    /// * `Ok(())` - If the entry was successfully added or updated.
    /// * `Err` - If fetching the anime fails.
    pub async fn create_entry(&mut self, id: usize) -> Result<()> {
        let entry = myanimelist::fetch_anime(id).await?;
        self.data.retain(|entry| entry.id() != id);
        self.data.push(entry);
        self.data.sort_by(|a, b| a.title().cmp(b.title()));
        Ok(())
    }

    /// Deletes all current anime entries with the given ID.
    ///
    /// If no entry with the given ID exists, the in-memory entries are unchanged.
    ///
    /// # Arguments
    /// * `id` - The MyAnimeList ID of the anime to delete.
    pub fn delete_entry(&mut self, id: usize) {
        self.data.retain(|entry| entry.id() != id);
    }

    /// Saves the current entries to the data file.
    ///
    /// # Returns
    /// * `Ok(())` - If the save was successful.
    /// * `Err` - If serialization or file writing fails.
    pub async fn save(&mut self) -> Result<()> {
        let buffer = serde_json::to_vec(&self.data)?;
        fs::write(&self.db_path, &buffer).await?;
        Ok(())
    }

    /// Loads entries from the data file, replacing the current entries.
    ///
    /// # Returns
    /// * `Ok(())` - If the load was successful.
    /// * `Err` - If file reading or deserialization fails.
    pub async fn load(&mut self) -> Result<()> {
        let buffer = fs::read(&self.db_path).await?;
        let entries: Vec<Anime> = serde_json::from_slice(&buffer)?;
        self.data.clear();
        self.data.extend(entries);
        Ok(())
    }
}

pub type SharedStorage = Arc<RwLock<Storage>>;

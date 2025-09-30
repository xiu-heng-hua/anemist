use crate::SharedStorage;

/// Application state for Anemist, holding shared storage for anime entries.
#[derive(Clone)]
pub struct AppState {
    /// Thread-safe storage for managing anime entries, backed by anemist.json.
    pub storage: SharedStorage,
}

impl AppState {
    /// Creates a new `AppState` with the given storage.
    pub fn new(storage: SharedStorage) -> Self {
        AppState { storage }
    }
}

pub mod appstate;
pub mod handlers;
pub mod models;
pub mod myanimelist;
pub mod router;
pub mod storage;

pub use appstate::AppState;
pub use models::Anime;
pub use storage::{SharedStorage, Storage};

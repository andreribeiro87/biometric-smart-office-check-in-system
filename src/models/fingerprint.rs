use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

// Shared state to store the last fingerprint
#[derive(Clone, Default)]
pub struct AppState {
    pub latest_fingerprint: Arc<RwLock<Option<String>>>,
}

#[derive(Serialize)]
pub struct FingerprintResponse {
    // Can be used for validation
    pub has_data: bool,
    pub data_size: usize,

    // The actual fingerprint data
    pub fingerprint: Option<String>,
}

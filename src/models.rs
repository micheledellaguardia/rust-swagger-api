use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct All {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAll {
    pub title: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAll {
    pub title: Option<String>,
    pub done: Option<bool>,
}

#[derive(Clone, Default)]
pub struct AppState {
    pub alls: Arc<Mutex<Vec<All>>>,
}

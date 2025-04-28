use serde::{Deserialize, Serialize};

/// Represents an application instance (a running Docker container).
#[derive(Serialize, Deserialize, Clone)]
pub struct AppInstance {
    pub id: u32,
    pub name: String,
    pub status: AppStatus,
    pub image: String,
    pub ports: Vec<u16>,
    pub created_at: String,
}

/// Represents the status of an application.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppStatus {
    Running,
    Stopped,
    Error,
}

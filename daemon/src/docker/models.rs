use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Container creation parameters
pub struct ContainerConfig<'a> {
    pub name: &'a str,
    pub image: &'a str,
    pub host_ports: &'a [u16],
    pub container_port: u16,
    pub labels: Option<&'a HashMap<String, String>>,
    pub env: Option<&'a HashMap<String, String>>,
    pub volumes: Option<&'a Vec<String>>,
    pub restart_policy: Option<&'a str>,
}

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

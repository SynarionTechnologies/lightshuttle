use axum::{extract::Query, extract::Path, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppInstance {
    pub id: u32,
    pub name: String,
    pub status: AppStatus,
    pub image: String,
    pub ports: Vec<u16>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppStatus {
    Running,
    Stopped,
    Error,
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    total: usize,
    page: usize,
    limit: usize,
    items: Vec<T>,
}

pub async fn list_apps(Query(pagination): Query<Pagination>) -> (StatusCode, Json<PaginatedResponse<AppInstance>>) {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let all_apps = get_mock_apps();
    let total = all_apps.len();

    let start = (page - 1).saturating_mul(limit);
    let end = (start + limit).min(total);
    
    let items = if start >= total {
        vec![]
    } else {
        all_apps[start..end].to_vec()
    };
    
    let response = PaginatedResponse {
        total,
        page,
        limit,
        items,
    };

    (StatusCode::OK, Json(response))
}

pub async fn get_app(Path(id): Path<u32>) -> (StatusCode, Json<Option<AppInstance>>) {
    let app = get_mock_apps().into_iter().find(|a| a.id == id);

    match app {
        Some(app) => (StatusCode::OK, Json(Some(app))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

pub fn get_mock_apps() -> Vec<AppInstance> {
    let statuses = vec![
        AppStatus::Running,
        AppStatus::Stopped,
        AppStatus::Error,
    ];

    (1..=50)
        .map(|id| AppInstance {
            id,
            name: format!("app-{}", id),
            status: statuses[(id as usize) % statuses.len()].clone(),
            image: format!("image-{}:latest", id),
            ports: vec![8000 + id as u16],
            created_at: "2025-04-22T15:00:00Z".to_string(),
        })
        .collect()
}

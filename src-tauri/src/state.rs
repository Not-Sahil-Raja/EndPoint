use sqlx::SqlitePool;
use tauri_plugin_http::reqwest;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub http: reqwest::Client,
}

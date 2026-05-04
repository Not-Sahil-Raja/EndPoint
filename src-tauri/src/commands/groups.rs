use tauri::State;

use crate::db::groups;
use crate::state::AppState;

#[tauri::command]
pub async fn get_all_groups(state: State<'_, AppState>) -> Result<Vec<groups::Group>, String> {
    let pool = &state.db;
    match groups::get_all(pool).await {
        Ok(groups) => Ok(groups),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn create_group(
    state: State<'_, AppState>,
    name: String,
) -> Result<groups::Group, String> {
    let pool = &state.db;
    match groups::create(pool, &name).await {
        Ok(group) => Ok(group),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn delete_group(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = &state.db;
    match groups::delete(pool, id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn rename_group(
    state: State<'_, AppState>,
    id: i64,
    name: String,
) -> Result<groups::Group, String> {
    let pool = &state.db;
    match groups::update(pool, id, &name).await {
        Ok(group) => Ok(group),
        Err(e) => Err(e.to_string()),
    }
}

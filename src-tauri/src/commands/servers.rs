use tauri::{Emitter, State};

use crate::db::servers::{self, Server};
use crate::state::AppState;

#[tauri::command]
pub async fn get_servers(state: State<'_, AppState>) -> Result<Vec<Server>, String> {
    servers::get_all(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_server(id: i64, state: State<'_, AppState>) -> Result<Server, String> {
    servers::get_one(&state.db, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_server(
    name: String,
    url: String,
    group_id: Option<i64>,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<Server, String> {
    let result = servers::insert(&state.db, &name, &url, group_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Err(err) = window.emit("servers-changed", ()) {
        eprintln!("Emit error: {}", err);
    }

    Ok(result)
}

#[tauri::command]
pub async fn update_server(
    id: i64,
    name: String,
    url: String,
    group_id: Option<i64>,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<Server, String> {
    let result = servers::update(&state.db, id, &name, &url, group_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Err(err) = window.emit("servers-changed", ()) {
        eprintln!("Emit error: {}", err);
    }

    Ok(result)
}

#[tauri::command]
pub async fn delete_server(
    id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<(), String> {
    servers::delete(&state.db, id)
        .await
        .map_err(|e| e.to_string())?;

    if let Err(err) = window.emit("servers-changed", ()) {
        eprintln!("Emit error: {}", err);
    }

    Ok(())
}

#[tauri::command]
pub async fn get_servers_by_groups_map(
    state: State<'_, AppState>,
) -> Result<std::collections::HashMap<String, Vec<crate::db::servers::Server>>, String> {
    let pool = &state.db;
    match crate::db::servers::get_servers_by_groups_map(pool).await {
        Ok(groups_map) => {
            let mut result = std::collections::HashMap::new();
            for (key, servers) in groups_map {
                let string_key = match key {
                    Some(id) => id.to_string(),
                    None => "null".to_string(),
                };
                result.insert(string_key, servers);
            }
            Ok(result)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn assign_server_group(
    server_id: i64,
    group_id: Option<i64>,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // First get the current server details
    let server = servers::get_one(&state.db, server_id)
        .await
        .map_err(|e| e.to_string())?;

    // Update the server with the new group_id
    servers::update(&state.db, server_id, &server.name, &server.url, group_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Err(err) = window.emit("servers-changed", ()) {
        eprintln!("Emit error: {}", err);
    }

    Ok(())
}

#[tauri::command]
pub async fn toggle_server_sync(
    server_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<crate::db::servers::Server, String> {
    let server = servers::toggle_sync_enabled(&state.db, server_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Err(err) = window.emit("servers-changed", ()) {
        eprintln!("Emit error: {}", err);
    }

    Ok(server)
}

#[tauri::command]
pub async fn toggle_group_sync(
    group_id: Option<i64>,
    enabled: bool,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<Vec<crate::db::servers::Server>, String> {
    let updated_servers = servers::toggle_group_sync_enabled(&state.db, group_id, enabled)
        .await
        .map_err(|e| e.to_string())?;

    if let Err(err) = window.emit("servers-changed", ()) {
        eprintln!("Emit error: {}", err);
    }

    Ok(updated_servers)
}

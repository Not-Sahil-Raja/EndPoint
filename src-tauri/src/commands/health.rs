use once_cell::sync::Lazy;
use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::{Emitter, State};

use crate::db::servers;
use crate::state::AppState;

static IS_RUNNING: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
static INTERVAL_SECS: Lazy<Arc<AtomicU64>> = Lazy::new(|| Arc::new(AtomicU64::new(30)));
static LAST_CHECK: Lazy<Arc<AtomicU64>> = Lazy::new(|| Arc::new(AtomicU64::new(0)));

#[derive(Serialize)]
pub struct HealthCheckStatus {
    pub is_running: bool,
    pub interval_secs: u64,
    pub last_check: u64, // Unix timestamp
    pub next_check: u64, // Unix timestamp
}

const TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Serialize, Clone)]
pub struct HealthResult {
    pub server_id: i64,
    pub url: String,
    pub status: String,
    pub response_time: i64,
    pub ok: bool,
}

async fn ping(http: &tauri_plugin_http::reqwest::Client, url: &str) -> (String, i64, bool) {
    let start = std::time::Instant::now();
    match http.get(url).timeout(TIMEOUT).send().await {
        Ok(r) => {
            let ms = start.elapsed().as_millis() as i64;
            let ok = r.status().is_success();
            (r.status().to_string(), ms, ok)
        }
        Err(e) => (format!("error: {e}"), 0, false),
    }
}

async fn check_and_persist(app_state: &AppState, app: &tauri::AppHandle) {
    let servers = match servers::get_sync_enabled_servers(&app_state.db).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to fetch sync-enabled servers: {e}");
            return;
        }
    };

    let mut any_changed = false;

    for server in servers {
        let (status, response_time, _ok) = ping(&app_state.http, &server.url).await;

        if let Err(e) =
            servers::update_status(&app_state.db, server.id, &status, response_time).await
        {
            eprintln!("Status update failed for {}: {e}", server.url);
            continue;
        }

        any_changed = true;
    }

    // Update last check time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    LAST_CHECK.store(now, Ordering::SeqCst);

    if any_changed {
        if let Err(e) = app.emit("servers-changed", ()) {
            eprintln!("Emit error: {e}");
        }
    }
}

#[tauri::command]
pub async fn check_health(
    server_id: i64,
    state: State<'_, AppState>,
) -> Result<HealthResult, String> {
    let server = servers::get_one(&state.db, server_id)
        .await
        .map_err(|e| e.to_string())?;

    let (status, response_time, ok) = ping(&state.http, &server.url).await;

    servers::update_status(&state.db, server.id, &status, response_time)
        .await
        .map_err(|e| e.to_string())?;

    Ok(HealthResult {
        server_id,
        url: server.url,
        status,
        response_time,
        ok,
    })
}

#[tauri::command]
pub async fn start_periodic_health_checks(
    interval_secs: u64,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    if IS_RUNNING.load(Ordering::SeqCst) {
        return Err("Health checks are already running.".into());
    }
    if interval_secs == 0 {
        return Err("interval_secs must be greater than 0.".into());
    }

    // Store the interval
    INTERVAL_SECS.store(interval_secs, Ordering::SeqCst);
    IS_RUNNING.store(true, Ordering::SeqCst);

    let app_state = state.inner().clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        println!("Health checks started (every {interval_secs}s).");

        while IS_RUNNING.load(Ordering::SeqCst) {
            interval.tick().await;
            check_and_persist(&app_state, &app).await;
        }
        println!("Health checks stopped.");
    });

    Ok(())
}

#[tauri::command]
pub fn stop_periodic_health_checks() -> Result<(), String> {
    if IS_RUNNING.load(Ordering::SeqCst) {
        IS_RUNNING.store(false, Ordering::SeqCst);
        Ok(())
    } else {
        Err("No health check loop is running.".into())
    }
}

#[tauri::command]
pub fn get_periodic_health_check_status() -> HealthCheckStatus {
    let is_running = IS_RUNNING.load(Ordering::SeqCst);
    let interval_secs = INTERVAL_SECS.load(Ordering::SeqCst);
    let last_check = LAST_CHECK.load(Ordering::SeqCst);

    let next_check = if is_running && last_check > 0 {
        last_check + interval_secs
    } else {
        0
    };

    HealthCheckStatus {
        is_running,
        interval_secs,
        last_check,
        next_check,
    }
}

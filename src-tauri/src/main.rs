#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod state;

use sqlx::sqlite::SqlitePoolOptions;
use state::AppState;
use tauri::Manager;
use tauri_plugin_http::reqwest;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            let db_path = app
                .path()
                .app_data_dir()
                .expect("Could not resolve app data dir")
                .join("app.db");

            std::fs::create_dir_all(db_path.parent().unwrap())
                .expect("Could not create app data dir");

            let db_url = format!("sqlite:{}?mode=rwc", db_path.to_string_lossy());

            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(4)
                    .connect(&db_url)
                    .await
                    .expect("Failed to connect to database");

                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to run migrations");

                pool
            });

            let http = reqwest::Client::builder()
                .build()
                .expect("Failed to build HTTP client");

            app.manage(AppState { db: pool, http });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::servers::get_servers,
            commands::servers::get_server,
            commands::servers::add_server,
            commands::servers::update_server,
            commands::servers::delete_server,
            commands::servers::get_servers_by_groups_map,
            commands::servers::assign_server_group,
            commands::servers::toggle_server_sync,
            commands::servers::toggle_group_sync,
            commands::health::check_health,
            commands::health::start_periodic_health_checks,
            commands::health::stop_periodic_health_checks,
            commands::health::get_periodic_health_check_status,
            commands::groups::get_all_groups,
            commands::groups::create_group,
            commands::groups::delete_group,
            commands::groups::rename_group,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

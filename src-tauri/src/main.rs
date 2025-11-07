// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_sql::{Migration, MigrationKind};

mod commands;

fn main() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: include_str!("../migrations/001_init.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_indexes",
            sql: include_str!("../migrations/002_add_indexes.sql"),
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:app.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![commands::check_health])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

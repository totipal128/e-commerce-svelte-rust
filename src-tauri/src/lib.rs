use sqlx::{Pool, Postgres};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod app;
mod base;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            app::authentication::controller::users::get_all_users,
            app::master_data::controller::items::items_get,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

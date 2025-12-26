use sqlx::{Pool, Postgres};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod app;
mod base;
pub async fn conn_postgrest() -> Result<Pool<Postgres>, sqlx::Error> {
    let result = crate::base::database::postgres::conn::postgrest_conn().await;

    // automatis migrasi data ke databases
    base::database::postgres::conn::migrate_db().await?;

    if result.is_ok() {
        return result;
    }

    println!("Connection To Databases Failed !!!");
    result
}

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

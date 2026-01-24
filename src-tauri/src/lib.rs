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
            app::authentication::controller::users::get_detail_users_by_id,
            app::authentication::controller::users::create_users,
            app::authentication::controller::users::update_users,
            app::authentication::controller::users::delete_users,
            // item
            app::master_data::controller::items::items_get,
            app::master_data::controller::items::get_items_by_id,
            app::master_data::controller::items::items_create,
            app::master_data::controller::items::items_update,
            app::master_data::controller::items::items_delete,
            app::master_data::controller::items::get_items_by_barcode,
            app::master_data::controller::items::get_items_price__by_items_id,
            //
            app::master_data::controller::sale::sale_list,
            app::master_data::controller::sale::sale_by_id,
            app::master_data::controller::sale::sale_create,
            app::master_data::controller::sale::sale_update,
            app::master_data::controller::sale::sale_delete,
            //
            app::master_data::controller::consumer::consumer_list,
            app::master_data::controller::consumer::consumer_by_id,
            app::master_data::controller::consumer::consumer_create,
            app::master_data::controller::consumer::consumer_update,
            app::master_data::controller::consumer::consumer_delete,
            //
            app::master_data::controller::supplier::supplier_list,
            app::master_data::controller::supplier::supplier_by_id,
            app::master_data::controller::supplier::supplier_create,
            app::master_data::controller::supplier::supplier_update,
            app::master_data::controller::supplier::supplier_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use sqlx::{Pool, Postgres};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod app;
mod base;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn check_db_connection() -> Result<bool, String> {
    match crate::base::database::postgres::conn::db_pool().await {
        Ok(pool) => {
            match sqlx::query("SELECT 1").execute(pool).await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
        Err(_) => Ok(false),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            check_db_connection,
            app::authentication::controller::users::get_all_users,
            app::authentication::controller::users::get_detail_users_by_id,
            app::authentication::controller::users::create_users,
            app::authentication::controller::users::update_users,
            app::authentication::controller::users::delete_users,
            app::authentication::controller::users::login_user,
            app::authentication::controller::users::reseed_database_users,
            // item
            app::master_data::controller::items::items_get,
            app::master_data::controller::items::get_items_by_id,
            app::master_data::controller::items::items_create,
            app::master_data::controller::items::items_update,
            app::master_data::controller::items::items_delete,
            app::master_data::controller::items::get_items_by_barcode,
            app::master_data::controller::items::get_items_price__by_items_id,
            //
            app::master_data::controller::sale::sale_get_code_txr,
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
            // purchase
            app::master_data::controller::purchase::purchase_get_code,
            app::master_data::controller::purchase::purchase_list,
            app::master_data::controller::purchase::purchase_by_id,
            app::master_data::controller::purchase::purchase_create,
            app::master_data::controller::purchase::purchase_delete,
            // dashboard
            app::master_data::controller::dashboard::dashboard_summary,
            app::master_data::controller::dashboard::dashboard_low_stock,
            app::master_data::controller::dashboard::dashboard_monthly_sales,
            // unit_barang (satuan barang)
            app::master_data::controller::unit_barang::unit_barang_list,
            app::master_data::controller::unit_barang::unit_barang_by_id,
            app::master_data::controller::unit_barang::unit_barang_create,
            app::master_data::controller::unit_barang::unit_barang_update,
            app::master_data::controller::unit_barang::unit_barang_delete,
            // jenis_barang (kategori barang)
            app::master_data::controller::jenis_barang::jenis_barang_list,
            app::master_data::controller::jenis_barang::jenis_barang_by_id,
            app::master_data::controller::jenis_barang::jenis_barang_create,
            app::master_data::controller::jenis_barang::jenis_barang_update,
            app::master_data::controller::jenis_barang::jenis_barang_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

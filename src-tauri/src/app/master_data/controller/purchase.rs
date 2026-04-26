use crate::app::master_data::model::purchase::{Purchase, PurchaseDetail, PurchaseFilter};
use crate::app::master_data::repository::purchase;
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn purchase_get_code() -> Result<String, String> {
    purchase::get_purchase_code().await
}

#[tauri::command]
pub async fn purchase_list(filter: Option<PurchaseFilter>) -> Result<Pagination<Purchase>, String> {
    purchase::get_all_purchases(filter).await
}

#[tauri::command]
pub async fn purchase_by_id(id: i32) -> Result<Responses<PurchaseDetail>, Responses<String>> {
    match purchase::get_purchase_by_id(id).await {
        Ok(res) => Ok(Responses {
            data: Some(res),
            message: Some("get data success".to_string()),
            success: true,
            ..Responses::default()
        }),
        Err(err) => Err(Responses {
            message: Some(err.to_string()),
            success: false,
            ..Responses::default()
        }),
    }
}

#[tauri::command]
pub async fn purchase_create(data: PurchaseDetail) -> Result<Responses<Purchase>, Responses<String>> {
    match purchase::create_purchase(data).await {
        Ok(res) => Ok(Responses {
            data: Some(res),
            message: Some("Berhasil Menambah Pembelian".to_string()),
            success: true,
            ..Responses::default()
        }),
        Err(err) => Err(Responses {
            message: Some(err.to_string()),
            success: false,
            ..Responses::default()
        }),
    }
}

#[tauri::command]
pub async fn purchase_delete(id: i32) -> Result<Responses<String>, Responses<String>> {
    match purchase::delete_purchase(id).await {
        Ok(res) => Ok(Responses {
            data: Some(res),
            message: Some("Berhasil Menghapus Pembelian".to_string()),
            success: true,
            ..Responses::default()
        }),
        Err(err) => Err(Responses {
            message: Some(err.to_string()),
            success: false,
            ..Responses::default()
        }),
    }
}

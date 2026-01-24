use crate::app::master_data::model::consumer::Consumer;
use crate::app::master_data::model::other_struct::Filter;
use crate::app::master_data::model::supplier::Supplier;
use crate::app::master_data::repository::supplier::{
    create_supplier, delete_supplier, get_detail_supplier_by_id, get_list_supplier, update_supplier,
};
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn supplier_list(filter: Option<Filter>) -> Result<Pagination<Supplier>, String> {
    get_list_supplier(filter).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn supplier_by_id(id: i32) -> Result<Responses<Supplier>, Responses<String>> {
    match get_detail_supplier_by_id(id).await {
        Ok(sale) => Ok(Responses {
            data: Some(sale),
            message: Some("Berhasil Get Data".to_string()),
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
pub async fn supplier_create(data: Supplier) -> Result<Responses<Supplier>, Responses<String>> {
    match create_supplier(data).await {
        Ok(sale) => Ok(Responses {
            data: Some(sale),
            message: Some("Berhasil Membuat Data ".to_string()),
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
pub async fn supplier_update(data: Supplier) -> Result<Responses<Supplier>, Responses<String>> {
    match update_supplier(data).await {
        Ok(sale) => Ok(Responses {
            data: Some(sale),
            message: Some("Berhasil Mengubah Data ".to_string()),
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
pub async fn supplier_delete(id: i32) -> Result<Responses<String>, Responses<String>> {
    match delete_supplier(id).await {
        Ok(sale) => Ok(Responses {
            data: Some(sale),
            message: Some("Berhasil Menghapus Data ".to_string()),
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

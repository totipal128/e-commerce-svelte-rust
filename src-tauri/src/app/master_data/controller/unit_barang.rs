use crate::app::master_data::model::unit_barang::UnitBarang;
use crate::app::master_data::repository::unit_barang::{
    create_unit_barang, delete_unit_barang, get_list_unit_barang, get_unit_barang_by_id,
    update_unit_barang,
};
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn unit_barang_list() -> Result<Pagination<UnitBarang>, String> {
    get_list_unit_barang().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn unit_barang_by_id(id: i32) -> Result<Responses<UnitBarang>, Responses<String>> {
    match get_unit_barang_by_id(id).await {
        Ok(data) => Ok(Responses {
            data: Some(data),
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
pub async fn unit_barang_create(data: UnitBarang) -> Result<Responses<UnitBarang>, Responses<String>> {
    match create_unit_barang(data).await {
        Ok(result) => Ok(Responses {
            data: Some(result),
            message: Some("Berhasil membuat satuan barang".to_string()),
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
pub async fn unit_barang_update(data: UnitBarang) -> Result<Responses<UnitBarang>, Responses<String>> {
    match update_unit_barang(data).await {
        Ok(result) => Ok(Responses {
            data: Some(result),
            message: Some("Berhasil mengubah satuan barang".to_string()),
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
pub async fn unit_barang_delete(id: i32) -> Result<Responses<String>, Responses<String>> {
    match delete_unit_barang(id).await {
        Ok(result) => Ok(Responses {
            data: Some(result),
            message: Some("Berhasil menghapus satuan barang".to_string()),
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

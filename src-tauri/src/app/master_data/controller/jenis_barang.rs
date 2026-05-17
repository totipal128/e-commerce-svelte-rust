use crate::app::master_data::model::jenis_barang::JenisBarang;
use crate::app::master_data::repository::jenis_barang::{
    create_jenis_barang, delete_jenis_barang, get_jenis_barang_by_id, get_list_jenis_barang,
    update_jenis_barang,
};
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn jenis_barang_list() -> Result<Pagination<JenisBarang>, String> {
    get_list_jenis_barang().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn jenis_barang_by_id(id: i32) -> Result<Responses<JenisBarang>, Responses<String>> {
    match get_jenis_barang_by_id(id).await {
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
pub async fn jenis_barang_create(data: JenisBarang) -> Result<Responses<JenisBarang>, Responses<String>> {
    match create_jenis_barang(data).await {
        Ok(result) => Ok(Responses {
            data: Some(result),
            message: Some("Berhasil membuat jenis barang".to_string()),
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
pub async fn jenis_barang_update(data: JenisBarang) -> Result<Responses<JenisBarang>, Responses<String>> {
    match update_jenis_barang(data).await {
        Ok(result) => Ok(Responses {
            data: Some(result),
            message: Some("Berhasil mengubah jenis barang".to_string()),
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
pub async fn jenis_barang_delete(id: i32) -> Result<Responses<String>, Responses<String>> {
    match delete_jenis_barang(id).await {
        Ok(result) => Ok(Responses {
            data: Some(result),
            message: Some("Berhasil menghapus jenis barang".to_string()),
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

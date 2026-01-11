use crate::app::master_data::controller::sale;
use crate::app::master_data::model::other_struct::Filter;
use crate::app::master_data::model::sale::{Sale, SaleDetail};

use crate::app::master_data::repository::sale::{
    create_sale, delete_sale, get_detail_sale_by_id, get_list_sale, update_sale,
};
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn sale_list(filter: Option<Filter>) -> Result<Pagination<Sale>, String> {
    get_list_sale(filter).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sale_by_id(id: i32) -> Result<Responses<SaleDetail>, Responses<String>> {
    match get_detail_sale_by_id(id).await {
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
pub async fn sale_create(data: SaleDetail) -> Result<Responses<Sale>, Responses<String>> {
    match create_sale(data).await {
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
pub async fn sale_update(data: SaleDetail) -> Result<Responses<Sale>, Responses<String>> {
    match update_sale(data).await {
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
pub async fn sale_delete(id: i32) -> Result<Responses<String>, Responses<String>> {
    match delete_sale(id).await {
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

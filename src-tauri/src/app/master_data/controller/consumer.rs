use crate::app::master_data::model::consumer::Consumer;
use crate::app::master_data::model::other_struct::Filter;

use crate::app::master_data::repository::consumer::{
    create_consumer, delete_consumer, get_detail_consumer_by_id, get_list_consumer, update_consumer,
};
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn consumer_list(filter: Option<Filter>) -> Result<Pagination<Consumer>, String> {
    get_list_consumer(filter).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn consumer_by_id(id: i32) -> Result<Responses<Consumer>, Responses<String>> {
    match get_detail_consumer_by_id(id).await {
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
pub async fn consumer_create(data: Consumer) -> Result<Responses<Consumer>, Responses<String>> {
    match create_consumer(data).await {
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
pub async fn consumer_update(data: Consumer) -> Result<Responses<Consumer>, Responses<String>> {
    match update_consumer(data).await {
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
pub async fn consumer_delete(id: i32) -> Result<Responses<String>, Responses<String>> {
    match delete_consumer(id).await {
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

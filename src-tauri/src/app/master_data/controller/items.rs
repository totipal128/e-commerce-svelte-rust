use crate::app::master_data::model::item_price::ItemPriceFind;
use crate::app::master_data::model::items::{
    ItemPrice, Items, ItemsCreate, ItemsDetail, ItemsFilter,
};
use crate::app::master_data::repository::items_repo;
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn items_get(filter: Option<ItemsFilter>) -> Result<Pagination<Items>, String> {
    items_repo::get_all_items(filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_items_by_id(id: i32) -> Result<Responses<ItemsDetail>, Responses<String>> {
    match items_repo::get_by_id(id).await {
        Ok(item) => Ok(Responses {
            data: Some(item),
            message: Some("get data".to_string()),
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
pub async fn items_create(data: ItemsCreate) -> Result<Responses<Items>, Responses<String>> {
    match items_repo::create_item(data).await {
        Ok(item) => Ok(Responses {
            data: Some(item),
            message: Some("Berhasil Membuat Barang".to_string()),
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
pub async fn items_update(data: ItemsDetail) -> Result<Responses<ItemsDetail>, Responses<Items>> {
    match items_repo::update_item(data).await {
        Ok(item) => Ok(Responses {
            data: Some(item),
            message: Some("Berhasil Mengubah Barang".to_string()),
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
pub async fn items_delete(id: i32) -> Result<Responses<Items>, Responses<Items>> {
    match items_repo::delete_item(id).await {
        Ok(item) => Ok(Responses {
            message: Some("Berhasil Menghapus Barang".to_string()),
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
pub async fn get_items_by_barcode(
    barcode: String,
) -> Result<Responses<ItemPriceFind>, Responses<String>> {
    match items_repo::get_by_barcode(barcode).await {
        Ok(item) => Ok(Responses {
            data: Some(item),
            message: Some("get data".to_string()),
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
pub async fn get_items_price__by_items_id(
    item_id: i32,
) -> Result<Responses<Vec<ItemPriceFind>>, Responses<String>> {
    match items_repo::get_by_items_id(item_id).await {
        Ok(item) => Ok(Responses {
            data: Some(item),
            message: Some("get data".to_string()),
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

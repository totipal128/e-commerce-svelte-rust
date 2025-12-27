use crate::app::master_data::model::items::{Items, ItemsFilter};
use crate::app::master_data::repository::items_repo;
use crate::base::database::postgres::orm::Pagination;

#[tauri::command]
pub async fn items_get(filter: Option<ItemsFilter>) -> Result<Pagination<Items>, String> {
    items_repo::get_all_items(filter)
        .await
        .map_err(|e| e.to_string())
}

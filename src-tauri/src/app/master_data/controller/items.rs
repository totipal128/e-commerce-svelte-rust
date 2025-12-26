use crate::app::master_data::model::items::{ItemsFilter, PaginationItems};
use crate::app::master_data::repository::items_repo;

#[tauri::command]
pub async fn items_get(filter: Option<ItemsFilter>) -> Result<PaginationItems, String> {
    items_repo::get_all_items(filter)
        .await
        .map_err(|e| e.to_string())
}

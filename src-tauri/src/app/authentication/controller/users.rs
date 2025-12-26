use crate::app::authentication::model::users::{PaginationUser, UserFilter};
use crate::app::authentication::repository::users_repo;

#[tauri::command]
pub async fn get_all_users(filter: Option<UserFilter>) -> Result<PaginationUser, String> {
    users_repo::get_all_user(filter)
        .await
        .map_err(|e| e.to_string())
}

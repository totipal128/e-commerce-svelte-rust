use crate::app::authentication::model::users::{User, UserFilter, UserNoPass};
use crate::app::authentication::repository::users_repo;
use crate::base::database::postgres::orm::Pagination;
use crate::base::responses::responses_struct::Responses;

#[tauri::command]
pub async fn get_all_users(filter: Option<UserFilter>) -> Result<Pagination<UserNoPass>, String> {
    users_repo::get_all(filter).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_detail_users_by_id(id: i32) -> Result<Responses<UserNoPass>, Responses<String>> {
    match users_repo::get_by_id(id).await.map_err(|e| e.to_string()) {
        Ok(user) => Ok(Responses {
            data: Some(user),
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
pub async fn create_users(data: User) -> Result<User, String> {
    users_repo::create(data).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_users(data: User) -> Result<User, String> {
    users_repo::update(data).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_users(id: i32) -> Result<String, String> {
    users_repo::delete(id).await.map_err(|e| e.to_string())
}

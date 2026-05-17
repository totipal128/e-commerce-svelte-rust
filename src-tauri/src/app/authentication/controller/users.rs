use crate::app::authentication::model::users::{User, UserFilter, UserNoPass, LoginRequest, LoginResponse};
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

#[tauri::command]
pub async fn login_user(data: LoginRequest) -> Result<Responses<LoginResponse>, Responses<String>> {
    match users_repo::login(data).await {
        Ok(res) => Ok(Responses {
            data: Some(res),
            message: Some("Login berhasil".to_string()),
            success: true,
            ..Responses::default()
        }),
        Err(err) => Err(Responses {
            message: Some(err),
            success: false,
            ..Responses::default()
        }),
    }
}

#[tauri::command]
pub async fn reseed_database_users() -> Result<Responses<String>, Responses<String>> {
    match crate::base::database::postgres::conn::db_pool().await {
        Ok(pool) => {
            // Delete standard seeded usernames first to force full reload
            match sqlx::query("DELETE FROM users WHERE username IN ('admin', 'kasir', 'gudang')").execute(pool).await {
                Ok(_) => {
                    match crate::base::database::postgres::conn::seed_db(pool).await {
                        Ok(_) => Ok(Responses {
                            message: Some("Berhasil memuat ulang data users default".to_string()),
                            success: true,
                            ..Responses::default()
                        }),
                        Err(e) => Err(Responses {
                            message: Some(format!("Gagal memuat ulang database: {}", e)),
                            success: false,
                            ..Responses::default()
                        })
                    }
                }
                Err(e) => Err(Responses {
                    message: Some(format!("Gagal membersihkan data lama: {}", e)),
                    success: false,
                    ..Responses::default()
                })
            }
        }
        Err(e) => Err(Responses {
            message: Some(format!("Koneksi database terputus: {}", e)),
            success: false,
            ..Responses::default()
        })
    }
}

use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct PaginationUser {
    pub is_next: Option<bool>,
    pub is_previous: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub total_page: Option<i64>,
    pub count: Option<i64>,
    pub result: Option<Vec<UserNoPass>>,
}
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub name: Option<String>,
    pub address: Option<String>,
    pub no_handphone: Option<String>,
    pub barcode: Option<String>,

    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct UserName {
    pub username: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UserNoPass {
    pub id: i32,
    pub username: String,
    pub email: String,

    pub name: Option<String>,
    pub address: Option<String>,
    pub no_handphone: Option<String>,
    pub barcode: Option<String>,

    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub total: Option<i64>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UserFilter {
    pub id: Option<i32>,
    pub search: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,

    pub name: Option<String>,
    pub address: Option<String>,
    pub no_handphone: Option<String>,
    pub barcode: Option<String>,

    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

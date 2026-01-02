use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,

    #[serde(skip_serializing)]
    pub password: Option<String>,

    pub name: Option<String>,
    pub address: Option<String>,
    pub no_handphone: Option<String>,
    pub barcode: Option<String>,
}
impl Model for User {
    const TABLE: &'static str = "users";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct UserName {
    pub username: String,
}

#[derive(sqlx::FromRow, Clone, Default, Debug, Serialize, Deserialize)]
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
}

impl Model for UserNoPass {
    const TABLE: &'static str = "users";
    const FIELDS_INSERT: &'static [&'static str] = &[];
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

    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

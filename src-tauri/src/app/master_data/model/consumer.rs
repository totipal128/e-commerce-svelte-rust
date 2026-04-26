use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Clone, serde::Serialize, Default, Deserialize, FromRow)]
pub struct Consumer {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub no_hp: Option<String>,
    // Newly added to link with 'users' table
    #[sqlx(default)]
    pub user_id: Option<i32>,
    
    // Only used to accept credentials on create/update
    #[sqlx(default)]
    pub username: Option<String>,
    #[sqlx(default)]
    pub password: Option<String>,

    pub created_by_id: Option<i32>,
    pub created_at: Option<DateTime<Local>>,
}
impl Model for Consumer {
    const TABLE: &'static str = "customer";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Clone, serde::Serialize, Default, Deserialize, FromRow)]
pub struct Supplier {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub no_hp: Option<String>,
    pub created_by_id: Option<i32>,
    pub created_at: Option<DateTime<Local>>,
}
impl Model for Supplier {
    const TABLE: &'static str = "suppliers";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

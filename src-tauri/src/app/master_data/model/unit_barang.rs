use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct UnitBarang {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Local>>,
}

impl Model for UnitBarang {
    const TABLE: &'static str = "unit_barang";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

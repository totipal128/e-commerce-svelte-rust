use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct JenisBarang {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Local>>,
}

impl Model for JenisBarang {
    const TABLE: &'static str = "jenis_barang";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

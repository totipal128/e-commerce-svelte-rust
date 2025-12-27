use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Default)]
pub struct PaginationItems {
    pub is_next: Option<bool>,
    pub is_previous: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub total_page: Option<i64>,
    pub count: Option<i64>,
    pub result: Option<Vec<Items>>,
}

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct Items {
    pub id: i32,
    pub barcode: String,
    pub name: String,
    pub items_type_id: Option<i32>,
    pub created_by: Option<i32>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub price: serde_json::Value,
    pub total: Option<i64>,
}
impl Model for Items {
    const TABLE: &'static str = "users";
    const FIELDS: &'static [&'static str] = &[stringify!(name), stringify!(email)];
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ItemPrice {
    id: i32,
    item_id: i32,
    barcode: String,
    type_item: String,
    price_buy: f64,
    price_sell: f64,
    parent_id: Option<i32>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ItemsFilter {
    pub id: Option<i32>,
    pub search: Option<String>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub type_item: Option<String>,
    pub items_type_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsCreate {
    pub id: i32,
    pub barcode: String,
    pub name: String,
    pub items_type_id: Option<i32>,
    pub created_by: Option<i32>,
    pub price: Option<Vec<ItemPrice>>,
}

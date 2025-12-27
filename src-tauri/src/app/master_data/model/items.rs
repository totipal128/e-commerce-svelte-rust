use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct Items {
    pub id: i32,
    pub barcode: String,
    pub name: String,
    pub items_category_id: Option<i32>,
    pub type_unit: Option<i32>,
    pub qty_stock: DateTime<Local>,
    pub created_by: DateTime<Local>,
    pub created_at: Option<DateTime<Local>>,
    pub price: serde_json::Value,
}
impl Model for Items {
    const TABLE: &'static str = "items";
    const FIELDS_INSERT: &'static [&'static str] = &[
        stringify!(barcode),
        stringify!(name),
        stringify!(items_category_id),
        stringify!(type_unit),
        stringify!(qty_stock),
    ];
}

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
struct ItemPrice {
    id: i32,
    item_id: i32,
    barcode: String,
    type_item: String,
    price_buy: f64,
    price_sell: f64,
    parent_id: Option<i32>,
}
impl Model for ItemPrice {
    const TABLE: &'static str = "items";
    const FIELDS_INSERT: &'static [&'static str] = &[
        stringify!(barcode),
        stringify!(name),
        stringify!(items_category_id),
        stringify!(type_unit),
        stringify!(qty_stock),
    ];
}

#[derive(Debug, Deserialize, Default)]
pub struct ItemsFilter {
    pub id: Option<i32>,
    pub search: Option<String>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub type_item: Option<String>,
    pub items_type_id: Option<i32>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct Items {
    pub id: Option<i32>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub type_unit: Option<String>,
    pub items_category_id: Option<i32>,
    pub qty_stock: Option<i32>,
    pub created_by: Option<DateTime<Local>>,
    pub created_at: Option<DateTime<Local>>,
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

// Create
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct ItemsCreate {
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub type_unit: Option<String>,
    pub items_category_id: Option<i32>,
    pub qty_stock: Option<i32>,
    pub price: Option<Vec<ItemPrice>>,
}

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
struct ItemPrice {
    id: Option<i32>,
    item_id: Option<i32>,
    barcode: Option<String>,
    type_item: Option<String>,
    price_buy: Option<f64>,
    price_sell: Option<f64>,
    content: Option<i32>,
    parent_id: Option<i32>,
}

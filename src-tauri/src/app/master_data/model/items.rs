use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct ItemsList {
    pub id: Option<i32>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub type_unit: Option<String>,
    pub items_category_id: Option<i32>,
    pub qty_stock: Option<i32>,
    pub price_buy: Option<f64>,
    pub price_sell: Option<f64>,
    pub created_by: Option<DateTime<Local>>,
    pub created_at: Option<DateTime<Local>>,
}
impl Model for ItemsList {
    const TABLE: &'static str = "items";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}
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
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct ItemsDetail {
    pub id: Option<i32>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub type_unit: Option<String>,
    pub items_category_id: Option<i32>,
    pub qty_stock: Option<i32>,
    pub created_by: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,

    #[sqlx(skip)]
    pub price: Option<Vec<ItemPrice>>,
}
impl Model for ItemsDetail {
    const TABLE: &'static str = "items";
    const FIELDS_INSERT: &'static [&'static str] = &[
        stringify!(barcode),
        stringify!(name),
        stringify!(items_category_id),
        stringify!(type_unit),
        stringify!(qty_stock),
    ];
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
pub struct ItemPrice {
    pub id: Option<i32>,
    pub item_id: Option<i32>,
    pub barcode: Option<String>,
    pub type_unit: Option<String>,
    pub parent_type_unit: Option<String>,
    pub price_buy: Option<f64>,
    pub price_sell: Option<f64>,
    pub parent_id: Option<i32>,
    pub content: Option<i32>,
    pub seq: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Model for ItemPrice {
    const TABLE: &'static str = "items_price";
    const FIELDS_INSERT: &'static [&'static str] = &[
        stringify!(item_id),
        stringify!(barcode),
        stringify!(type_unit),
        stringify!(parent_type_unit),
        stringify!(price_buy),
        stringify!(price_sell),
        stringify!(parent_id),
        stringify!(content),
        stringify!(seq),
    ];
}

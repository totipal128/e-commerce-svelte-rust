use crate::base::database::postgres::orm::Model;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// purchase
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct Purchase {
    pub id: Option<i32>,
    pub code: Option<String>,
    pub supplier_id: Option<i32>,
    #[sqlx(default)]
    pub supplier_name: Option<String>,
    pub ppn: Option<f64>,
    pub discount: Option<f64>,
    pub total_item: Option<i32>,
    pub total: Option<f64>,
    pub payment: Option<f64>,
    pub created_by_id: Option<i32>,
    pub created_at: Option<DateTime<Local>>,
}

impl Model for Purchase {
    const TABLE: &'static str = "purchase";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct PurchaseDetail {
    pub id: Option<i32>,
    pub code: Option<String>,
    pub supplier_id: Option<i32>,
    pub ppn: Option<f64>,
    pub discount: Option<f64>,
    pub total_item: Option<i32>,
    pub total: Option<f64>,
    pub payment: Option<f64>,
    pub created_by_id: Option<i32>,
    #[sqlx(skip)]
    pub items: Option<Vec<PurchaseItem>>,
}

impl Model for PurchaseDetail {
    const TABLE: &'static str = "purchase";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

// purchase_item
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct PurchaseItem {
    pub id: Option<i32>,
    #[sqlx(default)]
    pub code: Option<String>,
    pub purchase_id: Option<i32>,
    pub items_id: Option<i32>,
    pub items_name: Option<String>,
    pub items_unit: Option<String>,
    pub items_price: Option<f64>,
    pub total: Option<f64>,
    pub qty: Option<i32>,
}

impl Model for PurchaseItem {
    const TABLE: &'static str = "purchase_items";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

#[derive(Debug, Deserialize, Default)]
pub struct PurchaseFilter {
    pub id: Option<i32>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

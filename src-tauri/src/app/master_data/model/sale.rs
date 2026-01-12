use crate::base::database::postgres::orm::Model;
use chrono::format::Item;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// sale
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct Sale {
    pub id: Option<i32>,
    pub code: Option<String>,
    pub customer_id: Option<i32>,
    pub ppn: Option<f64>,
    pub discount: Option<f64>,
    pub total_item: Option<i32>,
    pub total: Option<f64>,
    pub change: Option<f64>,
    pub payment: Option<f64>,
    pub created_by_id: Option<i32>,
}

impl Model for Sale {
    const TABLE: &'static str = "sale";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct SaleDetail {
    pub id: Option<i32>,
    pub code: Option<String>,

    pub customer_id: Option<i32>,

    pub ppn: Option<f64>,
    pub discount: Option<f64>,
    pub total_item: Option<i32>,
    pub total: Option<f64>,
    pub change: Option<f64>,
    pub payment: Option<f64>,

    pub created_by_id: Option<i32>,

    #[sqlx(skip)]
    pub items: Option<Vec<SaleItem>>,
}

impl Model for SaleDetail {
    const TABLE: &'static str = "sale";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

// sale_item
#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct SaleItem {
    pub id: Option<i32>,
    pub sale_id: Option<i32>,
    pub items_id: Option<i32>,
    pub items_name: Option<String>,
    pub items_unit: Option<String>,
    pub items_price: Option<f64>,
    pub total: Option<f64>,
    pub qty: Option<i32>,
}

impl Model for SaleItem {
    const TABLE: &'static str = "sale_items";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

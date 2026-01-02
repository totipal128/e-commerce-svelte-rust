use crate::base::database::postgres::orm::Model;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct ItemPriceFind {
    pub id: Option<i32>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub items_category_id: Option<i32>,
    pub type_unit: Option<String>,
    pub price_buy: Option<f64>,
    pub price_sell: Option<f64>,
    pub qty: Option<i32>,
}

impl Model for ItemPriceFind {
    const TABLE: &'static str = "items";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

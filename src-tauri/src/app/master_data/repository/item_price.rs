use crate::app::master_data::model::items::{ItemPrice, Items, ItemsFilter};
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};
pub async fn get_items_price_by_item_id(mut item_id: i32) -> Result<Vec<ItemPrice>, String> {
    let mut qs = QueryBuilderPostgrest::<ItemPrice>::new();

    qs = qs
        .where_clause_i32("item_id", item_id)
        .order("created_at asc");

    Ok(qs.find_all().await.map_err(|e| e.to_string())?)
}

pub async fn delete_item_price__by_item_id(item_id: i32) -> Result<String, String> {
    let result = QueryBuilderPostgrest::<Items>::new()
        .where_clause_i32("item_id", item_id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn create_items_price(mut r: &ItemPrice, seq: usize) -> Result<ItemPrice, String> {
    let mut r_p = QueryBuilderPostgrest::<ItemPrice>::new();

    if r.item_id.is_some() {
        r_p = r_p.insert_i32("item_id", r.item_id.unwrap());
    }
    if r.barcode.is_some() {
        r_p = r_p.insert_str("barcode", r.barcode.as_ref().unwrap());
    }
    if r.type_unit.is_some() {
        r_p = r_p.insert_str("type_unit", r.type_unit.as_ref().unwrap());
    }
    if r.parent_type_unit.is_some() {
        r_p = r_p.insert_str("parent_type_unit", r.parent_type_unit.as_ref().unwrap());
    }
    if r.price_buy.is_some() {
        r_p = r_p.insert_f64("price_buy", r.price_buy.unwrap());
    }
    if r.price_sell.is_some() {
        r_p = r_p.insert_f64("price_sell", r.price_sell.unwrap());
    }
    if r.parent_id.is_some() {
        r_p = r_p.insert_i32("parent_id", r.parent_id.unwrap());
    }
    if r.content.is_some() {
        r_p = r_p.insert_i32("content", r.content.unwrap());
    }
    if seq > 0 {
        r_p = r_p.insert_usize("seq", seq);
    }

    Ok(r_p.create().await.map_err(|e| e.to_string())?)
}

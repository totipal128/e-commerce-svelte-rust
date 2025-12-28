use crate::app::master_data::model::items::{Items, ItemsCreate, ItemsFilter};
use crate::base::database::postgres::conn::db_pool;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};
use sqlx::Postgres;
use std::any::type_name;

pub async fn get_all_items(mut filter: Option<ItemsFilter>) -> Result<Pagination<Items>, String> {
    let mut qs = QueryBuilderPostgrest::<Items>::new();

    let (page, page_size) = match filter.as_mut() {
        Some(f) => (f.page.unwrap_or(1), f.page_size.unwrap_or(10)),
        None => (1, 10),
    };

    if filter.is_some() {
        if filter.as_mut().unwrap().search.is_some() {
            qs = qs.ilike("name", filter.unwrap().search.unwrap().as_str())
        }
    }

    Ok(qs
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?)
}
pub async fn get_by_id(id: i32) -> Result<Items, String> {
    let mut qs = QueryBuilderPostgrest::<Items>::new();

    Ok(qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?)
}

pub async fn create_item(item: ItemsCreate) -> Result<Items, String> {
    let mut qs = QueryBuilderPostgrest::<Items>::new();

    if item.barcode.is_some() {
        qs = qs.insert_str("barcode", item.barcode.as_ref().unwrap());
    }
    if item.name.is_some() {
        qs = qs.insert_str("name", item.name.as_ref().unwrap());
    }
    if item.type_unit.is_some() {
        qs = qs.insert_str("type_unit", item.type_unit.as_ref().unwrap());
    }
    if item.items_category_id.is_some() {
        qs = qs.insert_i32("items_category_id", item.items_category_id.unwrap())
    }
    if item.qty_stock.is_some() {
        qs = qs.insert_i32("qty_stock", item.qty_stock.unwrap())
    }

    let result = qs.create().await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn update_item(item: Items) -> Result<Items, String> {
    if !item.id.is_some() {
        return Err(String::from("Item ID missing"));
    }

    let mut qs = QueryBuilderPostgrest::<Items>::new();

    if item.barcode.is_some() {
        qs = qs.set_one("barcode", item.barcode.as_ref().unwrap(), Some("str"));
    }
    if item.name.is_some() {
        qs = qs.set_one("name", item.name.as_ref().unwrap(), Some("str"));
    }
    if item.type_unit.is_some() {
        qs = qs.set_one("type_unit", item.type_unit.as_ref().unwrap(), Some("str"));
    }
    if item.items_category_id.is_some() {
        qs = qs.set_one(
            "items_category_id",
            item.items_category_id.unwrap().to_string().as_str(),
            Some("int"),
        );
    }
    if item.qty_stock.is_some() {
        qs = qs.set_one(
            "qty_stock",
            item.qty_stock.unwrap().to_string().as_str(),
            Some("int"),
        )
    }

    let result = qs
        .update(format!("id={}", item.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn delete_item(id: i32) -> Result<String, String> {
    let result = QueryBuilderPostgrest::<Items>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

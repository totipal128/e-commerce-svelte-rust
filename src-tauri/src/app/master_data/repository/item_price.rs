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

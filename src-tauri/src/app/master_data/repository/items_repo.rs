use crate::app::master_data::model::items::{Items, ItemsFilter};
use crate::base::database::postgres::conn::db_pool;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};
use sqlx::Postgres;

pub async fn get_all_items(filter: Option<ItemsFilter>) -> Result<Pagination<Items>, String> {
    let mut qs = QueryBuilderPostgrest::<Items>::new();

    let (page, page_size) = match filter {
        Some(f) => (f.page.unwrap_or(1), f.page_size.unwrap_or(10)),
        None => (1, 10),
    };

    Ok(qs
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?)
}

async fn create_item(item: Items) -> Result<Items, String> {
    let qs = QueryBuilderPostgrest::<Items>::new()
        .values(vec![
            stringify!(item.barcode),
            stringify!(item.name),
            stringify!(item.items_category_id),
            stringify!(item.type_unit),
            stringify!(item.qty_stock),
        ])
        .create()
        .await
        .map_err(|e| e.to_string())?;

    Ok(qs)
}

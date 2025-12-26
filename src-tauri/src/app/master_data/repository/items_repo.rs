use crate::app::master_data::model::items::{Items, ItemsCreate, ItemsFilter, PaginationItems};
use crate::base::database::postgres::conn::db_pool;
use sqlx::Postgres;

pub async fn get_all_items(filter: Option<ItemsFilter>) -> Result<PaginationItems, String> {
    let pool = db_pool().await.map_err(|e| e.to_string())?;

    let mut query_set = sqlx::QueryBuilder::<Postgres>::new(
        "select
                    items.*,
                    COUNT(items.*) OVER() as total,
                    json_agg(
                to_json(ip.*)
                ) as price
                from
                    items
                left join items_price ip on
                    ip.item_id = items.id
                    ",
    );

    if let Some(filter) = filter {
        let mut has_where = false;

        if let Some(search) = filter.search {
            query_set.push(if !has_where { " WHERE" } else { " AND" });
            query_set.push(" items.name ILIKE ");
            query_set.push_bind(format!("%{}%", search));
            has_where = true;
        }
    }
    query_set.push(" group by items.id ");

    let data = query_set
        .build_query_as::<Items>()
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    let count = data.first().map(|u| u.total).unwrap_or(None);

    Ok(PaginationItems {
        result: Some(data),
        count,
        ..PaginationItems::default()
    })
}

// async fn create_item(item: ItemsCreate) -> Result<ItemsCreate, String> {
//     let mut pool = conn_postgrest().await.map_err(|e| e.to_string())?;
//
//     let mut inserted_item = sqlx::query_as(
//         "INSERT INTO items (barcode, name, items_type_id, created_by)
//          VALUES ($1, $2, $3, $4) RETURNING id",
//     )
//     .bind(&item.barcode)
//     .bind(&item.name)
//     .bind(item.items_type_id)
//     .bind(item.created_by)
//     .fetch_one(&mut pool)
//     .await?;
//
//     let item_id = inserted_item.0;
//
//     Ok(qs)
// }

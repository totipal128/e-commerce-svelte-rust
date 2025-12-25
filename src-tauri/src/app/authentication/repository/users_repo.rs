use crate::app::authentication::model::users::{UserFilter, UserNoPass};
use crate::conn_postgrest;
use sqlx::Postgres;
pub async fn get_all_user(filter: Option<UserFilter>) -> Result<Vec<UserNoPass>, sqlx::Error> {
    // koneksike databases
    let pool = conn_postgrest()
        .await
        .expect("Failed to connect to Postgres");

    // query filter
    let mut query_set = sqlx::QueryBuilder::<Postgres>::new("select * from users");

    // filter
    if let Some(filter) = filter {
        let mut has_where = false;

        if filter.search.is_some() {
            query_set.push(if !has_where { " WHERE" } else { " AND" });
            query_set.push(" username ilike ");
            query_set.push_bind(format!("%{}%", filter.search.unwrap()));
            has_where = true;
        }
    } else {
        // order
        query_set.push(" ORDER BY id desc");
    }

    let user = query_set
        .build_query_as::<UserNoPass>()
        .fetch_all(&pool)
        .await?;

    return Ok(user);
}

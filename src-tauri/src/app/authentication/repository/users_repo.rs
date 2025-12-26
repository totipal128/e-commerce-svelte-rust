use crate::app::authentication::model::users::{PaginationUser, UserFilter, UserNoPass};
use crate::base::database::postgres::conn::db_pool;
use sqlx::Postgres;

pub async fn get_all_user(filter: Option<UserFilter>) -> Result<PaginationUser, String> {
    let pool = db_pool().await.map_err(|e| e.to_string())?;

    let mut query_set =
        sqlx::QueryBuilder::<Postgres>::new("SELECT *, COUNT(*) OVER() AS total FROM users");

    if let Some(filter) = filter {
        let mut has_where = false;

        if let Some(search) = filter.search {
            query_set.push(if !has_where { " WHERE" } else { " AND" });
            query_set.push(" username ILIKE ");
            query_set.push_bind(format!("%{}%", search));
            query_set.push(" OR email ILIKE ");
            query_set.push_bind(format!("%{}%", search));
            query_set.push(" OR name ILIKE ");
            query_set.push_bind(format!("%{}%", search));
            has_where = true;
        }
    }

    let users = query_set
        .build_query_as::<UserNoPass>()
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    let count = users.first().map(|u| u.total).unwrap_or(None);

    Ok(PaginationUser {
        result: Some(users),
        count,
        ..PaginationUser::default()
    })
}

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
pub async fn postgrest_conn() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::path::Path;
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();
pub async fn db_pool() -> Result<&'static Pool<Postgres>, sqlx::Error> {
    DB_POOL
        .get_or_try_init(|| async {
            dotenvy::dotenv().ok();
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

            PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
        })
        .await
}

pub async fn migrate_db() -> Result<(), sqlx::Error> {
    let pool = db_pool().await?;
    let migaration_url = format!("../migrations");
    let migrator = Migrator::new(Path::new(&migaration_url)).await?;

    migrator.run(pool).await?;
    println!("migrated successfully ...");
    Ok(())
}

pub async fn close_db() {
    if let Some(pool) = DB_POOL.get() {
        pool.close().await;
    }
}

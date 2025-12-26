use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::path::Path;

pub async fn postgrest_conn() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn migrate_db() -> Result<(), sqlx::Error> {
    let pool = postgrest_conn().await?;
    let migaration_url = format!("../migrations");
    let migrator = Migrator::new(Path::new(&migaration_url)).await?;

    migrator.run(&pool).await?;
    println!("migrated successfully ...");
    Ok(())
}

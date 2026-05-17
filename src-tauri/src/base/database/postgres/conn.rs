use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::path::Path;
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn seed_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // 1. Seed Roles
    let roles = vec![
        ("Admin", "admin"),
        ("Kasir", "kasir"),
        ("Gudang", "gudang")
    ];
    for (name, code) in roles {
        sqlx::query("INSERT INTO role (name, code_name) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING")
            .bind(name)
            .bind(code)
            .execute(pool)
            .await?;
    }

    // 2. Define users to seed (username, email, password, full_name, role_code)
    let users_to_seed = vec![
        ("admin", "admin@pospro.com", "admin123", "Administrator", "admin"),
        ("kasir", "kasir@pospro.com", "kasir123", "Kasir Utama", "kasir"),
        ("gudang", "gudang@pospro.com", "gudang123", "Staf Gudang", "gudang"),
    ];

    for (username, email, password, name, role_code) in users_to_seed {
        let user_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(username)
            .fetch_one(pool)
            .await?;

        if !user_exists {
            let hashed_pw = match crate::app::authentication::services::password::hash_password(password.to_string()) {
                Ok(h) => h,
                Err(e) => return Err(sqlx::Error::Protocol(e.to_string())),
            };

            let user_id: i32 = sqlx::query_scalar(
                "INSERT INTO users (username, email, password, name, address, no_handphone) \
                 VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
            )
            .bind(username)
            .bind(email)
            .bind(hashed_pw)
            .bind(name)
            .bind("POS PRO Headquarters")
            .bind("08123456789")
            .fetch_one(pool)
            .await?;

            // Link with the specified role
            let role_id: i32 = sqlx::query_scalar("SELECT id FROM role WHERE code_name = $1")
                .bind(role_code)
                .fetch_one(pool)
                .await?;

            sqlx::query("INSERT INTO users_role (users_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(user_id)
                .bind(role_id)
                .execute(pool)
                .await?;
            
            println!("Default user '{}' with role '{}' seeded successfully!", username, role_code);
        }
    }

    // 3. Seed Example Items
    if let Err(e) = crate::base::database::postgres::items_seeder::seed_items(pool).await {
        println!("Error seeding default items: {}", e);
    }

    Ok(())
}

pub async fn db_pool() -> Result<&'static Pool<Postgres>, sqlx::Error> {
    DB_POOL
        .get_or_try_init(|| async {
            dotenvy::dotenv().ok();
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            println!("database_url: {}", database_url);

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await?;

            // Seed roles and admin
            if let Err(e) = seed_db(&pool).await {
                println!("Error seeding database: {}", e);
            }

            Ok(pool)
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

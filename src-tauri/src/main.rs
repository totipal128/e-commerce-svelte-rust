// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::{Pool, Postgres};

fn main() {
    e_commerce_lib::run()
}

// mod tests {
//     use crate::base;
//     use crate::base::database::postgres::conn;
//     use crate::base::database::postgres::conn::{postgrest_conn, PostgresSqlConnection};
//
//     #[tokio::test]
//     async fn example() {
//         let result = crate::base::database::postgres::conn::postgrest_conn().await;
//
//         // Pastikan result sukses
//         let pool = result.expect("Gagal mendapatkan pool");
//
//         // Test ping ke database
//         let check = sqlx::query("SELECT 1").execute(&pool).await;
//     }
// }

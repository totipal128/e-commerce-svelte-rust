use crate::base::database::postgres::conn::db_pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_sale_today: f64,
    pub total_sale_month: f64,
    pub total_purchase_month: f64,
    pub profit_month: f64,
    pub total_transactions_today: i64,
    pub total_transactions_month: i64,
    pub total_items: i64,
    pub low_stock_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LowStockItem {
    pub id: i32,
    pub name: String,
    pub barcode: Option<String>,
    pub qty_stock: i32,
    pub type_unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlySale {
    pub month: String,
    pub total: f64,
    pub transactions: i64,
}

pub async fn get_dashboard_summary() -> Result<DashboardSummary, String> {
    let pool = db_pool().await.map_err(|e| e.to_string())?;

    // Total penjualan hari ini
    let total_sale_today: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total), 0) FROM sale WHERE DATE(created_at) = CURRENT_DATE AND deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Total penjualan bulan ini
    let total_sale_month: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total), 0) FROM sale WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Total pembelian bulan ini
    let total_purchase_month: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total), 0) FROM purchase WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Jumlah transaksi hari ini
    let total_transactions_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sale WHERE DATE(created_at) = CURRENT_DATE AND deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Jumlah transaksi bulan ini
    let total_transactions_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sale WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Total jenis barang
    let total_items: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM items WHERE deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Jumlah barang dengan stok menipis (< 10)
    let low_stock_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM items WHERE qty_stock < 10 AND deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    let profit_month = total_sale_month - total_purchase_month;

    Ok(DashboardSummary {
        total_sale_today,
        total_sale_month,
        total_purchase_month,
        profit_month,
        total_transactions_today,
        total_transactions_month,
        total_items,
        low_stock_count,
    })
}

pub async fn get_low_stock_items() -> Result<Vec<LowStockItem>, String> {
    let pool = db_pool().await.map_err(|e| e.to_string())?;

    let rows: Vec<(i32, String, Option<String>, i32, Option<String>)> = sqlx::query_as(
        "SELECT id, name, barcode, qty_stock, type_unit FROM items WHERE qty_stock < 10 AND deleted_at IS NULL ORDER BY qty_stock ASC LIMIT 20"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(|(id, name, barcode, qty_stock, type_unit)| LowStockItem {
        id, name, barcode, qty_stock, type_unit
    }).collect())
}

pub async fn get_monthly_sales(months: i32) -> Result<Vec<MonthlySale>, String> {
    let pool = db_pool().await.map_err(|e| e.to_string())?;

    let rows: Vec<(String, f64, i64)> = sqlx::query_as(
        &format!(
            "SELECT TO_CHAR(DATE_TRUNC('month', created_at), 'Mon YYYY') as month,
             COALESCE(SUM(total), 0) as total,
             COUNT(*) as transactions
             FROM sale
             WHERE created_at >= DATE_TRUNC('month', CURRENT_DATE) - INTERVAL '{} months'
             AND deleted_at IS NULL
             GROUP BY DATE_TRUNC('month', created_at)
             ORDER BY DATE_TRUNC('month', created_at) ASC",
            months - 1
        )
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(|(month, total, transactions)| MonthlySale {
        month, total, transactions
    }).collect())
}

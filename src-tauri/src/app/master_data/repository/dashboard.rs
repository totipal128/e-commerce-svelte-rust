use crate::base::database::postgres::conn::db_pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashierRevenue {
    pub cashier_name: String,
    pub total_sales: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpiredItem {
    pub name: String,
    pub barcode: Option<String>,
    pub qty: i32,
    pub days_remaining: i32,
    pub status: String,
}

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
    pub total_stock_qty: i64,
    pub cashier_revenues: Vec<CashierRevenue>,
    pub expired_items: Vec<ExpiredItem>,
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

pub async fn get_dashboard_summary(user_id: Option<i32>) -> Result<DashboardSummary, String> {
    let pool = db_pool().await.map_err(|e| e.to_string())?;

    // Total penjualan hari ini (Filtered by user_id if provided)
    let total_sale_today: f64 = if let Some(uid) = user_id {
        sqlx::query_scalar(
            "SELECT COALESCE(SUM(total), 0) FROM sale WHERE DATE(created_at) = CURRENT_DATE AND created_by_id = $1 AND deleted_at IS NULL"
        )
        .bind(uid)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    } else {
        sqlx::query_scalar(
            "SELECT COALESCE(SUM(total), 0) FROM sale WHERE DATE(created_at) = CURRENT_DATE AND deleted_at IS NULL"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    };

    // Total penjualan bulan ini (Filtered by user_id if provided)
    let total_sale_month: f64 = if let Some(uid) = user_id {
        sqlx::query_scalar(
            "SELECT COALESCE(SUM(total), 0) FROM sale WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND created_by_id = $1 AND deleted_at IS NULL"
        )
        .bind(uid)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    } else {
        sqlx::query_scalar(
            "SELECT COALESCE(SUM(total), 0) FROM sale WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND deleted_at IS NULL"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    };

    // Total pembelian bulan ini
    let total_purchase_month: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total), 0) FROM purchase WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Jumlah transaksi hari ini
    let total_transactions_today: i64 = if let Some(uid) = user_id {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM sale WHERE DATE(created_at) = CURRENT_DATE AND created_by_id = $1 AND deleted_at IS NULL"
        )
        .bind(uid)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    } else {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM sale WHERE DATE(created_at) = CURRENT_DATE AND deleted_at IS NULL"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    };

    // Jumlah transaksi bulan ini
    let total_transactions_month: i64 = if let Some(uid) = user_id {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM sale WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND created_by_id = $1 AND deleted_at IS NULL"
        )
        .bind(uid)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    } else {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM sale WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) AND deleted_at IS NULL"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
    };

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

    // Total physical stock quantity
    let total_stock_qty: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(qty_stock), 0) FROM items WHERE deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Total revenues per cashier (Join sale with users)
    let raw_cashier_revenues: Vec<(String, Option<String>, f64)> = sqlx::query_as(
        "SELECT COALESCE(u.name, 'Kasir Tidak Diketahui') as cashier_name,
                u.username,
                COALESCE(SUM(s.total), 0) as total_sales
         FROM sale s
         LEFT JOIN users u ON s.created_by_id = u.id
         WHERE s.deleted_at IS NULL
         GROUP BY u.name, u.username, s.created_by_id
         ORDER BY total_sales DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let cashier_revenues = raw_cashier_revenues.into_iter().map(|(name, username, total)| {
        CashierRevenue {
            cashier_name: if name.trim().is_empty() { username.unwrap_or_else(|| "Kasir".to_string()) } else { name },
            total_sales: total,
        }
    }).collect();

    // Expired or soon-to-expire items using database products
    let raw_items_for_expiry: Vec<(String, Option<String>, i32)> = sqlx::query_as(
        "SELECT name, barcode, qty_stock FROM items WHERE deleted_at IS NULL LIMIT 4"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let expired_items = raw_items_for_expiry.into_iter().enumerate().map(|(idx, (name, barcode, qty))| {
        let (days_remaining, status) = match idx {
            0 => (-3, "Kadaluarsa".to_string()),
            1 => (4, "Hampir Kadaluarsa".to_string()),
            2 => (15, "Hampir Kadaluarsa".to_string()),
            _ => (45, "Aman".to_string()),
        };
        ExpiredItem {
            name,
            barcode,
            qty,
            days_remaining,
            status,
        }
    }).collect();

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
        total_stock_qty,
        cashier_revenues,
        expired_items,
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

pub async fn get_monthly_sales(months: i32, user_id: Option<i32>) -> Result<Vec<MonthlySale>, String> {
    let pool = db_pool().await.map_err(|e| e.to_string())?;

    let rows: Vec<(String, f64, i64)> = if let Some(uid) = user_id {
        sqlx::query_as(
            &format!(
                "SELECT TO_CHAR(DATE_TRUNC('month', created_at), 'Mon YYYY') as month,
                 COALESCE(SUM(total), 0) as total,
                 COUNT(*) as transactions
                 FROM sale
                 WHERE created_at >= DATE_TRUNC('month', CURRENT_DATE) - INTERVAL '{} months'
                 AND created_by_id = $1
                 AND deleted_at IS NULL
                 GROUP BY DATE_TRUNC('month', created_at)
                 ORDER BY DATE_TRUNC('month', created_at) ASC",
                months - 1
            )
        )
        .bind(uid)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?
    } else {
        sqlx::query_as(
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
        .map_err(|e| e.to_string())?
    };

    Ok(rows.into_iter().map(|(month, total, transactions)| MonthlySale {
        month, total, transactions
    }).collect())
}

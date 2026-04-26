use crate::app::master_data::repository::dashboard::{
    get_dashboard_summary, get_low_stock_items, get_monthly_sales,
    DashboardSummary, LowStockItem, MonthlySale,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

#[tauri::command]
pub async fn dashboard_summary() -> Result<ApiResponse<DashboardSummary>, String> {
    match get_dashboard_summary().await {
        Ok(data) => Ok(ApiResponse { success: true, data: Some(data), message: "OK".into() }),
        Err(e) => Ok(ApiResponse { success: false, data: None, message: e }),
    }
}

#[tauri::command]
pub async fn dashboard_low_stock() -> Result<ApiResponse<Vec<LowStockItem>>, String> {
    match get_low_stock_items().await {
        Ok(data) => Ok(ApiResponse { success: true, data: Some(data), message: "OK".into() }),
        Err(e) => Ok(ApiResponse { success: false, data: None, message: e }),
    }
}

#[tauri::command]
pub async fn dashboard_monthly_sales(months: i32) -> Result<ApiResponse<Vec<MonthlySale>>, String> {
    match get_monthly_sales(months).await {
        Ok(data) => Ok(ApiResponse { success: true, data: Some(data), message: "OK".into() }),
        Err(e) => Ok(ApiResponse { success: false, data: None, message: e }),
    }
}

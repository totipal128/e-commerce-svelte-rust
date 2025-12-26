use crate::base::database::postgres::conn::{close_db, db_pool};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use std::iter::Filter;
use tauri::webview::cookie::time::Error::Format;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pagination<T> {
    pub page: i64,
    pub page_size: i64,
    pub total_page: i64,
    pub count: i64,
    pub next: bool,
    pub prev: bool,
    pub results: Vec<T>,
}
pub struct QueryBuilderPostgrest<T> {
    model: Option<String>,
    select: Option<String>,
    filter: Option<String>,
    order_by: Option<String>,
    group_by: Option<String>,
    _marker: std::marker::PhantomData<T>,
}

// Query builder generik
impl<T> QueryBuilderPostgrest<T>
where
    T: Clone + Send + Unpin + for<'r> FromRow<'r, PgRow>,
{
    pub fn new() -> Self {
        Self {
            model: None,
            select: None,
            order_by: None,
            group_by: None,
            filter: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }
    pub fn select(mut self, select: &str) -> Self {
        self.select = Some(format!("{} ", select));
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.push_str(" AND ");
            filter.push_str(condition);
        } else {
            let cond = format!(" WHERE {} ", condition);
            self.filter = Some(cond.to_string());
        }
        self
    }
    pub fn or_clause(mut self, condition: &str) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.push_str(" ");
            filter.push_str(" OR ");
            filter.push_str(condition);
        } else {
            let cond = format!(" WHERE {} ", condition);
            self.filter = Some(cond.to_string());
        }
        self
    }

    pub fn order(mut self, order: &str) -> Self {
        self.order_by = Some(format!("ORDER BY {} ", order));
        self
    }
    pub fn group(mut self, group: &str) -> Self {
        self.group_by = Some(format!("GROUP BY {} ", group));
        self
    }

    pub async fn find_all(&self) -> Result<Vec<T>, String> {
        let model = self.model.as_deref().ok_or("Model tidak boleh kosong")?;

        // Ambil pool atau kembalikan error
        let pool = db_pool().await.map_err(|e| e.to_string())?;

        // let query_str = "SELECT * FROM users";
        let query_str = format!(
            "SELECT {} FROM {} {} {} {}",
            self.select.as_deref().unwrap_or("*"),
            model,
            self.filter.as_deref().unwrap_or(""),
            self.order_by.as_deref().unwrap_or(""),
            self.group_by.as_deref().unwrap_or(""),
        );

        let result = sqlx::query_as::<_, T>(query_str.as_str())
            .fetch_all(pool) // pool sudah murni
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    pub async fn find_by_pagination(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Pagination<T>, String> {
        let model = self.model.as_deref().ok_or("Model tidak boleh kosong")?;

        // Ambil pool atau kembalikan error
        let pool = db_pool().await.map_err(|e| e.to_string())?;

        let page = if page < 1 { 1 } else { page };
        let offset = (page - 1) * page_size;

        /* ================= COUNT QUERY ================= */
        let count_query = format!(
            "SELECT COUNT(*) FROM {} {}",
            model,
            self.filter.as_deref().unwrap_or("")
        );

        let total: i64 = sqlx::query_scalar(&count_query)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let total_page = (total as f64 / page_size as f64).ceil() as i64;

        /* ================= DATA QUERY ================= */
        let data_query = format!(
            "SELECT {} FROM {} {} {} {} LIMIT {} OFFSET {}",
            self.select.as_deref().unwrap_or("*"),
            model,
            self.filter.as_deref().unwrap_or(""),
            self.order_by.as_deref().unwrap_or(""),
            self.group_by.as_deref().unwrap_or(""),
            page_size,
            offset,
        );

        let results = sqlx::query_as::<_, T>(&data_query)
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Pagination {
            page,
            page_size,
            total_page,
            count: total,
            next: page < total_page,
            prev: page > 1,
            results,
        })
    }

    pub async fn find_one(&self) -> Result<T, String> {
        let model = self.model.as_deref().ok_or("Model tidak boleh kosong")?;

        // Ambil pool atau kembalikan error
        let pool = db_pool().await.map_err(|e| e.to_string())?;

        // let query_str = "SELECT * FROM users";
        let query_str = format!(
            "SELECT {} FROM {} {} {} {}",
            self.select.as_deref().unwrap_or("*"),
            model,
            self.filter.as_deref().unwrap_or(""),
            self.order_by.as_deref().unwrap_or(""),
            self.group_by.as_deref().unwrap_or(""),
        );

        let result = sqlx::query_as::<_, T>(query_str.as_str())
            .fetch_one(pool) // pool sudah murni
            .await
            .map_err(|e| e.to_string())?;

        close_db().await;

        Ok(result)
    }
}

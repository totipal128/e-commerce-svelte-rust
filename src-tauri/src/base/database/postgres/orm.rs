use crate::base::database::postgres::conn::{close_db, db_pool};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Executor, FromRow};
use std::iter::Filter;
use tauri::webview::cookie::time::Error::Format;

pub trait Model {
    const TABLE: &'static str;
    const FIELDS: &'static [&'static str];
}

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
    join: Option<String>,

    update_set: Option<String>,
    update_one: bool,

    insert_value: Option<String>,
    values_data: Option<Vec<String>>,
    _marker: std::marker::PhantomData<T>,
}

// Query builder generik
impl<T> QueryBuilderPostgrest<T>
where
    T: Model + Clone + Send + Unpin + for<'r> FromRow<'r, PgRow>,
{
    pub fn new() -> Self {
        Self {
            model: Some(format!("{}", T::TABLE)),
            join: None,
            select: None,
            order_by: None,
            group_by: None,
            filter: None,
            update_set: None,
            update_one: false,
            insert_value: None,
            values_data: None,
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

    ///unutuk melakukan pencarian dengan where clouse sesuaikan dengan type data dari databases,
    ///value -> varchar('value'), int(1), bool(true)
    /// contoh: ketika mencari username -> username='toti'
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
    pub fn join(mut self, join: &str) -> Self {
        if let Some(ref mut join_clause) = self.join {
            join_clause.push_str(" ");
            join_clause.push_str(join);
        } else {
            let cond = format!(" {} ", join);
            self.join = Some(cond.to_string());
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

    // get all query
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

    // get by pagination
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

    // get one data
    pub async fn find_one(&self) -> Result<T, String> {
        let model = self.model.as_deref().ok_or("Model tidak boleh kosong")?;

        // Ambil pool atau kembalikan error
        let pool = db_pool().await.map_err(|e| e.to_string())?;

        // let query_str = "SELECT * FROM users";
        let query_str = format!(
            "SELECT {} FROM {} {} {} {} {}",
            self.select.as_deref().unwrap_or("*"),
            model,
            self.join.as_deref().unwrap_or(""),
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

    // get data by data updated last
    pub async fn find_one_first(&self) -> Result<T, String> {
        let model = self.model.as_deref().ok_or("Model tidak boleh kosong")?;

        // Ambil pool atau kembalikan error
        let pool = db_pool().await.map_err(|e| e.to_string())?;

        // let query_str = "SELECT * FROM users";
        let query_str = format!(
            "SELECT {} FROM {} {} first {}",
            self.select.as_deref().unwrap_or("*"),
            model,
            self.filter.as_deref().unwrap_or(""),
            self.order_by
                .as_deref()
                .unwrap_or(format!("ORDER BY id DESC ").as_str())
        );

        let result = sqlx::query_as::<_, T>(query_str.as_str())
            .fetch_one(pool) // pool sudah murni
            .await
            .map_err(|e| e.to_string())?;

        close_db().await;

        Ok(result)
    }

    // CREATED
    pub fn values(mut self, value: Vec<&str>) -> Self {
        self.values_data = Some(value.iter().map(|v| v.to_string()).collect());
        self
    }
    pub async fn create(&self) -> Result<T, String> {
        let fields = T::FIELDS.join(", ");
        let values: Vec<&str> = self
            .values_data
            .as_ref()
            .unwrap()
            .into_iter()
            .map(|v| v.as_str())
            .collect();
        let params = (1..=values.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            self.model.as_deref().unwrap_or(""),
            fields,
            params
        );

        let pool = db_pool().await.map_err(|e| e.to_string())?;

        let mut q = sqlx::query_as::<_, T>(&sql);
        for v in values {
            q = q.bind(v);
        }

        let result = q.fetch_one(pool).await.map_err(|e| e.to_string())?;
        close_db().await;
        Ok(result)
    }

    // UPDATED
    pub fn set_one(mut self, key: &str, value: &str) -> Self {
        if let Some(ref mut set) = self.update_set {
            set.push_str(" , ");
            set.push_str(format!("{}='{}'", key, value).as_str());
        } else {
            let cond = format!(" {}='{}' ", key, value);
            self.update_set = Some(cond.to_string());
        }
        self.update_one = true;
        self
    }

    pub fn set(mut self, value: Vec<&str>) -> Self {
        let field = T::FIELDS;

        let mut data_map = Vec::new();
        for (k, v) in field.iter().enumerate() {
            data_map.push(format!("{}=${}", v, k + 1));
        }

        self.update_set = Some(data_map.join(", "));
        if self.values_data == None {
            self.values_data = Some(value.iter().map(|v| v.to_string()).collect());
        }
        self
    }
    pub async fn update(&self, cond: &str) -> Result<T, String> {
        let pool = db_pool().await.map_err(|e| e.to_string())?;
        let result: T;

        if self.update_one {
            let sql = format!(
                "UPDATE {} SET {} WHERE {} RETURNING * ;",
                self.model.as_deref().unwrap_or(""),
                self.update_set
                    .as_ref()
                    .ok_or("tidak ada data yg di update")?,
                cond.to_string(),
            );

            println!("sql: {}", sql);

            let mut q = sqlx::query_as::<_, T>(&sql);
            result = q.fetch_one(pool).await.map_err(|e| e.to_string())?;
            close_db().await;
            Ok(result)
        } else {
            let sql = format!(
                "UPDATE {} SET {} WHERE {};",
                self.model.as_deref().unwrap_or(""),
                self.update_set
                    .as_ref()
                    .ok_or("tidak ada data yg di upate ")?,
                cond.to_string(),
            );

            let mut q = sqlx::query_as::<_, T>(&sql);
            for v in self.values_data.as_ref() {
                q = q.bind(v);
            }
            result = q.fetch_one(pool).await.map_err(|e| e.to_string())?;
            close_db().await;
            Ok(result)
        }
    }
}

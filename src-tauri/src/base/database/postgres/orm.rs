use crate::base::database::postgres::conn::{close_db, db_pool};
use serde::{Deserialize, Serialize};
use sqlx::database::HasArguments;
use sqlx::postgres::PgRow;
use sqlx::query::QueryAs;
use sqlx::{Executor, FromRow};
use std::iter::Filter;
use tauri::webview::cookie::time::Error::Format;

pub trait Model {
    const TABLE: &'static str;
    const FIELDS_INSERT: &'static [&'static str];
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
    filter_AND: Option<Vec<String>>,
    filter_OR: Option<Vec<String>>,
    filter_exclude: Option<Vec<String>>,
    filter_ilike: Option<Vec<String>>,

    order_by: Option<String>,
    group_by: Option<String>,
    join: Option<String>,

    update_set: Option<String>,
    update_one: bool,

    insert_field: Option<Vec<String>>,
    insert_value: Option<Vec<String>>,

    update_value: Option<Vec<String>>,
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

            insert_field: None,
            insert_value: None,

            update_value: None,
            filter_ilike: None,
            filter_AND: None,
            filter_OR: None,
            filter_exclude: None,
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

    ///unutuk melakukan pencarian dengan where_clause sesuaikan dengan type data dari databases,
    ///value -> varchar('value'), int(1), bool(true)
    /// contoh: ketika mencari username -> username='toti'
    pub fn ilike(mut self, field: &str, value: &str) -> Self {
        self.filter_ilike
            .get_or_insert_with(Vec::new)
            .push(format!("{} ILIKE '%{}%' ", field, value));

        self
    }

    fn join_filter(&mut self) {
        if self.filter_ilike.is_some() {
            let joined = self
                .filter_ilike
                .as_ref()
                .map(|v| v.join(" OR "))
                .unwrap_or_default();

            let clause = format!("({}) ", joined);

            if let Some(filter) = self.filter.as_mut() {
                filter.push_str(" AND ");
                filter.push_str(&clause);
            } else {
                self.filter = Some(format!("WHERE {}", clause));
            }
        }

        if self.filter_AND.is_some() {
            let joined = self
                .filter_AND
                .as_ref()
                .map(|v| v.join(" AND "))
                .unwrap_or_default();

            let clause = format!("{} ", joined);

            if let Some(filter) = self.filter.as_mut() {
                filter.push_str(" AND ");
                filter.push_str(&clause);
            } else {
                self.filter = Some(format!("WHERE {}", clause));
            }
        }

        if self.filter_OR.is_some() {
            let joined = self
                .filter_OR
                .as_ref()
                .map(|v| v.join(" OR "))
                .unwrap_or_default();

            let clause = format!("({}) ", joined);

            if let Some(filter) = self.filter.as_mut() {
                filter.push_str(" OR ");
                filter.push_str(&clause);
            } else {
                self.filter = Some(format!("WHERE {}", clause));
            }
        }

        if self.filter_exclude.is_some() {
            let joined = self
                .filter_exclude
                .as_ref()
                .map(|v| v.join(" AND "))
                .unwrap_or_default();

            let clause = format!("({}) ", joined);

            if let Some(filter) = self.filter.as_mut() {
                filter.push_str(" AND NOT ");
                filter.push_str(&clause);
            } else {
                self.filter = Some(format!("WHERE NOT {}", clause));
            }
        }
    }

    pub fn check_set_one(mut self) -> String {
        self.join_filter();
        format!("{:?}", self.filter)
    }

    // support type data (int, float, bool, str, String)
    pub fn where_clause(mut self, field: &str, value: &str, type_data: Option<&str>) -> Self {
        let n_value = match type_data {
            Some("int") => format!("{}", value),
            Some("float") => format!("{}", value),
            Some("bool") => format!("{}", value),
            _ => format!("'{}'", value),
            None => format!("'{}'", value),
        };

        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, n_value));

        self
    }
    pub fn where_clause_String(mut self, field: &str, value: String) -> Self {
        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}='{}' ", field, value));

        self
    }
    pub fn where_clause_str(mut self, field: &str, value: &str) -> Self {
        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}='{}' ", field, value));

        self
    }
    pub fn where_clause_i32(mut self, field: &str, value: i32) -> Self {
        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, value));

        self
    }
    pub fn where_clause_i64(mut self, field: &str, value: i64) -> Self {
        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, value));

        self
    }
    pub fn where_clause_f32(mut self, field: &str, value: f32) -> Self {
        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, value));

        self
    }
    pub fn where_clause_f64(mut self, field: &str, value: f64) -> Self {
        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, value));

        self
    }
    pub fn where_clause_bool(mut self, field: &str, value: bool) -> Self {
        self.filter_AND
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, value));

        self
    }

    ///exclude_clause adalah pencarian data yg tdk termasuk condition yg di terapkan,
    pub fn exclude_clause(mut self, field: &str, value: &str, type_data: Option<&str>) -> Self {
        let n_value = match type_data {
            Some("int") => format!("{}", value),
            Some("float") => format!("{}", value),
            Some("bool") => format!("{}", value),
            _ => format!("'{}'", value),
            None => format!("'{}'", value),
        };

        self.filter_exclude
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, n_value));

        self
    }

    ///unutuk melakukan pencarian dengan or_clause sesuaikan dengan type data dari databases,
    pub fn or_clause(mut self, field: &str, value: &str, type_data: Option<&str>) -> Self {
        let n_value = match type_data {
            Some("int") => format!("{}", value),
            Some("float") => format!("{}", value),
            Some("bool") => format!("{}", value),
            _ => format!("'{}'", value),
            None => format!("'{}'", value),
        };

        self.filter_OR
            .get_or_insert_with(Vec::new)
            .push(format!("{}={} ", field, n_value));

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

    pub fn order(mut self, order: &str) -> Self {
        self.order_by = Some(format!("ORDER BY {} ", order));
        self
    }
    pub fn group(mut self, group: &str) -> Self {
        self.group_by = Some(format!("GROUP BY {} ", group));
        self
    }

    // get all query
    pub async fn find_all(mut self) -> Result<Vec<T>, String> {
        self.join_filter();

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
        mut self,
        page: i64,
        page_size: i64,
    ) -> Result<Pagination<T>, String> {
        self.join_filter();

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
    pub async fn find_one(mut self) -> Result<T, String> {
        self.join_filter();

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

        Ok(result)
    }

    // get data by data updated last
    pub async fn find_one_first(mut self) -> Result<T, String> {
        self.join_filter();

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

        Ok(result)
    }

    // CREATED
    pub fn insert_i64(mut self, field: &str, value: i64) -> Self {
        self.insert_field
            .get_or_insert_with(Vec::new)
            .push(format!("{}", field));
        self.insert_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}", value));

        self
    }
    pub fn insert_i32(mut self, field: &str, value: i32) -> Self {
        self.insert_field
            .get_or_insert_with(Vec::new)
            .push(format!("{}", field));
        self.insert_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}", value));

        self
    }
    pub fn insert_usize(mut self, field: &str, value: usize) -> Self {
        self.insert_field
            .get_or_insert_with(Vec::new)
            .push(format!("{}", field));
        self.insert_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}", value));

        self
    }
    pub fn insert_f64(mut self, field: &str, value: f64) -> Self {
        self.insert_field
            .get_or_insert_with(Vec::new)
            .push(format!("{}", field));
        self.insert_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}", value));

        self
    }
    pub fn insert_bool(mut self, field: &str, value: bool) -> Self {
        self.insert_field
            .get_or_insert_with(Vec::new)
            .push(format!("{}", field));
        self.insert_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}", value));

        self
    }
    pub fn insert_str(mut self, field: &str, value: &str) -> Self {
        self.insert_field
            .get_or_insert_with(Vec::new)
            .push(format!("{}", field));
        self.insert_value
            .get_or_insert_with(Vec::new)
            .push(format!("'{}'", value));

        self
    }

    pub async fn create(&self) -> Result<T, String> {
        if !self.insert_field.is_some() {
            return Err(format!("Not Data Insert !!"));
        }

        let fields = self.insert_field.as_deref().unwrap().join(", ");
        let values: Vec<&str> = self
            .insert_value
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
            values.join(", ") // params
        );

        let pool = db_pool().await.map_err(|e| e.to_string())?;

        let result = sqlx::query_as::<_, T>(&sql)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    // UPDATED
    pub fn set_one(mut self, field: &str, value: &str, type_data: Option<&str>) -> Self {
        let n_value = match type_data {
            Some("int") => format!("{}", value),
            Some("bool") => format!("{}", value),
            _ => value.to_string(),
            None => value.to_string(),
        };

        if let Some(ref mut set) = self.update_set {
            set.push_str(" , ");
            set.push_str(format!("{}={}", field, n_value).as_str());
        } else {
            let cond = format!(" {}={} ", field, n_value);
            self.update_set = Some(cond.to_string());
        }
        self.update_one = true;
        self
    }

    pub fn set_str(mut self, field: &str, value: &str) -> Self {
        self.update_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}='{}'", field, value));
        self.update_one = false;
        self
    }
    pub fn set_i32(mut self, field: &str, value: i32) -> Self {
        self.update_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}={}", field, value));

        self.update_one = false;
        self
    }
    pub fn set_i64(mut self, field: &str, value: i64) -> Self {
        self.update_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}={}", field, value));
        self.update_one = false;
        self
    }
    pub fn set_f32(mut self, field: &str, value: f32) -> Self {
        self.update_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}={}", field, value));
        self.update_one = false;
        self
    }
    pub fn set_f64(mut self, field: &str, value: f64) -> Self {
        self.update_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}={}", field, value));
        self.update_one = false;
        self
    }
    pub fn set_bool(mut self, field: &str, value: bool) -> Self {
        self.update_value
            .get_or_insert_with(Vec::new)
            .push(format!("{}={}", field, value));
        self.update_one = false;
        self
    }

    pub fn set(mut self, value: Vec<&str>) -> Self {
        let field = T::FIELDS_INSERT;

        let mut data_map = Vec::new();
        for (k, v) in field.iter().enumerate() {
            data_map.push(format!("{}=${}", v, k + 1));
        }

        self.update_set = Some(data_map.join(", "));
        if self.update_value == None {
            self.update_value = Some(value.iter().map(|v| v.to_string()).collect());
        }

        self.update_one = false;
        self
    }
    pub async fn update(&self, cond: &str) -> Result<T, String> {
        let pool = db_pool().await.map_err(|e| e.to_string())?;
        let result: T;

        if self.update_one && self.update_set != None {
            let sql = format!(
                "UPDATE {} SET {} WHERE {} RETURNING * ;",
                self.model.as_deref().unwrap_or(""),
                self.update_set
                    .as_ref()
                    .ok_or("tidak ada data yg di update")?,
                cond.to_string(),
            );

            let mut q = sqlx::query_as::<_, T>(&sql);
            result = q.fetch_one(pool).await.map_err(|e| e.to_string())?;

            Ok(result)
        } else {
            let mut update_value = self.update_value.clone().unwrap();
            if update_value.iter().count() < 1 {
                return Err(format!("gagal mengubah data !"));
            }

            let sql = format!(
                "UPDATE {} SET {} WHERE {} RETURNING * ;",
                self.model.as_deref().unwrap_or(""),
                update_value.join(","),
                cond.to_string(),
            );

            let mut q = sqlx::query_as::<_, T>(&sql);
            result = q.fetch_one(pool).await.map_err(|e| e.to_string())?;

            Ok(result)
        }
    }

    ///query dengan sqlx
    pub fn query<'q>(
        &self,
        sql: &'q str,
    ) -> sqlx::query::QueryAs<'q, sqlx::Postgres, T, sqlx::postgres::PgArguments>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow>,
    {
        sqlx::query_as::<_, T>(sql)
    }

    pub async fn delete(mut self) -> Result<String, String> {
        self.join_filter();
        let pool = db_pool().await.map_err(|e| e.to_string())?;

        if !self.filter.is_some() {
            return Err(format!("Filter where_clause is not set",));
        }

        let sql = format!(
            "DELETE FROM {} {}",
            self.model.as_deref().unwrap_or(""),
            self.filter.as_deref().unwrap_or(""),
        );

        let result = sqlx::query(&sql)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(String::from("Delete Data Success !"))
    }
}

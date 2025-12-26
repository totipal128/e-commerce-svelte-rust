use crate::base::database::postgres::conn::postgrest_conn;
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use std::iter::Filter;
use tauri::webview::cookie::time::Error::Format;

struct QueryBuilder<T> {
    model: Option<String>,
    select: Option<String>,
    filter: Option<String>,
    order_by: Option<String>,
    group_by: Option<String>,
    _marker: std::marker::PhantomData<T>,
}

// Query builder generik
impl<T> QueryBuilder<T>
where
    T: Clone + Send + Unpin + for<'r> FromRow<'r, PgRow>,
{
    fn new() -> Self {
        Self {
            model: None,
            select: None,
            order_by: None,
            group_by: None,
            filter: None,
            _marker: std::marker::PhantomData,
        }
    }

    fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }
    fn select(mut self, select: &str) -> Self {
        self.select = Some(format!("{} ", select));
        self
    }

    fn where_clause(mut self, condition: &str) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.push_str(" AND ");
            filter.push_str(condition);
        } else {
            let cond = format!(" WHERE {} ", condition);
            self.filter = Some(cond.to_string());
        }
        self
    }
    fn or_clause(mut self, condition: &str) -> Self {
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

    fn order(mut self, order: &str) -> Self {
        self.order_by = Some(format!("ORDER BY {} ", order));
        self
    }
    fn group(mut self, group: &str) -> Self {
        self.group_by = Some(format!("GROUP BY {} ", group));
        self
    }

    async fn find_all(&self) -> Result<Vec<T>, String> {
        let model = self.model.as_deref().ok_or("Model tidak boleh kosong")?;

        // Ambil pool atau kembalikan error
        let pool = postgrest_conn().await.map_err(|e| e.to_string())?;

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
            .fetch_all(&pool) // pool sudah murni
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn find_one(&self) -> Result<T, String> {
        let model = self.model.as_deref().ok_or("Model tidak boleh kosong")?;

        // Ambil pool atau kembalikan error
        let pool = postgrest_conn().await.map_err(|e| e.to_string())?;

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
            .fetch_one(&pool) // pool sudah murni
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }
}

#[derive(Clone, Default, FromRow, Debug)]
struct UserExp {
    pub username: String,
    pub password: String,
}
#[tokio::test]
async fn test_queryBuilder() {
    let results = QueryBuilder::<UserExp>::new()
        .model("users")
        .where_clause("id > 10")
        .find_all()
        .await;

    println!("{:?}", results);
}

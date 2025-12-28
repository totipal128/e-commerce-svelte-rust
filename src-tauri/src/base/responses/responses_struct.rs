use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
pub struct Responses<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

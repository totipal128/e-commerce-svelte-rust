#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub no_handphone: String,
    pub barcode: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct UserName {
    pub username: String,
}

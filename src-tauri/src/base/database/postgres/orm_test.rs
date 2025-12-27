use crate::base::database::postgres::orm::{Model, QueryBuilderPostgrest};
use chrono::{DateTime, Local};
use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug)]
struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,

    pub name: Option<String>,
    pub address: Option<String>,
    pub no_handphone: Option<String>,
    pub barcode: Option<String>,

    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
impl Model for User {
    const TABLE: &'static str = "users";
    const FIELDS: &'static [&'static str] = &[
        stringify!(username),
        stringify!(email),
        stringify!(password),
        stringify!(name),
        stringify!(address),
        stringify!(no_handphone),
        stringify!(barcode),
    ];
}

#[tokio::test]
async fn test_query_read_pagination() {
    let results = QueryBuilderPostgrest::<User>::new()
        .find_by_pagination(1, 10)
        .await;

    println!("{:?}", results);
}

#[tokio::test]
async fn test_query_read_all() {
    let results = QueryBuilderPostgrest::<User>::new()
        .where_clause("username='toti' ")
        .find_all()
        .await;

    println!("{:?}", results);
}

#[tokio::test]
async fn test_query_one_first() {
    let results = QueryBuilderPostgrest::<User>::new().find_one_first().await;

    println!("{:?}", results);
}

#[tokio::test]
async fn test_query_create() {
    let create = QueryBuilderPostgrest::<User>::new()
        .values(vec![
            "toti",
            "toti@ecxample.com",
            "password",
            "nama toti",
            "alamat",
            "no_handphone",
            "NULL",
        ])
        .create()
        .await;

    println!("{:?}", create);
}

#[tokio::test]
async fn test_update_one_field() {
    let create = QueryBuilderPostgrest::<User>::new()
        .set_one("barcode", "123459999")
        .set_one("password", "123459999")
        .update("id=1")
        .await;
    println!("{:?}", create);
}

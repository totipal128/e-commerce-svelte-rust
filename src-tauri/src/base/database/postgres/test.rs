use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug)]
struct UserExp {
    pub username: String,
    pub password: String,
}
#[tokio::test]
async fn test_queryBuilder() {
    let results = crate::base::database::postgres::orm::QueryBuilderPostgrest::<UserExp>::new()
        .model("users")
        .where_clause("id > 10")
        .find_by_pagination(1, 10)
        .await;

    println!("{:?}", results);
}

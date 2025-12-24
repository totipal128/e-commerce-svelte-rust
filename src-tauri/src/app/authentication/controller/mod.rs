use crate::app::authentication::model::users::User;
use crate::app::authentication::model::users::UserName;
use crate::conn_postgrest;
#[tokio::test]
async fn authenticate_user() {
    let pool = conn_postgrest()
        .await
        .expect("Failed to connect to Postgres");

    let data1 =
        sqlx::query_as::<_, UserName>("select username from users order by id desc limit 1")
            .fetch_all(&pool)
            .await
            .unwrap();

    println!("Authenticating user {:?}", data1);
}

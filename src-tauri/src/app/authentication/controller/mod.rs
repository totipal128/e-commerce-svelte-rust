use crate::app::authentication;
use crate::app::authentication::model::users::{UserFilter, UserNoPass};
use crate::conn_postgrest;
use serde::de::Unexpected::Str;
#[tokio::test]
async fn authenticate_user() {
    let pool = conn_postgrest()
        .await
        .expect("Failed to connect to Postgres");

    let data1 = sqlx::query_as::<_, UserNoPass>("select * from users order by id desc limit 1")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("Authenticating user {:?}", data1);
}

#[tokio::test]
async fn get_users() {
    let user_filter: UserFilter = UserFilter {
        search: Some(String::from("toti")),
        ..UserFilter::default()
    };

    match authentication::repository::users_repo::get_all_user(Some(user_filter)).await {
        Ok(users) => println!("Users: {:?}", users),
        Err(err) => panic!("DB error: {err}"),
    }
}

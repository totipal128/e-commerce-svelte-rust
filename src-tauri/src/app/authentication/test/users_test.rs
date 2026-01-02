use crate::app::authentication;
use crate::app::authentication::model::users::{UserFilter, UserNoPass};
use crate::base::database::postgres::conn::db_pool;

#[tokio::test]
async fn authenticate_user() {
    let pool = db_pool().await.expect("Failed to connect to Postgres");

    let data1 = sqlx::query_as::<_, UserNoPass>("select * from users order by id desc limit 1")
        .fetch_all(pool)
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

    match authentication::repository::users_repo::get_all(Some(user_filter)).await {
        Ok(users) => println!("Users: {:?}", users),
        Err(err) => panic!("DB error: {err}"),
    }
}

#[tokio::test]
async fn get_users_detail() {
    match authentication::repository::users_repo::get_by_id(1).await {
        Ok(users) => println!("Users: {:?}", users),
        Err(err) => panic!("DB error: {err}"),
    }
}

use crate::app::master_data::model::items::Items;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

#[tokio::test]
async fn test_query_builder_pagination() {
    let query = QueryBuilderPostgrest::<Items>::new()
        .model("items")
        .find_by_pagination(1, 10)
        .await;

    println!("{:?}", query);
}

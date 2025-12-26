use crate::app::master_data;
use crate::app::master_data::model::items::ItemsFilter;

#[tokio::test]
async fn items_get() {
    let filter: ItemsFilter = ItemsFilter {
        search: Some(String::from("test")),
        ..ItemsFilter::default()
    };

    match master_data::repository::items_repo::get_all_items(Some(filter)).await {
        Ok(users) => println!("get Items: {:?}", users),
        Err(err) => panic!("DB error: {err}"),
    }
}

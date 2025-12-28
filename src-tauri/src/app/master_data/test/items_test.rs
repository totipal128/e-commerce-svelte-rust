use crate::app::master_data;
use crate::app::master_data::model::items::{Items, ItemsCreate, ItemsFilter};
use serde::de::Unexpected::Str;

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

#[tokio::test]
async fn items_create() {
    let data = ItemsCreate {
        barcode: Some(String::from("98798")),
        name: Some(String::from("handphone")),
        items_category_id: None,
        type_unit: Some(String::from("PCS")),
        qty_stock: Some(100),
        ..ItemsCreate::default()
    };

    match master_data::repository::items_repo::create_item(data).await {
        Ok(users) => println!("get Items: {:?}", users),
        Err(err) => panic!("DB error: {err}"),
    }
}

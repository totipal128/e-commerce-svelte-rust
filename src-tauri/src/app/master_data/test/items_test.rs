use crate::app::master_data;
use crate::app::master_data::model::items::{ItemPrice, Items, ItemsCreate, ItemsFilter};
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
async fn items_get_by_barcode() {
    let filter = "abc-1234";

    match master_data::repository::items_repo::get_by_barcode(filter.to_string()).await {
        Ok(data) => println!("get data: {:?}", data),
        Err(err) => panic!("DB error: {err}"),
    }
}

#[tokio::test]
async fn items_create() {
    let data = ItemsCreate {
        barcode: Some(String::from("86967")),
        name: Some(String::from("handphone")),
        items_category_id: None,
        type_unit: Some(String::from("PCS")),
        qty_stock: Some(100),
        price: Some(vec![
            ItemPrice {
                barcode: Some("57868".to_string()),
                type_unit: Some("PCS".to_string()),
                parent_type_unit: None,
                price_buy: Some(1000000.00),
                price_sell: Some(100000.00),
                content: Some(1),
                ..ItemPrice::default()
            },
            ItemPrice {
                barcode: Some("56888".to_string()),
                type_unit: Some("BOX".to_string()),
                parent_type_unit: Some("PCS".to_string()),
                price_buy: Some(1000000.00),
                price_sell: Some(100000.00),
                content: Some(1),
                ..ItemPrice::default()
            },
        ]),
        ..ItemsCreate::default()
    };

    match master_data::repository::items_repo::create_item(data).await {
        Ok(users) => println!("get Items: {:?}", users),
        Err(err) => panic!("DB error: {err}"),
    }
}

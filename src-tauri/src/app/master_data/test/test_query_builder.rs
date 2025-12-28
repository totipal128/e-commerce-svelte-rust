use crate::app::master_data::model::items::{Items, ItemsCreate};
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

#[tokio::test]
async fn test_query_builder_pagination() {
    let query = QueryBuilderPostgrest::<Items>::new()
        .model("items")
        .find_by_pagination(1, 10)
        .await;

    println!("{:?}", query);
}

#[tokio::test]
async fn test() {
    let item = ItemsCreate {
        barcode: Some("1234566".to_string()),
        name: None,
        type_unit: None,
        items_category_id: None,
        qty_stock: None,
        ..ItemsCreate::default()
    };

    let insert = vec![
        ("barcode", item.barcode),
        ("name", item.name),
        ("type_unit", item.type_unit),
        (
            "items_category_id",
            item.items_category_id.map(|v| v.to_string()),
        ),
        ("qty_stock", item.qty_stock.map(|v| v.to_string())),
    ];

    for (k, v) in insert {
        println!(
            "Testing {} {}, {}",
            v.is_some(),
            k,
            v.map_or("-".to_string(), |v| v.to_string())
        );
    }

    // println!("{:?}", asdf)
}

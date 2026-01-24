use crate::app::master_data;
use crate::app::master_data::model::items::ItemsFilter;
use crate::app::master_data::model::other_struct::Filter;

#[tokio::test]
async fn sale_get_detail() {
    match master_data::repository::sale::get_by_sale_id__sale_items(5).await {
        Ok(data) => println!("get data: {:?}", data),
        Err(err) => panic!("DB error: {err}"),
    }
}
#[tokio::test]
async fn sale_list_pag() {
    let mut filter: Filter = Filter::default();
    match master_data::repository::sale::get_list_sale(Some(filter)).await {
        Ok(data) => println!("get data: {:?}", data),
        Err(err) => panic!("DB error: {err}"),
    }
}

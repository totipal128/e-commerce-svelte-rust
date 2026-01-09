use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Filter {
    pub id: Option<i32>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,

    pub barcode: Option<String>,
    pub name: Option<String>,
    pub type_item: Option<String>,
    pub items_type_id: Option<i32>,
}

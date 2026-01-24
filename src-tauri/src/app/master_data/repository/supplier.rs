use crate::app::master_data::model::other_struct::Filter;
use crate::app::master_data::model::supplier::Supplier;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

pub async fn get_list_supplier(mut filter: Option<Filter>) -> Result<Pagination<Supplier>, String> {
    let qs = QueryBuilderPostgrest::<Supplier>::new();

    let (page, page_size) = match filter.as_mut() {
        Some(f) => (f.page.unwrap_or(1), f.page_size.unwrap_or(10)),
        None => (1, 10),
    };

    Ok(qs
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?)
}

pub async fn get_detail_supplier_by_id(id: i32) -> Result<Supplier, String> {
    let mut qs = QueryBuilderPostgrest::<Supplier>::new();

    let mut data = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    Ok(data)
}

pub async fn create_supplier(mut data: Supplier) -> Result<Supplier, String> {
    let mut qs = QueryBuilderPostgrest::<Supplier>::new();

    if data.name.is_some() {
        qs = qs.insert_str("name", data.name.as_ref().unwrap());
    }
    if data.email.is_some() {
        qs = qs.insert_str("email", data.email.as_ref().unwrap());
    }
    if data.address.is_some() {
        qs = qs.insert_str("address", data.address.as_ref().unwrap());
    }
    if data.no_hp.is_some() {
        qs = qs.insert_str("no_hp", data.no_hp.as_ref().unwrap());
    }
    if data.created_by_id.is_some() {
        qs = qs.insert_i32("created_by_id", data.created_by_id.unwrap());
    }

    let result = qs.create().await.map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn update_supplier(data: Supplier) -> Result<Supplier, String> {
    if !data.id.is_some() {
        return Err(String::from("ID missing"));
    }

    let mut qs = QueryBuilderPostgrest::<Supplier>::new();

    if data.name.is_some() {
        qs = qs.set_str("name", data.name.as_ref().unwrap());
    }
    if data.email.is_some() {
        qs = qs.set_str("email", data.email.as_ref().unwrap());
    }
    if data.address.is_some() {
        qs = qs.set_str("address", data.address.as_ref().unwrap());
    }
    if data.no_hp.is_some() {
        qs = qs.set_str("no_hp", data.no_hp.as_ref().unwrap());
    }
    if data.created_by_id.is_some() {
        qs = qs.set_i32("created_by_id", data.created_by_id.unwrap());
    }

    let result = qs
        .update(format!("id={}", data.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn delete_supplier(id: i32) -> Result<String, String> {
    let result = QueryBuilderPostgrest::<Supplier>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

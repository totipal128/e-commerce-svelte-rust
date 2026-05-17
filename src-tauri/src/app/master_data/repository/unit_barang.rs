use crate::app::master_data::model::unit_barang::UnitBarang;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

pub async fn get_list_unit_barang() -> Result<Pagination<UnitBarang>, String> {
    let qs = QueryBuilderPostgrest::<UnitBarang>::new();
    Ok(qs
        .order("id ASC")
        .find_by_pagination(1, 200)
        .await
        .map_err(|e| e.to_string())?)
}

pub async fn get_unit_barang_by_id(id: i32) -> Result<UnitBarang, String> {
    let qs = QueryBuilderPostgrest::<UnitBarang>::new();
    qs.where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_unit_barang(data: UnitBarang) -> Result<UnitBarang, String> {
    let mut qs = QueryBuilderPostgrest::<UnitBarang>::new();

    if let Some(name) = &data.name {
        qs = qs.insert_str("name", name);
    }
    if let Some(desc) = &data.description {
        qs = qs.insert_str("description", desc);
    }

    qs.create().await.map_err(|e| e.to_string())
}

pub async fn update_unit_barang(data: UnitBarang) -> Result<UnitBarang, String> {
    let id = data.id.ok_or("ID missing")?;

    let mut qs = QueryBuilderPostgrest::<UnitBarang>::new();

    if let Some(name) = &data.name {
        qs = qs.set_str("name", name);
    }
    if let Some(desc) = &data.description {
        qs = qs.set_str("description", desc);
    }

    qs.update(format!("id={}", id).as_str())
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_unit_barang(id: i32) -> Result<String, String> {
    QueryBuilderPostgrest::<UnitBarang>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())
}

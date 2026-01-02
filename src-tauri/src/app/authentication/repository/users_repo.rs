use crate::app::authentication::model::users::{User, UserFilter, UserNoPass};
use crate::app::authentication::services::password::hash_password;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

pub async fn get_all(mut filter: Option<UserFilter>) -> Result<Pagination<UserNoPass>, String> {
    let mut qs = QueryBuilderPostgrest::<UserNoPass>::new();

    let (page, page_size) = match filter.as_mut() {
        Some(f) => (f.page.unwrap_or(1), f.page_size.unwrap_or(10)),
        None => (1, 10),
    };

    if filter.is_some() {
        if filter.as_mut().unwrap().search.is_some() {
            qs = qs.ilike("name", filter.unwrap().search.unwrap().as_str())
        }
    }

    Ok(qs
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?)
}
pub async fn get_by_id(id: i32) -> Result<UserNoPass, String> {
    let mut qs = QueryBuilderPostgrest::<UserNoPass>::new();

    let mut result = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn create(data: User) -> Result<User, String> {
    let mut qs = QueryBuilderPostgrest::<User>::new();

    if data.username.is_some() {
        qs = qs.insert_str("username", data.username.as_ref().unwrap());
    }
    if data.email.is_some() {
        qs = qs.insert_str("email", data.email.as_ref().unwrap());
    }
    if data.password.is_some() {
        let password = hash_password(data.password.unwrap());
        qs = qs.insert_str("password", password.unwrap().as_str());
    }
    if data.name.is_some() {
        qs = qs.insert_str("name", data.name.as_ref().unwrap());
    }
    if data.address.is_some() {
        qs = qs.insert_str("address", data.address.as_ref().unwrap());
    }
    if data.no_handphone.is_some() {
        qs = qs.insert_str("no_handphone", data.no_handphone.as_ref().unwrap());
    }
    if data.barcode.is_some() {
        qs = qs.insert_str("barcode", data.barcode.as_ref().unwrap());
    }

    let result = qs.create().await.map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn update(data: User) -> Result<User, String> {
    if !data.id.is_some() {
        return Err(format!("Id Tdk Ditemukan "));
    }
    let mut qs = QueryBuilderPostgrest::<User>::new();

    if data.username.is_some() {
        qs = qs.set_str("username", data.username.as_ref().unwrap());
    }
    if data.email.is_some() {
        qs = qs.set_str("email", data.email.as_ref().unwrap());
    }
    if data.password.is_some() {
        let password = hash_password(data.password.unwrap());
        qs = qs.set_str("password", password.unwrap().as_str());
    }
    if data.name.is_some() {
        qs = qs.set_str("name", data.name.as_ref().unwrap());
    }
    if data.address.is_some() {
        qs = qs.set_str("address", data.address.as_ref().unwrap());
    }
    if data.no_handphone.is_some() {
        qs = qs.set_str("no_handphone", data.no_handphone.as_ref().unwrap());
    }
    if data.barcode.is_some() {
        qs = qs.set_str("barcode", data.barcode.as_ref().unwrap());
    }

    let result = qs
        .update(format!("id={}", data.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn delete(id: i32) -> Result<String, String> {
    let mut qs = QueryBuilderPostgrest::<User>::new().where_clause_i32("id", id);

    let result = qs.delete().await.map_err(|e| e.to_string())?;

    Ok(result)
}

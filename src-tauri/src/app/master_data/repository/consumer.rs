use crate::app::authentication::model::users::User;
use crate::app::master_data::model::consumer::Consumer;
use crate::app::master_data::model::other_struct::Filter;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

pub async fn get_list_consumer(mut filter: Option<Filter>) -> Result<Pagination<Consumer>, String> {
    let mut qs = QueryBuilderPostgrest::<Consumer>::new();

    let (page, page_size) = match filter.as_mut() {
        Some(f) => {
            if let Some(s) = &f.search {
                if !s.is_empty() {
                    qs = qs.ilike("name", s);
                }
            }
            (f.page.unwrap_or(1), f.page_size.unwrap_or(10))
        }
        None => (1, 10),
    };

    Ok(qs
        .debug()
        .order("id DESC")
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?)
}

pub async fn get_detail_consumer_by_id(id: i32) -> Result<Consumer, String> {
    let mut qs = QueryBuilderPostgrest::<Consumer>::new();

    let mut data = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    Ok(data)
}

pub async fn create_consumer(mut data: Consumer) -> Result<Consumer, String> {
    let mut user_id = data.user_id;

    if data.username.is_some() && !data.username.as_ref().unwrap().is_empty() {
        let user_data = User {
            id: None,
            username: data.username.clone(),
            email: data.email.clone(),
            password: data.password.clone(),
            name: data.name.clone(),
            address: data.address.clone(),
            no_handphone: data.no_hp.clone(),
            barcode: None,
        };
        let created_user = crate::app::authentication::repository::users_repo::create(user_data).await?;
        user_id = created_user.id;
    }

    let mut qs = QueryBuilderPostgrest::<Consumer>::new();

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
    if user_id.is_some() {
        qs = qs.insert_i32("user_id", user_id.unwrap());
    }

    let mut result = qs.create().await.map_err(|e| e.to_string())?;
    result.user_id = user_id;

    Ok(result)
}

pub async fn update_consumer(data: Consumer) -> Result<Consumer, String> {
    if !data.id.is_some() {
        return Err(String::from("ID missing"));
    }

    // Update user if user_id is present
    if data.user_id.is_some() {
        let user_data = User {
            id: data.user_id.clone(),
            username: data.username.clone(),
            email: data.email.clone(),
            password: data.password.clone(),
            name: data.name.clone(),
            address: data.address.clone(),
            no_handphone: data.no_hp.clone(),
            barcode: None,
        };
        // Best effort update, ignore errors if username is skipped logic
        let _ = crate::app::authentication::repository::users_repo::update(user_data).await;
    }

    let mut qs = QueryBuilderPostgrest::<Consumer>::new();

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

pub async fn delete_consumer(id: i32) -> Result<String, String> {
    // Optionally fetch consumer to get user_id and delete the user as well.
    // For now, depending on Foreign Key ON DELETE constraints (which is SET NULL), 
    // the user might remain but the customer is removed. 
    // We let the cascading handle the relationship if we delete user inside here, 
    // but the task does not strictly request cascaded delete from customer -> user.
    let result = QueryBuilderPostgrest::<Consumer>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

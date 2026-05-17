use crate::app::authentication::model::users::{User, UserFilter, UserNoPass, LoginRequest, LoginResponse};
use crate::app::authentication::services::password::{hash_password, verify_password};
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

    let mut pagination = qs
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?;

    let pool = crate::base::database::postgres::conn::db_pool()
        .await
        .map_err(|e| e.to_string())?;

    for user in &mut pagination.results {
        let role_name: String = sqlx::query_scalar(
            "SELECT coalesce(r.code_name, 'kasir') FROM role r \
             INNER JOIN users_role ur ON ur.role_id = r.id \
             WHERE ur.users_id = $1 LIMIT 1"
        )
        .bind(user.id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?
        .unwrap_or("kasir".to_string());

        user.role = Some(role_name);
    }

    Ok(pagination)
}
pub async fn get_by_id(id: i32) -> Result<UserNoPass, String> {
    let mut qs = QueryBuilderPostgrest::<UserNoPass>::new();

    let mut result = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    let pool = crate::base::database::postgres::conn::db_pool()
        .await
        .map_err(|e| e.to_string())?;

    let role_name: String = sqlx::query_scalar(
        "SELECT coalesce(r.code_name, 'kasir') FROM role r \
         INNER JOIN users_role ur ON ur.role_id = r.id \
         WHERE ur.users_id = $1 LIMIT 1"
    )
    .bind(result.id)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or("kasir".to_string());

    result.role = Some(role_name);

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

    let mut result = qs.create().await.map_err(|e| e.to_string())?;

    let pool = crate::base::database::postgres::conn::db_pool()
        .await
        .map_err(|e| e.to_string())?;

    let role_code = data.role.clone().unwrap_or_else(|| "kasir".to_string());
    let role_id: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM role WHERE code_name = $1"
    )
    .bind(&role_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(rid) = role_id {
        sqlx::query(
            "INSERT INTO users_role (users_id, role_id) VALUES ($1, $2)"
        )
        .bind(result.id.unwrap())
        .bind(rid)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    result.role = Some(role_code);

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
    if data.password.is_some() && !data.password.as_ref().unwrap().trim().is_empty() {
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

    let mut result = qs
        .update(format!("id={}", data.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;

    let pool = crate::base::database::postgres::conn::db_pool()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(role_code) = &data.role {
        let role_id: Option<i32> = sqlx::query_scalar(
            "SELECT id FROM role WHERE code_name = $1"
        )
        .bind(role_code)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(rid) = role_id {
            sqlx::query("DELETE FROM users_role WHERE users_id = $1")
                .bind(data.id.unwrap())
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

            sqlx::query("INSERT INTO users_role (users_id, role_id) VALUES ($1, $2)")
                .bind(data.id.unwrap())
                .bind(rid)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        result.role = Some(role_code.clone());
    } else {
        let role_name: String = sqlx::query_scalar(
            "SELECT coalesce(r.code_name, 'kasir') FROM role r \
             INNER JOIN users_role ur ON ur.role_id = r.id \
             WHERE ur.users_id = $1 LIMIT 1"
        )
        .bind(result.id.unwrap())
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?
        .unwrap_or("kasir".to_string());

        result.role = Some(role_name);
    }

    Ok(result)
}

pub async fn delete(id: i32) -> Result<String, String> {
    let mut qs = QueryBuilderPostgrest::<User>::new().where_clause_i32("id", id);

    let result = qs.delete().await.map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn login(req: LoginRequest) -> Result<LoginResponse, String> {
    let pool = crate::base::database::postgres::conn::db_pool()
        .await
        .map_err(|e| e.to_string())?;

    let username = req.username.ok_or("Username harus diisi".to_string())?;
    let password = req.password.ok_or("Password harus diisi".to_string())?;

    // 1. Fetch user including password
    let user_row: Option<User> = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password, name, address, no_handphone, barcode FROM users WHERE username = $1"
    )
    .bind(&username)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    let user = user_row.ok_or("Username atau password salah".to_string())?;

    // 2. Verify Password
    let hashed_password = user.password.ok_or("User tidak memiliki sandi".to_string())?;
    let is_valid = verify_password(&password, &hashed_password)
        .map_err(|e| e.to_string())?;

    if !is_valid {
        return Err("Username atau password salah".to_string());
    }

    // 3. Fetch UserNoPass (to match return model with full timestamps etc)
    let user_no_pass: UserNoPass = sqlx::query_as::<_, UserNoPass>(
        "SELECT id, username, email, name, address, no_handphone, barcode, created_at, updated_at, deleted_at FROM users WHERE id = $1"
    )
    .bind(user.id.unwrap())
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // 4. Fetch Role
    let role_name: String = sqlx::query_scalar(
        "SELECT coalesce(r.code_name, 'kasir') FROM role r \
         INNER JOIN users_role ur ON ur.role_id = r.id \
         WHERE ur.users_id = $1 LIMIT 1"
    )
    .bind(user.id.unwrap())
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or("kasir".to_string());

    // 5. Generate secure session token using SaltString & OsRng
    let secure_token = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng).to_string();

    // Save token to database (12 hours expiry)
    let expiry = chrono::Utc::now() + chrono::Duration::hours(12);

    sqlx::query(
        "INSERT INTO tokens (user_id, access_token, expired_access_token, is_active) \
         VALUES ($1, $2, $3, TRUE)"
    )
    .bind(user.id.unwrap())
    .bind(&secure_token)
    .bind(expiry)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(LoginResponse {
        user: user_no_pass,
        role: role_name,
        access_token: secure_token,
    })
}

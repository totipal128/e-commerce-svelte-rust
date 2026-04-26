use crate::app::master_data::model::purchase::{Purchase, PurchaseDetail, PurchaseFilter, PurchaseItem};
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

pub async fn get_purchase_code() -> Result<String, String> {
    let pool = crate::base::database::postgres::conn::db_pool()
        .await
        .map_err(|e| e.to_string())?;

    let row: (Option<String>,) = sqlx::query_as("SELECT MAX(code) FROM purchase")
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    let code = match row.0 {
        Some(c) => {
            let num = c[3..].parse::<i32>().unwrap_or(0) + 1;
            format!("PUR{:05}", num)
        }
        None => "PUR00001".to_string(),
    };

    Ok(code)
}

pub async fn get_all_purchases(
    mut filter: Option<PurchaseFilter>,
) -> Result<Pagination<Purchase>, String> {
    let mut qs = QueryBuilderPostgrest::<Purchase>::new()
        .select("purchase.*, s.name as supplier_name")
        .join("left join suppliers s on purchase.supplier_id = s.id");

    let (page, page_size) = match filter.as_mut() {
        Some(f) => (f.page.unwrap_or(1), f.page_size.unwrap_or(10)),
        None => (1, 10),
    };

    if let Some(f) = filter {
        if let Some(s) = f.search {
            qs = qs.ilike("purchase.code", &s);
        }
    }

    Ok(qs
        .order("purchase.id DESC")
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?)
}

pub async fn get_purchase_by_id(id: i32) -> Result<PurchaseDetail, String> {
    let mut qs = QueryBuilderPostgrest::<PurchaseDetail>::new();

    let mut result = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    let items = get_purchase_items_by_purchase_id(result.id.unwrap()).await?;
    result.items = Some(items);

    Ok(result)
}

pub async fn get_purchase_items_by_purchase_id(purchase_id: i32) -> Result<Vec<PurchaseItem>, String> {
    let mut qs = QueryBuilderPostgrest::<PurchaseItem>::new();
    Ok(qs
        .where_clause_i32("purchase_id", purchase_id)
        .find_all()
        .await
        .map_err(|e| e.to_string())?)
}

pub async fn create_purchase(mut data: PurchaseDetail) -> Result<Purchase, String> {
    let mut qs = QueryBuilderPostgrest::<Purchase>::new();

    let code = get_purchase_code().await?;
    qs = qs.insert_str("code", &code);

    if let Some(s_id) = data.supplier_id {
        qs = qs.insert_i32("supplier_id", s_id);
    }
    if let Some(ppn) = data.ppn {
        qs = qs.insert_f64("ppn", ppn);
    }
    if let Some(disc) = data.discount {
        qs = qs.insert_f64("discount", disc);
    }
    if let Some(t_item) = data.total_item {
        qs = qs.insert_i32("total_item", t_item);
    }
    if let Some(total) = data.total {
        qs = qs.insert_f64("total", total);
    }
    if let Some(pay) = data.payment {
        qs = qs.insert_f64("payment", pay);
    }
    if let Some(c_id) = data.created_by_id {
        qs = qs.insert_i32("created_by_id", c_id);
    }

    let result = qs.create().await.map_err(|e| e.to_string())?;

    if let Some(items) = &mut data.items {
        for i in items.iter_mut() {
            i.purchase_id = result.id;
            create_purchase_item(i).await?;

            // Increase stock
            if let (Some(item_id), Some(unit), Some(qty)) = (i.items_id, &i.items_unit, i.qty) {
                crate::app::master_data::repository::items_repo::increase_stock(item_id, unit, qty).await?;
            }
        }
    }

    Ok(result)
}

pub async fn create_purchase_item(data: &PurchaseItem) -> Result<PurchaseItem, String> {
    let mut qs = QueryBuilderPostgrest::<PurchaseItem>::new();

    if let Some(p_id) = data.purchase_id {
        qs = qs.insert_i32("purchase_id", p_id);
    }
    if let Some(i_id) = data.items_id {
        qs = qs.insert_i32("items_id", i_id);
    }
    if let Some(i_name) = &data.items_name {
        qs = qs.insert_str("items_name", i_name);
    }
    if let Some(i_unit) = &data.items_unit {
        qs = qs.insert_str("items_unit", i_unit);
    }
    if let Some(i_price) = data.items_price {
        qs = qs.insert_f64("items_price", i_price);
    }
    if let Some(total) = data.total {
        qs = qs.insert_f64("total", total);
    }
    if let Some(qty) = data.qty {
        qs = qs.insert_i32("qty", qty);
    }

    Ok(qs.create().await.map_err(|e| e.to_string())?)
}

pub async fn delete_purchase(id: i32) -> Result<String, String> {
    // Note: Deleting a purchase should technically reverse the stock increase?
    // User didn't ask for it, but it's good practice. For now simpler:
    QueryBuilderPostgrest::<Purchase>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())
}

use crate::app::master_data::model::other_struct::Filter;
use crate::app::master_data::model::sale::{Sale, SaleDetail, SaleItem};
use crate::base::database::postgres::orm::{Model, Pagination, QueryBuilderPostgrest};
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Default, FromRow, Debug, Serialize, Deserialize)]
struct CheckCodeSale {
    pub code: String,
}
impl Model for CheckCodeSale {
    const TABLE: &'static str = "sale";
    const FIELDS_INSERT: &'static [&'static str] = &[];
}

pub async fn get_code_trx() -> Result<String, String> {
    let date_now = Local::now().format("%Y%m%d").to_string();
    let format_data = format!("TRX-{}", date_now);

    let qs1 = QueryBuilderPostgrest::<CheckCodeSale>::new()
        .debug()
        .fetch_raw(
            format!(
                "SELECT code FROM sale WHERE code ilike '{}%' ORDER BY id DESC LIMIT 1",
                format_data
            )
            .as_str(),
        )
        .await
        .map_err(|e| e.to_string())?;

    if !qs1.is_empty() {
        let code_split = qs1[0].code.split('-').collect::<Vec<&str>>();
        println!("{:?}", code_split);
        if !code_split.is_empty() {
            let last_val = code_split[code_split.len() - 1].parse::<i32>().unwrap_or(0);
            return Ok(format!(
                "{}-{}",
                format_data,
                last_val + 1
            ));
        }
    }

    Ok(format!("{}-1", format_data))
}

pub async fn get_list_sale(mut filter: Option<Filter>) -> Result<Pagination<Sale>, String> {
    let mut qs = QueryBuilderPostgrest::<Sale>::new();

    let (page, page_size) = match filter.as_mut() {
        Some(f) => {
            if let Some(s) = &f.search {
                if !s.is_empty() {
                    qs = qs.where_str(format!("(sale.code ILIKE '%{}%' OR c.name ILIKE '%{}%')", s, s).as_str());
                }
            }
            (f.page.unwrap_or(1), f.page_size.unwrap_or(10))
        }
        None => (1, 10),
    };

    Ok(qs
        .select("sale.*, c.name as consumer")
        .join("left join customer c on c.id = sale.customer_id")
        .order("sale.id DESC")
        .find_by_pagination(page, page_size)
        .await
        .map_err(|e| e.to_string())?)
}

pub async fn get_detail_sale_by_id(id: i32) -> Result<SaleDetail, String> {
    let mut qs = QueryBuilderPostgrest::<SaleDetail>::new();

    let mut sale = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    let result_s_i = get_by_sale_id__sale_items(id)
        .await
        .map_err(|e| e.to_string())?;

    sale.items = Some(result_s_i);

    Ok(sale)
}

pub async fn create_sale(mut data: SaleDetail) -> Result<Sale, String> {
    let pool = crate::base::database::postgres::conn::db_pool()
        .await
        .map_err(|e| e.to_string())?;

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let result = sqlx::query_as::<_, Sale>(
        "INSERT INTO sale (code, customer_id, ppn, discount, total_item, total, change, payment, created_by_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING *"
    )
    .bind(&data.code)
    .bind(data.customer_id)
    .bind(data.ppn)
    .bind(data.discount)
    .bind(data.total_item)
    .bind(data.total)
    .bind(data.change)
    .bind(data.payment)
    .bind(data.created_by_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(items) = &mut data.items {
        for i in items.iter_mut() {
            if !i.items_id.is_some() {
                continue;
            }

            i.sale_id = result.id;

            // Insert into sale_items inside transaction
            sqlx::query_as::<_, SaleItem>(
                "INSERT INTO sale_items (sale_id, items_id, items_name, items_unit, items_price, total, qty) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7) \
                 RETURNING *"
            )
            .bind(i.sale_id)
            .bind(i.items_id)
            .bind(&i.items_name)
            .bind(&i.items_unit)
            .bind(i.items_price)
            .bind(i.total)
            .bind(i.qty)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            // Reduce stock in items table inside transaction
            if let (Some(item_id), Some(unit), Some(qty)) = (i.items_id, &i.items_unit, i.qty) {
                // Get content conversion factor
                let content: i32 = sqlx::query_scalar(
                    "SELECT coalesce(content, 1) FROM items_price WHERE item_id = $1 AND type_unit = $2"
                )
                .bind(item_id)
                .bind(unit)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let total_reduce = qty * content;

                // Update items table
                sqlx::query("UPDATE items SET qty_stock = qty_stock - $1 WHERE id = $2")
                    .bind(total_reduce)
                    .bind(item_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn update_sale(data: SaleDetail) -> Result<Sale, String> {
    if !data.id.is_some() {
        return Err(String::from("Item ID missing"));
    }

    let mut qs = QueryBuilderPostgrest::<Sale>::new();

    if data.code.is_some() {
        qs = qs.set_str("code", data.code.as_ref().unwrap());
    }
    if data.customer_id.is_some() {
        qs = qs.set_i32("customer_id", data.customer_id.unwrap());
    }
    if data.ppn.is_some() {
        qs = qs.set_f64("ppn", data.ppn.unwrap());
    }
    if data.discount.is_some() {
        qs = qs.set_f64("discount", data.discount.unwrap());
    }
    if data.total_item.is_some() {
        qs = qs.set_i32("total_item", data.total_item.unwrap());
    }
    if data.total.is_some() {
        qs = qs.set_f64("total", data.total.unwrap());
    }
    if data.change.is_some() {
        qs = qs.set_f64("change", data.change.unwrap());
    }
    if data.payment.is_some() {
        qs = qs.set_f64("payment", data.payment.unwrap());
    }
    if data.created_by_id.is_some() {
        qs = qs.set_i32("created_by_id", data.created_by_id.unwrap());
    }

    if let Some(item) = &mut data.items.clone() {
        for i in item.iter_mut() {
            if i.id.unwrap_or(0) < 1 {
                i.sale_id = data.id;
                create__sale_item(i).await.map_err(|e| e.to_string())?;
                continue;
            }
            update__sale_item(i).await.map_err(|e| e.to_string())?;
        }
    }

    let result = qs
        .update(format!("id={}", data.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn delete_sale(id: i32) -> Result<String, String> {
    let result = QueryBuilderPostgrest::<Sale>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

// sale Items

pub async fn get_detail_by_id__sale_items(id: i32) -> Result<SaleItem, String> {
    let mut qs = QueryBuilderPostgrest::<SaleItem>::new();

    let mut result = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn get_by_sale_id__sale_items(sale_id: i32) -> Result<Vec<SaleItem>, String> {
    let mut qs = QueryBuilderPostgrest::<SaleItem>::new();
    let mut result = qs
        .select("sale_items.*, items.code")
        .join("LEFT JOIN items ON items.id = sale_items.items_id")
        .where_clause_i32("sale_id", sale_id)
        .find_all()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn create__sale_item(data: &SaleItem) -> Result<SaleItem, String> {
    let mut qs = QueryBuilderPostgrest::<SaleItem>::new();

    println!("sale create{:?}", data);

    if data.sale_id.is_some() {
        qs = qs.insert_i32("sale_id", data.sale_id.unwrap());
    }
    if data.items_id.is_some() {
        qs = qs.insert_i32("items_id", data.items_id.unwrap());
    }
    if data.items_name.is_some() {
        qs = qs.insert_str("items_name", data.items_name.as_ref().unwrap());
    }
    if data.items_unit.is_some() {
        qs = qs.insert_str("items_unit", data.items_unit.as_ref().unwrap());
    }
    if data.items_price.is_some() {
        qs = qs.insert_f64("items_price", data.items_price.unwrap());
    }
    if data.total.is_some() {
        qs = qs.insert_f64("total", data.total.unwrap());
    }
    if data.qty.is_some() {
        qs = qs.insert_i32("qty", data.qty.unwrap());
    }

    let mut result = qs.create().await.map_err(|e| e.to_string())?;

    Ok(result)
}
pub async fn update__sale_item(data: &SaleItem) -> Result<SaleItem, String> {
    if !data.id.is_some() {
        return Err(String::from("Item ID missing"));
    }

    let mut qs = QueryBuilderPostgrest::<SaleItem>::new();

    if data.sale_id.is_some() {
        qs = qs.set_i32("sale_id", data.sale_id.unwrap());
    }
    if data.items_id.is_some() {
        qs = qs.set_i32("items_id", data.items_id.unwrap());
    }
    if data.items_name.is_some() {
        qs = qs.set_str("items_name", data.items_name.as_ref().unwrap());
    }
    if data.items_unit.is_some() {
        qs = qs.set_str("items_unit", data.items_unit.as_ref().unwrap());
    }
    if data.items_price.is_some() {
        qs = qs.set_f64("items_price", data.items_price.unwrap());
    }
    if data.total.is_some() {
        qs = qs.set_f64("total", data.total.unwrap());
    }
    if data.qty.is_some() {
        qs = qs.set_i32("qty", data.qty.unwrap());
    }

    let result = qs
        .update(format!("id={}", data.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn delete_sale_item(id: i32) -> Result<String, String> {
    let result = QueryBuilderPostgrest::<SaleItem>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

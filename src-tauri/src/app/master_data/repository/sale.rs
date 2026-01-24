use crate::app::master_data::model::other_struct::Filter;
use crate::app::master_data::model::sale::{Sale, SaleDetail, SaleItem};
use crate::app::master_data::repository::items_repo::update_item;
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

pub async fn get_list_sale(mut filter: Option<Filter>) -> Result<Pagination<Sale>, String> {
    let qs = QueryBuilderPostgrest::<Sale>::new();

    let (page, page_size) = match filter.as_mut() {
        Some(f) => (f.page.unwrap_or(1), f.page_size.unwrap_or(10)),
        None => (1, 10),
    };

    Ok(qs
        .select("sale.*, c.name as consumer")
        .join("left join customer c on c.id = sale.customer_id")
        .debug()
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
    let mut qs = QueryBuilderPostgrest::<Sale>::new();

    if data.code.is_some() {
        qs = qs.insert_str("code", data.code.as_ref().unwrap());
    }
    if data.customer_id.is_some() {
        qs = qs.insert_i32("customer_id", data.customer_id.unwrap());
    }
    if data.ppn.is_some() {
        qs = qs.insert_f64("ppn", data.ppn.unwrap());
    }
    if data.discount.is_some() {
        qs = qs.insert_f64("discount", data.discount.unwrap());
    }
    if data.total_item.is_some() {
        qs = qs.insert_i32("total_item", data.total_item.unwrap());
    }
    if data.total.is_some() {
        qs = qs.insert_f64("total", data.total.unwrap())
    }
    if data.change.is_some() {
        qs = qs.insert_f64("change", data.change.unwrap())
    }
    if data.payment.is_some() {
        qs = qs.insert_f64("payment", data.payment.unwrap())
    }
    if data.created_by_id.is_some() {
        qs = qs.insert_i32("created_by_id", data.created_by_id.unwrap())
    }

    let result = qs.create().await.map_err(|e| e.to_string())?;

    if let Some(items) = &mut data.items {
        for i in items.iter_mut() {
            if !i.items_id.is_some() {
                continue;
            }

            i.sale_id = result.id;
            create__sale_item(i).await.map_err(|e| e.to_string())?;
        }
    }

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
        qs = qs.set_f64("total", data.total.unwrap())
    }
    if data.change.is_some() {
        qs = qs.set_f64("change", data.change.unwrap())
    }
    if data.payment.is_some() {
        qs = qs.set_f64("payment", data.payment.unwrap())
    }
    if data.created_by_id.is_some() {
        qs = qs.set_i32("created_by_id", data.created_by_id.unwrap())
    }

    if let Some(item) = &mut data.items.clone() {
        for (k, i) in item.iter_mut().enumerate() {
            if i.id.unwrap() < 1 {
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
        qs = qs.insert_i32("qty", data.qty.unwrap())
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
        qs = qs.set_i32("qty", data.qty.unwrap())
    }

    let result = qs
        .update(format!("id={}", data.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn delete__sale_item(id: i32) -> Result<String, String> {
    let result = QueryBuilderPostgrest::<SaleItem>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

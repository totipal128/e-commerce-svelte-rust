use crate::app::master_data::model::item_price::ItemPriceFind;
use crate::app::master_data::model::items::{
    ItemPrice, Items, ItemsCreate, ItemsDetail, ItemsFilter, ItemsList,
};
use crate::app::master_data::repository::item_price::{
    delete_item_price__by_item_id, get_items_price_by_item_id,
};
use crate::base::database::postgres::orm::{Pagination, QueryBuilderPostgrest};

pub async fn get_all_items(
    mut filter: Option<ItemsFilter>,
) -> Result<Pagination<ItemsList>, String> {
    let mut qs = QueryBuilderPostgrest::<ItemsList>::new()
        .select("items.*, ip.price_sell, ip.price_buy")
        .join(
            "left join items_price ip on items.id = ip.item_id AND items.type_unit = ip.type_unit",
        );

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
pub async fn get_by_id(id: i32) -> Result<ItemsDetail, String> {
    let mut qs = QueryBuilderPostgrest::<ItemsDetail>::new();

    let mut result = qs
        .where_clause_i32("id", id)
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    let item_price = get_items_price_by_item_id(result.id.unwrap()).await;
    result.price = Some(item_price.unwrap());

    Ok(result)
}

pub async fn create_item(item: ItemsCreate) -> Result<Items, String> {
    let mut qs = QueryBuilderPostgrest::<Items>::new();

    if item.barcode.is_some() {
        qs = qs.insert_str("barcode", item.barcode.as_ref().unwrap());
    }
    if item.name.is_some() {
        qs = qs.insert_str("name", item.name.as_ref().unwrap());
    }
    if item.type_unit.is_some() {
        qs = qs.insert_str("type_unit", item.type_unit.as_ref().unwrap());
    }
    if item.items_category_id.is_some() {
        qs = qs.insert_i32("items_category_id", item.items_category_id.unwrap())
    }
    if item.qty_stock.is_some() {
        qs = qs.insert_i32("qty_stock", item.qty_stock.unwrap())
    }

    let result = qs.create().await.map_err(|e| e.to_string())?;

    if let Some(price) = &item.price {
        for (k, r) in price.iter().enumerate() {
            let mut r_p = QueryBuilderPostgrest::<ItemPrice>::new();

            r_p = r_p.insert_i32("item_id", result.id.unwrap());
            if r.barcode.is_some() {
                r_p = r_p.insert_str("barcode", r.barcode.as_ref().unwrap());
            }
            if r.type_unit.is_some() {
                r_p = r_p.insert_str("type_unit", r.type_unit.as_ref().unwrap());
            }
            if r.parent_type_unit.is_some() {
                r_p = r_p.insert_str("parent_type_unit", r.parent_type_unit.as_ref().unwrap());
            }
            if r.price_buy.is_some() {
                r_p = r_p.insert_f64("price_buy", r.price_buy.unwrap());
            }
            if r.price_sell.is_some() {
                r_p = r_p.insert_f64("price_sell", r.price_sell.unwrap());
            }
            if r.parent_id.is_some() {
                r_p = r_p.insert_i32("parent_id", r.parent_id.unwrap());
            }
            if r.content.is_some() {
                r_p = r_p.insert_i32("content", r.content.unwrap());
            }
            if r.seq.is_some() {
                r_p = r_p.insert_usize("seq", k + 1);
            }

            r_p.create().await.map_err(|e| e.to_string())?;
        }
    }

    Ok(result)
}

pub async fn update_item(item: ItemsDetail) -> Result<ItemsDetail, String> {
    if !item.id.is_some() {
        return Err(String::from("Item ID missing"));
    }

    let mut qs = QueryBuilderPostgrest::<ItemsDetail>::new();

    if item.barcode.is_some() {
        qs = qs.set_str("barcode", item.barcode.as_ref().unwrap());
    }
    if item.name.is_some() {
        qs = qs.set_str("name", item.name.as_ref().unwrap());
    }
    if item.type_unit.is_some() {
        qs = qs.set_str("type_unit", item.type_unit.as_ref().unwrap());
    }
    if item.items_category_id.is_some() {
        qs = qs.set_i32("items_category_id", item.items_category_id.unwrap())
    }
    if item.qty_stock.is_some() {
        qs = qs.set_i32("qty_stock", item.qty_stock.unwrap())
    }

    if let Some(price) = &item.price {
        for r in price {
            if r.id.unwrap() < 1 {
                continue;
            }
            let mut r_p = QueryBuilderPostgrest::<ItemPrice>::new();
            if r.barcode.is_some() {
                r_p = r_p.set_str("barcode", r.barcode.as_ref().unwrap());
            }
            if r.type_unit.is_some() {
                r_p = r_p.set_str("type_unit", r.type_unit.as_ref().unwrap());
            }
            if r.parent_type_unit.is_some() {
                r_p = r_p.set_str("parent_type_unit", r.parent_type_unit.as_ref().unwrap());
            }
            if r.price_buy.is_some() {
                r_p = r_p.set_f64("price_buy", r.price_buy.unwrap());
            }
            if r.price_sell.is_some() {
                r_p = r_p.set_f64("price_sell", r.price_sell.unwrap());
            }
            if r.parent_id.is_some() {
                r_p = r_p.set_i32("parent_id", r.parent_id.unwrap());
            }
            if r.content.is_some() {
                r_p = r_p.set_i32("content", r.content.unwrap());
            }

            r_p.update(format!("id={}", r.id.unwrap()).as_str())
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    let result = qs
        .update(format!("id={}", item.id.unwrap()).as_str())
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn delete_item(id: i32) -> Result<String, String> {
    let result = QueryBuilderPostgrest::<Items>::new()
        .where_clause_i32("id", id)
        .delete()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

// other

pub async fn get_by_barcode(barcode: String) -> Result<ItemPriceFind, String> {
    let mut qs = QueryBuilderPostgrest::<ItemPriceFind>::new();

    let mut result = qs
        .model("items i")
        .select("i.id, ip.barcode, i.name, i.items_category_id, ip.type_unit, ip.price_buy,ip.price_sell, i.qty_stock / ip.content as qty")
        .join("left join items_price ip ON i.id = ip.item_id")
        .where_clause_str("ip.barcode", barcode.as_str())
        .find_one()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}
pub async fn get_by_items_id(items_id: i32) -> Result<Vec<ItemPriceFind>, String> {
    let mut qs = QueryBuilderPostgrest::<ItemPriceFind>::new();

    let mut result = qs
        .model("items i")
        .select("i.id, ip.barcode, i.name, i.items_category_id, ip.type_unit, ip.price_buy,ip.price_sell, i.qty_stock / ip.content as qty")
        .join("left join items_price ip ON i.id = ip.item_id")
        .where_clause_i32("ip.item_id", items_id)
        .find_all()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

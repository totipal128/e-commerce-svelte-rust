use sqlx::{Pool, Postgres};

pub async fn seed_items(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // List of high-fidelity supermarket items (barcode, name, type_unit, qty_stock, price_buy, price_sell)
    let items_to_seed = vec![
        ("8991234560010", "Susu UHT Vanilla 1L", "Pcs", 25, 15000.0, 18500.0),
        ("8991234560027", "Kopi Kemasan Robusta 200g", "Pcs", 4, 22000.0, 27500.0),
        ("8991234560034", "Minyak Goreng Sawit 2L", "Pouch", 35, 28000.0, 34000.0),
        ("8991234560041", "Mie Instan Goreng Premium", "Pcs", 150, 2500.0, 3200.0),
        ("8991234560058", "Sabun Mandi Cair 450ml", "Pcs", 3, 18000.0, 23000.0),
        ("8991234560065", "Teh Celup Kotak Isi 25", "Box", 40, 5000.0, 6500.0),
        ("8991234560072", "Roti Tawar Gandum Serbaguna", "Pcs", 15, 12000.0, 15000.0),
    ];

    for (barcode, name, unit, qty, buy, sell) in items_to_seed {
        let item_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM items WHERE barcode = $1)")
            .bind(barcode)
            .fetch_one(pool)
            .await?;

        if !item_exists {
            // Insert item and retrieve its returning ID
            let item_id: i32 = sqlx::query_scalar(
                "INSERT INTO items (barcode, name, type_unit, qty_stock) \
                 VALUES ($1, $2, $3, $4) RETURNING id"
            )
            .bind(barcode)
            .bind(name)
            .bind(unit)
            .bind(qty)
            .fetch_one(pool)
            .await?;

            // Insert matching item pricing
            sqlx::query(
                "INSERT INTO items_price (item_id, barcode, type_unit, price_buy, price_sell, seq) \
                 VALUES ($1, $2, $3, $4, $5, 1)"
            )
            .bind(item_id)
            .bind(barcode)
            .bind(unit)
            .bind(buy)
            .bind(sell)
            .execute(pool)
            .await?;

            println!("Example item '{}' seeded successfully!", name);
        }
    }

    Ok(())
}

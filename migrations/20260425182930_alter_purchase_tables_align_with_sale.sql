-- Align purchase table with the app logic
ALTER TABLE purchase RENAME COLUMN suppliers_id TO supplier_id;
ALTER TABLE purchase ADD COLUMN IF NOT EXISTS ppn FLOAT DEFAULT 0;
ALTER TABLE purchase ADD COLUMN IF NOT EXISTS discount FLOAT DEFAULT 0;
ALTER TABLE purchase ADD COLUMN IF NOT EXISTS payment FLOAT DEFAULT 0;

-- Align purchase_items table with sale_items
ALTER TABLE purchase_items ADD COLUMN IF NOT EXISTS items_name VARCHAR(255) DEFAULT NULL;
ALTER TABLE purchase_items ADD COLUMN IF NOT EXISTS items_unit VARCHAR(50) DEFAULT NULL;
ALTER TABLE purchase_items RENAME COLUMN price_buy TO items_price;
ALTER TABLE purchase_items ADD COLUMN IF NOT EXISTS total FLOAT DEFAULT 0;
ALTER TABLE purchase_items DROP COLUMN IF EXISTS price_sale;

-- 1. Buat tabel items_type terlebih dahulu (karena items merujuk ke sini)
CREATE TABLE items_category
(
    id         SERIAL PRIMARY KEY,
    name       TEXT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);

-- 2. Buat tabel items
CREATE TABLE items
(
    id                SERIAL PRIMARY KEY,
    barcode           VARCHAR(150) NOT NULL UNIQUE,
    name              TEXT         NOT NULL,
    items_category_id INTEGER      REFERENCES items_category (id) ON DELETE SET NULL,
    type_unit         VARCHAR(50) DEFAULT 'PCS',
    qty_stock         INTEGER     DEFAULT 0,

    created_by        INTEGER,

    created_at        TIMESTAMPTZ DEFAULT NOW(),
    updated_at        TIMESTAMPTZ DEFAULT NOW(),
    deleted_at        TIMESTAMPTZ DEFAULT NULL
);

-- 3. Buat tabel items_price
CREATE TABLE items_price
(
    id               SERIAL PRIMARY KEY,
    item_id          INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,

    barcode          VARCHAR(150) UNIQUE DEFAULT NULL,
    type_unit        VARCHAR(50)         DEFAULT 'PCS',
    parent_type_unit VARCHAR(50)         DEFAULT NULL,

    price_buy        DOUBLE PRECISION    DEFAULT 0.0,
    price_sell       DOUBLE PRECISION    DEFAULT 0.0,
    parent_id        INTEGER REFERENCES items_price (id), -- Self-referencing (untuk level harga)

    content          INTEGER             DEFAULT 0,
    seq              INTEGER             DEFAULT 0,

    created_at       TIMESTAMPTZ         DEFAULT NOW(),
    updated_at       TIMESTAMPTZ         DEFAULT NOW(),
    deleted_at       TIMESTAMPTZ         DEFAULT NULL
);
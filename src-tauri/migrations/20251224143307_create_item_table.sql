-- 1. Buat tabel items_type terlebih dahulu (karena items merujuk ke sini)
CREATE TABLE items_type
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
    id            SERIAL PRIMARY KEY,
    barcode       TEXT    NOT NULL UNIQUE,
    name          TEXT    NOT NULL,
    items_type_id INTEGER REFERENCES items_type (id) ON DELETE SET NULL,
    created_by    INTEGER,

    created_at    TIMESTAMPTZ DEFAULT NOW(),
    updated_at    TIMESTAMPTZ DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ DEFAULT NULL
);

-- 3. Buat tabel items_price
CREATE TABLE items_price
(
    id         SERIAL PRIMARY KEY,
    item_id    INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,

    barcode    TEXT UNIQUE      DEFAULT NULL,
    type       TEXT             DEFAULT 'PCS',
    price_buy  DOUBLE PRECISION DEFAULT 0.0,
    price_sell DOUBLE PRECISION DEFAULT 0.0,
    parent_id  INTEGER REFERENCES items_price (id), -- Self-referencing (untuk level harga)

    created_at TIMESTAMPTZ      DEFAULT NOW(),
    updated_at TIMESTAMPTZ      DEFAULT NOW(),
    deleted_at TIMESTAMPTZ      DEFAULT NULL
);
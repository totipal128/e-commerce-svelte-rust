-- Add migration script here
CREATE TABLE purchase
(
    id            SERIAL PRIMARY KEY,
    code          VARCHAR(50)                                          DEFAULT NULL,
    suppliers_id  INTEGER REFERENCES suppliers (id) ON DELETE SET NULL DEFAULT NULL,
    total_item    INTEGER                                              DEFAULT 0,
    total         FLOAT                                                DEFAULT 0,

    created_by_id INTEGER REFERENCES users (id) ON DELETE SET NULL     DEFAULT NULL,

    created_at    TIMESTAMPTZ                                          DEFAULT NOW(),
    updated_at    TIMESTAMPTZ                                          DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ                                          DEFAULT NULL
);

CREATE TABLE purchase_items
(
    id          SERIAL PRIMARY KEY,
    purchase_id INTEGER REFERENCES purchase (id) ON DELETE CASCADE,
    items_id    INTEGER REFERENCES items (id) ON DELETE SET NULL,
    price_buy   FLOAT       DEFAULT 0,
    price_sale  FLOAT       DEFAULT 0,
    qty         INTEGER     DEFAULT 0,

    created_at  TIMESTAMPTZ DEFAULT NOW(),
    updated_at  TIMESTAMPTZ DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ DEFAULT NULL
);

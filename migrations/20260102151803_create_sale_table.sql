-- Add migration script here
CREATE TABLE sale
(
    id            SERIAL PRIMARY KEY,
    code          VARCHAR(50)                                         DEFAULT NULL,
    customer_id   INTEGER REFERENCES customer (id) ON DELETE SET NULL DEFAULT NULL,
    ppn           FLOAT                                               DEFAULT 0,
    discount      FLOAT                                               DEFAULT 0,
    total_item    INTEGER                                             DEFAULT 0,
    total         FLOAT                                               DEFAULT 0,
    change        FLOAT                                               DEFAULT 0,
    payment       FLOAT                                               DEFAULT 0,


    created_by_id INTEGER REFERENCES users (id) ON DELETE SET NULL    DEFAULT NULL,

    created_at    TIMESTAMPTZ                                         DEFAULT NOW(),
    updated_at    TIMESTAMPTZ                                         DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ                                         DEFAULT NULL
);

CREATE TABLE sale_items
(
    id          SERIAL PRIMARY KEY,
    sale_id     INTEGER REFERENCES sale (id) ON DELETE CASCADE,
    items_id    INTEGER REFERENCES items (id) ON DELETE SET NULL,
    items_name  VARCHAR(255) DEFAULT NULL,
    items_unit  VARCHAR(50)  DEFAULT NULL,
    items_price FLOAT        DEFAULT 0,
    total       FLOAT        DEFAULT 0,
    qty         INTEGER      DEFAULT 0,

    created_at  TIMESTAMPTZ  DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ  DEFAULT NULL
);

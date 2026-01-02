CREATE TABLE customer
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100)                                     DEFAULT NULL,
    email      VARCHAR(50)                                      DEFAULT NULL,
    address    TEXT                                             DEFAULT NULL,
    no_hp      varchar(20)                                      DEFAULT NULL,

    created_by INTEGER REFERENCES users (id) ON DELETE SET NULL DEFAULT NULL,

    created_at TIMESTAMPTZ                                      DEFAULT NOW(),
    updated_at TIMESTAMPTZ                                      DEFAULT NOW(),
    deleted_at TIMESTAMPTZ                                      DEFAULT NULL
);

CREATE TABLE suppliers
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100)                                     DEFAULT NULL,
    email      VARCHAR(50)                                      DEFAULT NULL,
    address    TEXT                                             DEFAULT NULL,
    no_hp      varchar(20)                                      DEFAULT NULL,

    created_by INTEGER REFERENCES users (id) ON DELETE SET NULL DEFAULT NULL,

    created_at TIMESTAMPTZ                                      DEFAULT NOW(),
    updated_at TIMESTAMPTZ                                      DEFAULT NOW(),
    deleted_at TIMESTAMPTZ                                      DEFAULT NULL
);

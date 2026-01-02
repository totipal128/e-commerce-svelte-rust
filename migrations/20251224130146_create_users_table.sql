-- Add migration script here
CREATE TABLE users
(
    id           SERIAL PRIMARY KEY,
    username     TEXT NOT NULL UNIQUE,
    email        TEXT NOT NULL UNIQUE,
    password     TEXT,
    name         TEXT,
    address      TEXT,
    no_handphone TEXT,
    barcode      TEXT,


    created_at   TIMESTAMPTZ DEFAULT NOW(),
    updated_at   TIMESTAMPTZ DEFAULT NOW(),
    deleted_at   TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE role
(
    id         SERIAL PRIMARY KEY,
    name       TEXT NOT NULL UNIQUE,
    code_name  TEXT,


    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE users_role
(
    users_id   INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    role_id    INTEGER NOT NULL REFERENCES role (id) ON DELETE CASCADE,

    -- Primary Key gabungan agar tidak ada duplikasi relasi yang sama
    PRIMARY KEY (users_id, role_id),

    -- Opsional: mencatat kapan relasi ini dibuat
    created_at TIMESTAMPTZ DEFAULT NOW()
);

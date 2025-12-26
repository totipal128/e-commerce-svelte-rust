# Tauri + SvelteKit

This template should help get you started developing with Tauri and SvelteKit in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

# BACKEND - RUST

## CONNECTION DB

### SQLX - rust => library orm databases rust

- instal sqlx

```bash
cargo install sqlx-cli --features postgres
```

- buat table migration di sqlx rust

```angular2html
sqlx migrate add create_users_table
```

---
contoh create table item and item_type

```angular2html
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
```

---
contoh menambahkan field pada table items

```angular2html
ALTER TABLE items
ADD COLUMN IF NOT EXISTS stoct INT DEFAULT 0;
```

- migrate sqlx

```bash
sqlx migrate run 
```

untuk migrasi ke databases

```bash
sqlx migrate revert
```

untuk membatalkan migrasi terakhir



## started tauri developer
install nvm via brew

```
brew install nvm

# add direktori nvm and add source to .zshrc
mkdir ~/.nvm
echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.zshrc
echo 'source $(brew --prefix nvm)/nvm.sh' >> ~/.zshrc
source ~/.zshrc

```

install node(include npm)
``` bash
nvm install --lts
```
verify instalation node and npm
``` bash
node -v
npm -v
```

install tauri cli
``` bash
npm install --save-dev @tauri-apps/cli
```

try run taury
``` bash
npm run tauri dev
```



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

## IMPLEMTASI CRUD ORM

unuk melakukan implemetasi (create, read, update, delete) dengan
<br>
import data dari module database postgres
[use crate::base::database::postgres::orm::QueryBuilderPostgrest]();

### Example Model User

```angular2html

#[derive(Clone, Default, FromRow, Debug)]
struct User {
pub id: i32,
pub username: String,
pub email: String,
pub password: String,

pub name: Option<String>,
pub address: Option<String>,
pub no_handphone: Option<String>,
pub barcode: Option<String>,

pub created_at: DateTime<Local>,
pub updated_at: DateTime<Local>,
pub deleted_at: Option<DateTime<Local>>,
}

impl Model for User {
const TABLE: &'static str = "users";
const FIELDS: &'static [&'static str] = &[
stringify!(username),
stringify!(email),
stringify!(password),
stringify!(name),
stringify!(address),
stringify!(no_handphone),
stringify!(barcode),
];
}
```

### Read

example read pagination

```angular2html
#[tokio::test]
async fn test_query_read_pagination() {
let results = QueryBuilderPostgrest::<User>::new()
.find_by_pagination(1, 10)
.await;

println!("{:?}", results);
}

```

example read all

```angular2html
#[tokio::test]
async fn test_query_read_all() {
let results = QueryBuilderPostgrest::<User>::new()
.where_clause("username='toti' ")
.find_all()
.await;

println!("{:?}", results);
}

```

example read first terbaru

```angular2html
#[tokio::test]
async fn test_query_one_first() {
let results = QueryBuilderPostgrest::<User>::new().find_one_first().await;

println!("{:?}", results);
}

```

### Create

example

```angular2html
#[tokio::test]
async fn test_query_create() {
let create = QueryBuilderPostgrest::<User>::new()
.values(vec![
"toti",
"toti@ecxample.com",
"password",
"nama toti",
"alamat",
"no_handphone",
"NULL",
])
.create()
.await;

println!("{:?}", create);
}
```

urutan value berdasarkan urutan FIELD dari model impl MODEL user

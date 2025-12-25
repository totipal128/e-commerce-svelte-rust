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

- migrate sqlx

```bash
sqlx migrate run 
```

untuk migrasi ke databases

```bash
sqlx migrate revert
```

untuk membatalkan migrasi terakhir


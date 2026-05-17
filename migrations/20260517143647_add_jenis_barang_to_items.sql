-- Tambahkan tabel jenis_barang jika belum ada
CREATE TABLE IF NOT EXISTS jenis_barang (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);

-- Tambahkan tabel unit_barang jika belum ada
CREATE TABLE IF NOT EXISTS unit_barang (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);

-- Tambahkan foreign key jenis_barang_id ke tabel items
ALTER TABLE items ADD COLUMN IF NOT EXISTS jenis_barang_id INTEGER REFERENCES jenis_barang(id) ON DELETE SET NULL;

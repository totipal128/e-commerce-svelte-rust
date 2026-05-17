# 📝 Product Requirement Document (PRD)
## Sistem Manajemen POS PRO (Tauri v2 + Svelte 5 + Rust + PostgreSQL)

---

## 1. PENDAHULUAN & TUJUAN UTAMA
Aplikasi **POS PRO** adalah sistem kasir (*Point of Sale*) dan manajemen inventaris berbasis desktop yang dirancang menggunakan tumpukan teknologi modern berkinerja tinggi (**Tauri v2**, **Svelte 5**, **Rust**, dan **PostgreSQL**). 

Tujuan dari peningkatan sistem ini adalah meningkatkan integritas penyimpanan data keuangan, mendesentralisasikan wewenang staf ritel menggunakan keamanan berbasis peran (**RBAC**), meningkatkan opsi pembayaran kasir (QRIS & Bank Transfer), serta memberikan antarmuka pengguna kasir yang sangat modern (*premium glassmorphism*) demi efisiensi tinggi.

---

## 2. SPESIFIKASI PENINGKATAN FITUR (CORE FEATURE SPECS)

### 🔑 Fitur 1: Autentikasi JWT & Manajemen Pengguna
* **Deskripsi**: Kasir dan staf logistik harus masuk terlebih dahulu menggunakan kombinasi kredensial yang valid sebelum dapat mengoperasikan mesin kasir.
* **Persyaratan Fungsional**:
  * **Registrasi & Sandi Aman**: Kolom password disimpan dalam bentuk hash terenkripsi searah menggunakan algoritme **Argon2** di database PostgreSQL.
  * **JWT Generation**: Sisi backend Rust memvalidasi kecocokan sandi, kemudian memproduksi string token JWT (*JSON Web Token*) yang menyimpan metadata user (`user_id`, `username`, `role`).
  * **Session Storage**: Svelte 5 frontend menyimpan JWT aktif pada local storage kasir secara aman, dan menyertakan token ini pada setiap pemanggilan Tauri command.
  * **Auto-Logout**: Sesi otomatis kedaluwarsa setelah 12 jam tidak ada aktivitas belanja.

### 👥 Fitur 2: Role Management (RBAC) & Menu Dinamis
* **Deskripsi**: Hak akses halaman dibatasi sesuai dengan fungsi kerja staf agar meminimalkan kebocoran data laba dan manipulasi stok.
* **Pembagian Peran & Hak Akses**:
  
  | Role | Hak Akses Halaman/Fitur | Deskripsi |
  | :--- | :--- | :--- |
  | **Admin** | Seluruh Menu & Fitur | Memiliki wewenang absolut untuk melihat Laba/Rugi, menambah User baru, mengubah harga barang, memproses pembelian, dan penjualan. |
  | **Kasir** | Kasir Penjualan (POS), Riwayat Struk, Pelanggan | Hanya berfokus melayani transaksi pelanggan harian. Menu harga beli barang dan omset toko disembunyikan. |
  | **Gudang** | Data Barang (Items), Pemasok (Supplier), Restock (Purchase) | Staf logistik yang bertanggung jawab menginput barang masuk, mengelola detail unit satuan, dan mencatat pemasok. |

* **Proteksi Akses**:
  * Komponen [Sidebar.svelte](file:///Users/rltmacbook17/Documents/APPLIKASI/rust/desktop/e-commerce-svelte-rust/src/lib/component/Sidebar.svelte) akan membaca role aktif dan menyembunyikan tautan menu yang tidak diizinkan.
  * Router frontend (`+layout.js` atau hooks) akan menolak akses secara paksa jika kasir mencoba memasukkan alamat URL admin secara manual.

### 🛡️ Fitur 3: Implementasi SQLx Transaction (SELESAI 100% ✓)
* **Deskripsi**: Menjamin seluruh data penjualan, detail struk, dan pengurangan stok dieksekusi secara atomic (ACID) agar tidak ada data kasir yang menggantung atau salah hitung.
* **Hasil Implementasi**:
  * Telah dipasang pada repositori Rust [sale.rs](file:///Users/rltmacbook17/Documents/APPLIKASI/rust/desktop/e-commerce-svelte-rust/src-tauri/src/app/master_data/repository/sale.rs) menggunakan penanganan transaksi eksplisit `pool.begin()`.
  * **Skema Rollback**: Jika terjadi kesalahan (seperti stok habis di item kasir ke-4), sistem langsung membatalkan seluruh penyimpanan struk dan penyesuaian stok. Data dijamin konsisten 100%!

### 🛒 Fitur 4: Fitur Kasir Pintar (Smart Search, Auto-Discounts, & Retur)
* **Deskripsi**: Peningkatan fungsionalitas agar antrean pembayaran pelanggan di toko menjadi jauh lebih pendek.
* **Spesifikasi Teknis**:
  * **Smart Search (Pencarian Samar)**: Kasir dapat mencari barang di panel kasir [Sale.svelte](file:///Users/rltmacbook17/Documents/APPLIKASI/rust/desktop/e-commerce-svelte-rust/src/routes/sale/Sale.svelte) tidak hanya berdasarkan scan barcode, melainkan dengan mengetikkan inisial nama barang secara parsial (menggunakan klausa `ILIKE` kustom di Rust).
  * **Diskon Otomatis (Auto-Discount Engine)**:
    * **Diskon Kuantitas**: Pembelian grosir otomatis dipotong (contoh: membeli > 12 Pcs memotong Rp 2.000 per unit).
    * **Diskon Member**: Otomatis memotong Rp X atau persentase Y% jika kasir melink transaksi dengan ID Pelanggan terdaftar.
  * **Modul Retur Barang**:
    * Menu untuk memproses barang yang dikembalikan oleh pembeli.
    * Mengurangi nominal transaksi lama dan otomatis melakukan penambahan stok kembali (*restock*) menggunakan fungsi `increase_stock`.

### 🎨 Fitur 5: UI/UX Premium Kasir Svelte 5 (Glassmorphic)
* **Deskripsi**: Desain ergonomis kelas premium yang memanjakan mata kasir dan memberikan citra modern bagi toko.
* **Persyaratan Visual**:
  * **Glassmorphism**: Panel kasir menggunakan perpaduan warna Slate Gray gelap dengan efek buram transparan (`backdrop-blur-md` dan warna RGB semi-transparan `bg-white/10`) di atas latar belakang estetik.
  * **Micro-Animations**: Hover transisi sehalus 150ms pada tombol kasir dan baris belanjaan. Kilatan cahaya hijau halus (*flash*) pada baris keranjang kasir ketika barang baru berhasil di-scan.
  * **Ergonomi**: Penempatan teks total harga belanja yang sangat besar di sudut kanan atas agar mudah dibaca oleh kasir maupun pembeli yang mengintip ke layar komputer.

### 💳 Fitur 6: Multi-Payment Gateway (Cash, QRIS, Bank Transfer)
* **Deskripsi**: Metode pembayaran bervariasi untuk mengimbangi tren *cashless* di masyarakat.
* **Alur Pembayaran**:
  * **Tunai (Cash)**: Membuka layar kalkulator kembalian otomatis yang sangat responsif.
  * **QRIS (QR Code)**: Menampilkan QR Code dinamis berisi nilai total transaksi yang dibuat secara otomatis, kasir tinggal menunjukkan kode ke pembeli untuk langsung di-scan.
  * **Transfer Bank**: Menampilkan nama bank tujuan transfer, dan kasir dapat mencatatkan nomor referensi transfer bank sebagai bukti audit.

---

## 3. RANCANGAN STRUKTUR TABEL BARU (DATABASE SCHEMA)

Untuk mendukung fitur-fitur di atas, berikut adalah draf perintah migrasi SQLx PostgreSQL:

```sql
-- 1. Membuat tabel hak akses peran (Role Management)
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE, -- 'admin', 'kasir', 'gudang'
    permissions TEXT[] -- daftar menu yang diizinkan, misal: '["pos", "items"]'
);

-- 2. Menghubungkan tabel users dengan roles & password terenkripsi
ALTER TABLE users ADD COLUMN role_id INTEGER REFERENCES roles(id) ON DELETE SET NULL;
ALTER TABLE users ADD COLUMN password_hash TEXT;

-- 3. Menambahkan jenis metode pembayaran pada tabel struk penjualan
ALTER TABLE sale ADD COLUMN payment_method VARCHAR(50) DEFAULT 'CASH'; -- 'CASH', 'QRIS', 'TRANSFER'
ALTER TABLE sale ADD COLUMN payment_reference VARCHAR(100); -- ID transaksi QRIS atau nomor transfer bank

-- 4. Tabel Retur Barang Penjualan
CREATE TABLE sale_returns (
    id SERIAL PRIMARY KEY,
    sale_id INTEGER REFERENCES sale(id) ON DELETE CASCADE,
    returned_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT,
    total_refund NUMERIC(15, 2) NOT NULL,
    created_by_id INTEGER REFERENCES users(id)
);

-- 5. Detail barang yang di-retur oleh pembeli
CREATE TABLE sale_return_items (
    id SERIAL PRIMARY KEY,
    sale_return_id INTEGER REFERENCES sale_returns(id) ON DELETE CASCADE,
    items_id INTEGER REFERENCES items(id),
    qty INTEGER NOT NULL,
    items_unit VARCHAR(50) NOT NULL,
    refund_amount NUMERIC(15, 2) NOT NULL
);
```

---

## 4. PETA JALAN PENGEMBANGAN (DEVELOPMENT ROADMAP)

* **Fase 1: Keamanan & Basis Data (Estimasi 4 Hari)**
  * [x] **Tugas 1.1**: Pasang SQLx Transactions pada `create_sale` (SELESAI ✓).
  * [ ] **Tugas 1.2**: Migrasi tabel `roles` dan modifikasi `users` (Argon2 hash).
  * [ ] **Tugas 1.3**: Pemasangan Token JWT di Backend Rust & Svelte Store.
* **Fase 2: Fitur Kasir Pintar & Multi-Payment (Estimasi 5 Hari)**
  * [ ] **Tugas 2.1**: Pembuatan modul Smart Fuzzy Search dan Diskon Otomatis.
  * [ ] **Tugas 2.2**: Integrasi alur QRIS & Transfer Bank di modal checkout.
  * [ ] **Tugas 2.3**: Pembuatan modul Retur Barang.
* **Fase 3: Pemolesan Desain Visual & Animasi (Estimasi 3 Hari)**
  * [ ] **Tugas 3.1**: Penerapan tema Premium Glassmorphism pada panel kasir.
  * [ ] **Tugas 3.2**: Pengujian transaksi penuh dan penanganan error kasir offline.

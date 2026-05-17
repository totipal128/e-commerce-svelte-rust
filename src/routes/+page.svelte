<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { authStore } from "$lib/store/auth.js";
    import { formatCurrencyIDR } from "$lib/helpers/parse_number_format_currency";

    /** @type {any} */
    let summary = $state(null);
    /** @type {any[]} */
    let lowStockItems = $state([]);
    /** @type {any[]} */
    let monthlySales = $state([]);
    /** @type {any[]} */
    let recentSales = $state([]);
    let loading = $state(true);

    async function fetchDashboard() {
        loading = true;
        try {
            // Filter queries by active User ID if logged in as Kasir (role-based security)
            const uid = $authStore.role === 'kasir' ? ($authStore.user?.id || null) : null;

            // Fetch core statistics
            const [s, l, m] = await Promise.all([
                invoke("dashboard_summary", { userId: uid }),
                invoke("dashboard_low_stock"),
                invoke("dashboard_monthly_sales", { months: 6, userId: uid }),
            ]);
            if (s.success) summary = s.data;
            if (l.success) lowStockItems = l.data;
            if (m.success) monthlySales = m.data;

            // Fetch recent transaction lists
            const r = await invoke("sale_list", {
                filter: {
                    page: 1,
                    page_size: 6
                }
            });
            recentSales = r.results || [];
        } catch (e) {
            console.error("Dashboard fetch error:", e);
        } finally {
            loading = false;
        }
    }

    onMount(fetchDashboard);

    // Calculate maximum bar for charts
    let maxSale = $derived(
        monthlySales.length > 0 ? Math.max(...monthlySales.map(m => m.total)) : 1
    );

    /** @param {number} qty */
    function getStockColor(qty) {
        if (qty <= 0) return "text-red-600 bg-red-50 border border-red-100";
        if (qty <= 5) return "text-orange-600 bg-orange-50 border border-orange-100";
        return "text-yellow-600 bg-yellow-50 border border-yellow-100";
    }

    /** @param {number} qty */
    function getStockBadge(qty) {
        if (qty <= 0) return "Habis";
        if (qty <= 5) return "Kritis";
        return "Menipis";
    }

    /** @param {number} idx */
    function getBarColor(idx) {
        const colors = ["bg-brand", "bg-indigo-500", "bg-purple-500", "bg-pink-500", "bg-sky-500", "bg-emerald-500"];
        return colors[idx % colors.length];
    }

    function today() {
        return new Date().toLocaleDateString("id-ID", { weekday: "long", day: "numeric", month: "long", year: "numeric" });
    }

    /** @param {string} role */
    function getRoleLabel(role) {
        switch (role?.toLowerCase()) {
            case "admin": return "Administrator";
            case "kasir": return "Kasir";
            case "gudang": return "Staf Gudang";
            default: return role || "Staff";
        }
    }

    /** @param {number} days */
    function getExpiryColor(days) {
        if (days < 0) return "text-red-700 bg-red-50 border border-red-200";
        if (days <= 7) return "text-orange-700 bg-orange-50 border border-orange-200";
        if (days <= 15) return "text-amber-700 bg-amber-50 border border-amber-200";
        return "text-green-700 bg-green-50 border border-green-200";
    }
</script>

<svelte:head>
    <title>Dashboard | POS PRO</title>
</svelte:head>

<div class="p-6 space-y-6 min-h-full">

    <!-- Top Header -->
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-2xl font-black text-gray-800 flex items-center gap-2">
                Dashboard
                <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-black bg-brand/10 text-brand border border-brand/20 uppercase tracking-widest">
                    {getRoleLabel($authStore.role)}
                </span>
            </h1>
            <p class="text-sm text-gray-400 mt-1">{today()} · Selamat datang kembali, <span class="font-bold text-gray-700">{$authStore.user?.name || 'Karyawan'}</span>!</p>
        </div>
        <button onclick={fetchDashboard} class="flex items-center gap-2 px-4 py-2.5 bg-white rounded-xl border border-gray-200 text-sm font-bold text-gray-600 hover:bg-gray-50 hover:text-brand transition-all shadow-xs cursor-pointer">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 {loading ? 'animate-spin' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            Refresh Data
        </button>
    </div>

    {#if loading && !summary}
        <!-- Skeleton Loading Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
            {#each [1, 2, 3, 4] as _}
                <div class="bg-white rounded-2xl p-6 shadow-xs animate-pulse border border-gray-50">
                    <div class="h-3 bg-gray-200 rounded w-1/2 mb-4"></div>
                    <div class="h-8 bg-gray-200 rounded w-3/4 mb-2"></div>
                    <div class="h-3 bg-gray-200 rounded w-1/3"></div>
                </div>
            {/each}
        </div>
    {:else if summary}

        <!-- ==================== 1. ADMIN DASHBOARD VIEW ==================== -->
        {#if $authStore.role === 'admin'}
            <!-- KPI Cards Row -->
            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
                
                <!-- Penghasilan Hari Ini -->
                <div class="bg-gradient-to-br from-brand to-brand-strong rounded-2xl p-6 text-white shadow-lg shadow-brand/20 relative overflow-hidden">
                    <div class="absolute -right-4 -top-4 w-24 h-24 bg-white/10 rounded-full animate-pulse"></div>
                    <div class="relative">
                        <div class="flex items-center gap-2 mb-3">
                            <div class="p-2 bg-white/20 rounded-lg">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                            </div>
                            <span class="text-xs font-bold text-white/80 uppercase tracking-wider">Penghasilan Hari Ini</span>
                        </div>
                        <p class="text-2xl font-black mb-1">{formatCurrencyIDR(summary.total_sale_today)}</p>
                        <p class="text-xs text-white/70">{summary.total_transactions_today} transaksi penjualan</p>
                    </div>
                </div>

                <!-- Penghasilan Bulan Ini -->
                <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100 relative overflow-hidden">
                    <div class="absolute -right-3 -top-3 w-20 h-20 bg-purple-50 rounded-full"></div>
                    <div class="relative">
                        <div class="flex items-center gap-2 mb-3">
                            <div class="p-2 bg-purple-100 rounded-lg text-purple-600">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                                </svg>
                            </div>
                            <span class="text-xs font-semibold text-gray-500 uppercase tracking-wider">Penghasilan Bulan Ini</span>
                        </div>
                        <p class="text-2xl font-black text-gray-800 mb-1">{formatCurrencyIDR(summary.total_sale_month)}</p>
                        <p class="text-xs text-gray-400">{summary.total_transactions_month} transaksi total</p>
                    </div>
                </div>

                <!-- Total Fisik Stok Gudang -->
                <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100 relative overflow-hidden">
                    <div class="absolute -right-3 -top-3 w-20 h-20 bg-emerald-50 rounded-full"></div>
                    <div class="relative">
                        <div class="flex items-center gap-2 mb-3">
                            <div class="p-2 bg-emerald-100 rounded-lg text-emerald-600">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                                </svg>
                            </div>
                            <span class="text-xs font-semibold text-gray-500 uppercase tracking-wider">Total Stok Barang</span>
                        </div>
                        <p class="text-2xl font-black text-emerald-700 mb-1">{summary.total_stock_qty || 0} <span class="text-sm font-medium text-gray-400">Unit</span></p>
                        <p class="text-xs text-gray-400">Tersebar di {summary.total_items} jenis produk</p>
                    </div>
                </div>

                <!-- Expired Barang Alerts (Admin view) -->
                <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100 relative overflow-hidden">
                    <div class="absolute -right-3 -top-3 w-20 h-20 bg-rose-50 rounded-full"></div>
                    <div class="relative">
                        <div class="flex items-center gap-2 mb-3">
                            <div class="p-2 bg-rose-100 rounded-lg text-rose-600">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                            </div>
                            <span class="text-xs font-semibold text-gray-500 uppercase tracking-wider">Barang Kadaluarsa</span>
                        </div>
                        <p class="text-2xl font-black text-red-600 mb-1">
                            {summary.expired_items?.filter(/** @param {any} i */ i => i.days_remaining <= 0).length || 0}
                            <span class="text-xs font-bold text-gray-400"> Item</span>
                        </p>
                        <p class="text-xs text-gray-400">
                            {summary.expired_items?.filter(/** @param {any} i */ i => i.days_remaining > 0 && i.days_remaining <= 15).length || 0} barang akan kadaluarsa
                        </p>
                    </div>
                </div>
            </div>

            <!-- Dashboard Analytics Grid -->
            <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">

                <!-- Monthly Revenue Chart -->
                <div class="xl:col-span-2 bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                    <div class="flex items-center justify-between mb-6">
                        <div>
                            <h2 class="font-black text-gray-800">Grafik Penghasilan Toko</h2>
                            <p class="text-xs text-gray-400">Performa penjualan bulanan</p>
                        </div>
                        <div class="px-3 py-1.5 bg-brand/10 text-brand text-xs font-bold rounded-full uppercase tracking-wider">Bulanan</div>
                    </div>

                    {#if monthlySales.length === 0}
                        <div class="flex flex-col items-center justify-center h-56 text-gray-300">
                            <svg class="h-12 w-12 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2m0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                            </svg>
                            <p class="text-sm">Belum ada transaksi tersimpan</p>
                        </div>
                    {:else}
                        <div class="flex items-end gap-4 h-56 pt-6">
                            {#each monthlySales as sale, i}
                                <div class="flex-1 flex flex-col items-center gap-2 group">
                                    <div class="relative w-full flex items-end justify-center" style="height: 160px">
                                        <div
                                            class="{getBarColor(i)} w-full rounded-t-xl transition-all duration-700 cursor-pointer relative group-hover:opacity-90"
                                            style="height: {Math.max(10, (sale.total / maxSale) * 100)}%"
                                        >
                                            <!-- Tooltip bubble -->
                                            <div class="absolute -top-16 left-1/2 -translate-x-1/2 bg-gray-800 text-white text-[10px] font-bold p-2 rounded-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10 shadow-lg">
                                                {formatCurrencyIDR(sale.total)}
                                                <br/><span class="font-normal text-gray-300">{sale.transactions} transaksi</span>
                                                <div class="absolute top-full left-1/2 -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-800"></div>
                                            </div>
                                        </div>
                                    </div>
                                    <p class="text-[10px] text-gray-400 font-bold tracking-tight text-center">{sale.month}</p>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <!-- Total Penghasilan Setiap Kasir (Revenue per Cashier) -->
                <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                    <h2 class="font-black text-gray-800 mb-1">Penghasilan Setiap Kasir</h2>
                    <p class="text-xs text-gray-400 mb-5">Kontribusi omset penjualan dari tim kasir</p>

                    {#if !summary.cashier_revenues || summary.cashier_revenues.length === 0}
                        <div class="flex flex-col items-center justify-center h-56 text-gray-300">
                            <svg class="h-10 w-10 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                            </svg>
                            <p class="text-xs">Tidak ada data kontribusi kasir</p>
                        </div>
                    {:else}
                        <div class="space-y-4 max-h-[240px] overflow-y-auto pr-1">
                            {#each summary.cashier_revenues as cr}
                                <div class="flex items-center justify-between p-3 border border-gray-50 rounded-xl bg-gray-50/50 hover:bg-gray-50 transition-all">
                                    <div class="flex items-center gap-3">
                                        <div class="h-8 w-8 rounded-full bg-brand/10 text-brand flex items-center justify-center font-bold text-xs">
                                            {cr.cashier_name.charAt(0).toUpperCase()}
                                        </div>
                                        <div>
                                            <p class="text-xs font-bold text-gray-800">{cr.cashier_name}</p>
                                            <p class="text-[10px] text-gray-400">Petugas Shift Aktif</p>
                                        </div>
                                    </div>
                                    <span class="text-xs font-black text-brand">{formatCurrencyIDR(cr.total_sales)}</span>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>
            </div>

            <!-- Lower Ledger: Expired Goods & Inventory Limits -->
            <div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
                <!-- Expired Goods Alert List -->
                <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                    <h2 class="font-black text-gray-800 mb-1 flex items-center gap-2">
                        <svg class="w-5 h-5 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                        </svg>
                        Pengawasan Tanggal Kadaluarsa
                    </h2>
                    <p class="text-xs text-gray-400 mb-5">Daftar produk expired & rawan mendekati masa tenggang</p>

                    {#if !summary.expired_items || summary.expired_items.length === 0}
                        <div class="flex flex-col items-center justify-center h-48 text-gray-300">
                            <p class="text-xs font-medium">Tidak ada rincian log barang kadaluarsa</p>
                        </div>
                    {:else}
                        <div class="space-y-2.5 max-h-[300px] overflow-y-auto">
                            {#each summary.expired_items as item}
                                <div class="flex items-center justify-between p-3 rounded-xl hover:bg-gray-50 border border-gray-50 transition-colors">
                                    <div class="min-w-0">
                                        <p class="text-xs font-bold text-gray-800 truncate">{item.name}</p>
                                        <p class="text-[10px] text-gray-400">Barcode: {item.barcode || 'N/A'} · Qty: {item.qty} unit</p>
                                    </div>
                                    <span class="text-[10px] font-bold px-3 py-1 rounded-full {getExpiryColor(item.days_remaining)}">
                                        {item.days_remaining < 0 ? 'Kadaluarsa' : `${item.days_remaining} Hari Lagi`}
                                    </span>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <!-- Limit Stock Items Ledger -->
                <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                    <div class="flex items-center justify-between mb-1">
                        <h2 class="font-black text-gray-800">Limit Persediaan Stok Barang</h2>
                        {#if summary.low_stock_count > 0}
                            <span class="h-6 w-6 bg-orange-500 text-white text-xs font-black rounded-full flex items-center justify-center animate-pulse">{summary.low_stock_count}</span>
                        {/if}
                    </div>
                    <p class="text-xs text-gray-400 mb-5">Katalog barang menipis di bawah 10 unit</p>

                    {#if lowStockItems.length === 0}
                        <div class="flex flex-col items-center justify-center h-48 text-gray-300">
                            <svg class="h-10 w-10 mb-2 text-gray-200" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <p class="text-xs font-semibold">Semua persediaan barang stabil ✓</p>
                        </div>
                    {:else}
                        <div class="space-y-2.5 max-h-[300px] overflow-y-auto pr-1">
                            {#each lowStockItems as item}
                                <div class="flex items-center gap-3 p-3 rounded-xl hover:bg-gray-50 border border-gray-50 transition-colors">
                                    <div class="h-9 w-9 rounded-xl {getStockColor(item.qty_stock)} flex items-center justify-center text-xs font-black shrink-0">
                                        {item.qty_stock}
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <p class="text-xs font-bold text-gray-800 truncate">{item.name}</p>
                                        <p class="text-[10px] text-gray-400">{item.type_unit || 'Unit'} · Barcode: {item.barcode || 'N/A'}</p>
                                    </div>
                                    <span class="text-[10px] font-black px-2 py-0.5 rounded-full {getStockColor(item.qty_stock)}">{getStockBadge(item.qty_stock)}</span>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>
            </div>

        <!-- ==================== 2. KASIR DASHBOARD VIEW ==================== -->
        {:else if $authStore.role === 'kasir'}
            <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">
                
                <!-- Quick Cashier Action Panel (2 Columns) -->
                <div class="xl:col-span-2 flex flex-col gap-6">
                    
                    <!-- Today Sales Big Glass Card -->
                    <div class="bg-gradient-to-br from-emerald-600 to-teal-700 rounded-3xl p-8 text-white shadow-xl shadow-teal-700/20 relative overflow-hidden">
                        <div class="absolute -right-6 -bottom-6 w-40 h-40 bg-white/10 rounded-full blur-xl animate-pulse"></div>
                        <div class="absolute right-10 top-10 w-24 h-24 bg-white/5 rounded-full blur-md"></div>
                        
                        <div class="relative flex flex-col gap-6">
                            <div>
                                <span class="text-xs font-black uppercase tracking-widest bg-white/20 px-3.5 py-1.5 rounded-full">KONTRIBUSI SHIFT ANDA</span>
                                <h2 class="text-4xl font-black mt-4">{formatCurrencyIDR(summary.total_sale_today)}</h2>
                                <p class="text-sm text-emerald-100 mt-2">Penghasilan total penjualan Anda sepanjang shift hari ini.</p>
                            </div>
                            
                            <div class="grid grid-cols-2 gap-4 pt-6 border-t border-white/10">
                                <div>
                                    <p class="text-xs text-emerald-200">Jumlah Transaksi</p>
                                    <p class="text-xl font-bold">{summary.total_transactions_today} Penjualan</p>
                                </div>
                                <div>
                                    <p class="text-xs text-emerald-200">Total Penghasilan Bulan Ini</p>
                                    <p class="text-xl font-bold">{formatCurrencyIDR(summary.total_sale_month)}</p>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Personal Sales Progress Chart -->
                    <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                        <h2 class="font-black text-gray-800 mb-1">Grafik Penghasilan Anda</h2>
                        <p class="text-xs text-gray-400 mb-6">Omset bulanan khusus transaksi yang Anda layani</p>

                        {#if monthlySales.length === 0}
                            <div class="flex flex-col items-center justify-center h-40 text-gray-300">
                                <p class="text-xs font-semibold">Belum ada riwayat input penjualan</p>
                            </div>
                        {:else}
                            <div class="flex items-end gap-3 h-40 pt-4">
                                {#each monthlySales as sale, i}
                                    <div class="flex-1 flex flex-col items-center gap-1.5 group">
                                        <div class="relative w-full flex items-end justify-center" style="height: 100px">
                                            <div
                                                class="bg-emerald-500 w-full rounded-t-lg transition-all duration-500 cursor-pointer relative group-hover:opacity-90"
                                                style="height: {Math.max(12, (sale.total / maxSale) * 100)}%"
                                            >
                                                <div class="absolute -top-12 left-1/2 -translate-x-1/2 bg-gray-800 text-white text-[9px] font-bold px-2 py-1 rounded shadow-md opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
                                                    {formatCurrencyIDR(sale.total)}
                                                </div>
                                            </div>
                                        </div>
                                        <p class="text-[9px] text-gray-400 font-bold">{sale.month}</p>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>

                    <!-- Quick Navigation Cards -->
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <a href="/sale" class="flex items-center justify-between p-6 bg-white border border-gray-100 hover:border-brand rounded-2xl shadow-xs hover:shadow-md transition-all group">
                            <div class="flex items-center gap-4">
                                <div class="p-4 bg-brand/10 text-brand rounded-2xl group-hover:bg-brand group-hover:text-white transition-colors">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="font-black text-gray-800 group-hover:text-brand transition-colors">Buka Layar Kasir</h3>
                                    <p class="text-xs text-gray-400 mt-0.5">Input scanner / transaksi POS baru</p>
                                </div>
                            </div>
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-300 group-hover:text-brand group-hover:translate-x-1 transition-all" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                            </svg>
                        </a>

                        <a href="/pelanggan" class="flex items-center justify-between p-6 bg-white border border-gray-100 hover:border-purple-500 rounded-2xl shadow-xs hover:shadow-md transition-all group">
                            <div class="flex items-center gap-4">
                                <div class="p-4 bg-purple-50 text-purple-600 rounded-2xl group-hover:bg-purple-500 group-hover:text-white transition-colors">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="font-black text-gray-800 group-hover:text-purple-600 transition-colors">Daftar Pelanggan</h3>
                                    <p class="text-xs text-gray-400 mt-0.5">Database member & customer POS</p>
                                </div>
                            </div>
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-300 group-hover:text-purple-500 group-hover:translate-x-1 transition-all" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                            </svg>
                        </a>
                    </div>
                </div>

                <!-- Right Side: Limit Stock & Riwayat lists -->
                <div class="space-y-6">
                    <!-- Low Stock Alert (Cashier Warning) -->
                    <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="font-black text-gray-800">Alert Stok Kritis!</h2>
                            <span class="h-6 w-6 bg-red-100 text-red-600 text-xs font-black rounded-full flex items-center justify-center">{summary.low_stock_count}</span>
                        </div>

                        {#if lowStockItems.length === 0}
                            <div class="flex flex-col items-center justify-center h-48 text-gray-300">
                                <p class="text-xs">Stok aman ✓</p>
                            </div>
                        {:else}
                            <div class="space-y-2 max-h-[220px] overflow-y-auto">
                                {#each lowStockItems.slice(0, 5) as item}
                                    <div class="flex items-center gap-3 p-2.5 rounded-xl bg-red-50/50 border border-red-100/30">
                                        <div class="h-8 w-8 rounded-lg bg-red-50 text-red-600 flex items-center justify-center text-xs font-black shrink-0">
                                            {item.qty_stock}
                                        </div>
                                        <div class="min-w-0 flex-1">
                                            <p class="text-xs font-bold text-gray-800 truncate">{item.name}</p>
                                            <p class="text-[9px] text-gray-400">Barcode: {item.barcode || 'N/A'}</p>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>

                    <!-- Recent Sales Shift Ledger -->
                    <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                        <h2 class="font-black text-gray-800 mb-4 flex items-center gap-2">
                            <svg class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            Aktivitas Penjualan Terkini
                        </h2>

                        {#if recentSales.length === 0}
                            <div class="flex flex-col items-center justify-center h-48 text-gray-300">
                                <p class="text-xs">Belum ada transaksi shift ini</p>
                            </div>
                        {:else}
                            <div class="space-y-3 max-h-[300px] overflow-y-auto">
                                {#each recentSales as sale}
                                    <div class="p-3 border border-gray-50 rounded-xl bg-gray-50/50 flex flex-col gap-1 hover:bg-gray-50 transition-colors">
                                        <div class="flex items-center justify-between">
                                            <span class="text-xs font-bold text-gray-700">{sale.code}</span>
                                            <span class="text-[10px] font-black bg-teal-50 text-teal-700 px-2 py-0.5 rounded-full">Selesai</span>
                                        </div>
                                        <div class="flex items-center justify-between text-xs text-gray-400">
                                            <span>Total: <strong class="text-gray-800">{formatCurrencyIDR(sale.total)}</strong></span>
                                            <span>{new Date(sale.created_at).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' })}</span>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>
                </div>
            </div>

        <!-- ==================== 3. GUDANG DASHBOARD VIEW ==================== -->
        {:else if $authStore.role === 'gudang'}
            <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">

                <!-- Left Panel: Warehouse Ledger & Logistics shortcuts -->
                <div class="xl:col-span-2 flex flex-col gap-6">
                    
                    <!-- Stats Grid -->
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100 flex items-center gap-4">
                            <div class="p-4 bg-orange-100 text-orange-600 rounded-2xl">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-xs text-gray-400 font-bold uppercase tracking-wider">Persediaan Kritis (Limit)</p>
                                <p class="text-3xl font-black text-gray-800 mt-1">{summary.low_stock_count} <span class="text-sm font-semibold text-gray-400">item</span></p>
                            </div>
                        </div>

                        <div class="bg-white rounded-2xl p-6 shadow-xs border border-gray-100 flex items-center gap-4">
                            <div class="p-4 bg-emerald-100 text-emerald-600 rounded-2xl">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-xs text-gray-400 font-bold uppercase tracking-wider">Total Volume Stok Fisik</p>
                                <p class="text-3xl font-black text-emerald-700 mt-1">{summary.total_stock_qty || 0} <span class="text-sm font-semibold text-gray-400">Unit</span></p>
                            </div>
                        </div>
                    </div>

                    <!-- Logistics Actions Grid -->
                    <div class="bg-white rounded-2xl p-6 border border-gray-100 shadow-xs">
                        <h3 class="font-black text-gray-800 mb-4 flex items-center gap-2">
                            <svg class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                            </svg>
                            Tindakan Logistik & Restock
                        </h3>

                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <a href="/purchase" class="flex flex-col gap-2 p-5 border border-gray-100 rounded-2xl hover:border-brand hover:bg-brand/5 transition-all text-center">
                                <span class="text-sm font-black text-gray-800">Restock Supplier</span>
                                <span class="text-[10px] text-gray-400 leading-normal">Buka pembelian & input pasokan logistik</span>
                            </a>

                            <a href="/items" class="flex flex-col gap-2 p-5 border border-gray-100 rounded-2xl hover:border-purple-500 hover:bg-purple-50/5 transition-all text-center">
                                <span class="text-sm font-black text-gray-800">Ubah Katalog Barang</span>
                                <span class="text-[10px] text-gray-400 leading-normal">Definisikan harga beli, harga jual, & unit baru</span>
                            </a>

                            <a href="/supplier" class="flex flex-col gap-2 p-5 border border-gray-100 rounded-2xl hover:border-orange-500 hover:bg-orange-50/5 transition-all text-center">
                                <span class="text-sm font-black text-gray-800">Kelola Supplier</span>
                                <span class="text-[10px] text-gray-400 leading-normal">Daftar vendor aktif & kontak penanggung jawab</span>
                            </a>
                        </div>
                    </div>

                    <!-- Soon-to-expire Items Tracker for Warehouse -->
                    <div class="bg-white rounded-2xl p-6 border border-gray-100 shadow-xs">
                        <h3 class="font-black text-gray-800 mb-4 flex items-center gap-2">
                            <svg class="h-5 w-5 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            Batas Kedaluwarsa Produk (Warehouse Stock)
                        </h3>

                        {#if !summary.expired_items || summary.expired_items.length === 0}
                            <div class="text-center py-6 text-gray-300">Tidak ada pencatatan produk kadaluarsa</div>
                        {:else}
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                {#each summary.expired_items as item}
                                    <div class="p-4 border border-gray-100 rounded-xl bg-gray-50/30 flex justify-between items-center gap-3">
                                        <div class="min-w-0">
                                            <p class="text-xs font-bold text-gray-800 truncate">{item.name}</p>
                                            <p class="text-[10px] text-gray-400">Barcode: {item.barcode || 'N/A'} · Qty: {item.qty} unit</p>
                                        </div>
                                        <span class="text-[10px] font-black px-2.5 py-1 rounded-full {getExpiryColor(item.days_remaining)} shrink-0">
                                            {item.days_remaining < 0 ? 'Kadaluarsa!' : `${item.days_remaining} Hari`}
                                        </span>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>

                </div>

                <!-- Right Column: Urgent Restock Alerts -->
                <div class="xl:col-span-1 bg-white rounded-2xl p-6 shadow-xs border border-gray-100">
                    <h2 class="font-black text-gray-800 mb-1 flex items-center gap-2">
                        <svg class="h-5 w-5 text-orange-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                        </svg>
                        Limit Stok Gudang (Kritis)
                    </h2>
                    <p class="text-xs text-gray-400 mb-5">Harap segera buat pemesanan logistik restock ke supplier</p>

                    {#if lowStockItems.length === 0}
                        <div class="flex flex-col items-center justify-center h-64 text-gray-300">
                            <p class="text-xs font-bold text-gray-400">Seluruh stok barang aman ✓</p>
                        </div>
                    {:else}
                        <div class="space-y-3 max-h-[480px] overflow-y-auto pr-1">
                            {#each lowStockItems as item}
                                <div class="p-3.5 border border-red-50 rounded-xl bg-red-50/10 flex items-center justify-between gap-3">
                                    <div class="min-w-0">
                                        <p class="text-xs font-bold text-gray-800 truncate">{item.name}</p>
                                        <p class="text-[10px] text-gray-400">SKU: {item.barcode || 'N/A'}</p>
                                    </div>
                                    <span class="text-xs font-black px-2.5 py-1 rounded-full shrink-0 {getStockColor(item.qty_stock)}">
                                        {item.qty_stock} {item.type_unit || 'Unit'}
                                    </span>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

            </div>
        {/if}

    {/if}
</div>
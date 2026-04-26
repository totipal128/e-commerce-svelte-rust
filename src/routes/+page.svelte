<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { formatCurrencyIDR } from "$lib/helpers/parse_number_format_currency.ts";

    let summary = $state(null);
    let lowStockItems = $state([]);
    let monthlySales = $state([]);
    let loading = $state(true);

    async function fetchDashboard() {
        loading = true;
        try {
            const [s, l, m] = await Promise.all([
                invoke("dashboard_summary"),
                invoke("dashboard_low_stock"),
                invoke("dashboard_monthly_sales", { months: 6 }),
            ]);
            if (s.success) summary = s.data;
            if (l.success) lowStockItems = l.data;
            if (m.success) monthlySales = m.data;
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    onMount(fetchDashboard);

    // Calculate max for bar chart
    let maxSale = $derived(
        monthlySales.length > 0 ? Math.max(...monthlySales.map(m => m.total)) : 1
    );

    function getStockColor(qty) {
        if (qty <= 0) return "text-red-600 bg-red-50";
        if (qty <= 5) return "text-orange-600 bg-orange-50";
        return "text-yellow-600 bg-yellow-50";
    }
    function getStockBadge(qty) {
        if (qty <= 0) return "Habis";
        if (qty <= 5) return "Kritis";
        return "Menipis";
    }
    function getBarColor(idx) {
        const colors = ["bg-brand", "bg-purple-500", "bg-pink-500", "bg-orange-500", "bg-teal-500", "bg-blue-500"];
        return colors[idx % colors.length];
    }
    function today() {
        return new Date().toLocaleDateString("id-ID", { weekday: "long", day: "numeric", month: "long", year: "numeric" });
    }
</script>

<svelte:head>
    <title>Dashboard | POS PRO</title>
</svelte:head>

<div class="p-6 space-y-6 min-h-full">

    <!-- Header -->
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-2xl font-black text-gray-800">Dashboard</h1>
            <p class="text-sm text-gray-400 mt-1">{today()}</p>
        </div>
        <button onclick={fetchDashboard} class="flex items-center gap-2 px-4 py-2.5 bg-white rounded-xl border border-gray-200 text-sm font-bold text-gray-600 hover:bg-gray-50 hover:text-brand transition-all shadow-sm">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 {loading ? 'animate-spin' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            Refresh
        </button>
    </div>

    {#if loading && !summary}
        <!-- Skeleton Loading -->
        <div class="grid grid-cols-2 xl:grid-cols-4 gap-4">
            {#each [1,2,3,4] as _}
                <div class="bg-white rounded-2xl p-6 shadow-sm animate-pulse">
                    <div class="h-3 bg-gray-200 rounded w-1/2 mb-4"></div>
                    <div class="h-8 bg-gray-200 rounded w-3/4 mb-2"></div>
                    <div class="h-3 bg-gray-200 rounded w-1/3"></div>
                </div>
            {/each}
        </div>
    {:else if summary}

        <!-- KPI Cards -->
        <div class="grid grid-cols-2 xl:grid-cols-4 gap-4">

            <!-- Penjualan Hari Ini -->
            <div class="bg-gradient-to-br from-brand to-brand-strong rounded-2xl p-6 text-white shadow-lg shadow-brand/20 relative overflow-hidden">
                <div class="absolute -right-4 -top-4 w-24 h-24 bg-white/10 rounded-full"></div>
                <div class="absolute -right-2 -bottom-6 w-20 h-20 bg-white/5 rounded-full"></div>
                <div class="relative">
                    <div class="flex items-center gap-2 mb-3">
                        <div class="p-2 bg-white/20 rounded-lg">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </div>
                        <span class="text-sm font-semibold text-white/80">Penjualan Hari Ini</span>
                    </div>
                    <p class="text-2xl font-black mb-1">{formatCurrencyIDR(summary.total_sale_today)}</p>
                    <p class="text-xs text-white/70">{summary.total_transactions_today} transaksi</p>
                </div>
            </div>

            <!-- Penjualan Bulan Ini -->
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-100 relative overflow-hidden">
                <div class="absolute -right-3 -top-3 w-20 h-20 bg-purple-50 rounded-full"></div>
                <div class="relative">
                    <div class="flex items-center gap-2 mb-3">
                        <div class="p-2 bg-purple-100 rounded-lg text-purple-600">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                            </svg>
                        </div>
                        <span class="text-sm font-semibold text-gray-500">Penjualan Bulan Ini</span>
                    </div>
                    <p class="text-2xl font-black text-gray-800 mb-1">{formatCurrencyIDR(summary.total_sale_month)}</p>
                    <p class="text-xs text-gray-400">{summary.total_transactions_month} transaksi</p>
                </div>
            </div>

            <!-- Keuntungan Bulan Ini -->
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-100 relative overflow-hidden">
                <div class="absolute -right-3 -top-3 w-20 h-20 bg-green-50 rounded-full"></div>
                <div class="relative">
                    <div class="flex items-center gap-2 mb-3">
                        <div class="p-2 bg-green-100 rounded-lg text-green-600">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
                            </svg>
                        </div>
                        <span class="text-sm font-semibold text-gray-500">Keuntungan Bulan Ini</span>
                    </div>
                    <p class="text-2xl font-black mb-1 {summary.profit_month >= 0 ? 'text-green-600' : 'text-red-600'}">{formatCurrencyIDR(summary.profit_month)}</p>
                    <p class="text-xs text-gray-400">Omset - Modal</p>
                </div>
            </div>

            <!-- Stok Menipis -->
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-100 relative overflow-hidden {summary.low_stock_count > 0 ? 'border-orange-200' : ''}">
                <div class="absolute -right-3 -top-3 w-20 h-20 {summary.low_stock_count > 0 ? 'bg-orange-50' : 'bg-teal-50'} rounded-full"></div>
                <div class="relative">
                    <div class="flex items-center gap-2 mb-3">
                        <div class="p-2 {summary.low_stock_count > 0 ? 'bg-orange-100 text-orange-600' : 'bg-teal-100 text-teal-600'} rounded-lg">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                            </svg>
                        </div>
                        <span class="text-sm font-semibold text-gray-500">Stok Menipis</span>
                    </div>
                    <p class="text-2xl font-black text-gray-800 mb-1">{summary.low_stock_count} <span class="text-sm font-medium text-gray-400">barang</span></p>
                    <p class="text-xs text-gray-400">Total {summary.total_items} jenis barang</p>
                </div>
            </div>
        </div>

        <!-- Charts & Low Stock Section -->
        <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">

            <!-- Monthly Sales Bar Chart -->
            <div class="xl:col-span-2 bg-white rounded-2xl p-6 shadow-sm border border-gray-100">
                <div class="flex items-center justify-between mb-6">
                    <div>
                        <h2 class="font-black text-gray-800">Grafik Penjualan</h2>
                        <p class="text-xs text-gray-400">6 bulan terakhir</p>
                    </div>
                    <div class="px-3 py-1.5 bg-brand/10 text-brand text-xs font-bold rounded-full">Bulanan</div>
                </div>

                {#if monthlySales.length === 0}
                    <div class="flex flex-col items-center justify-center h-48 text-gray-300">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                        </svg>
                        <p class="text-sm">Belum ada data penjualan</p>
                    </div>
                {:else}
                    <div class="flex items-end gap-3 h-48">
                        {#each monthlySales as sale, i}
                            <div class="flex-1 flex flex-col items-center gap-1 group">
                                <div class="relative w-full flex items-end justify-center" style="height: 160px">
                                    <div
                                        class="{getBarColor(i)} w-full rounded-t-xl transition-all duration-700 cursor-pointer relative group-hover:opacity-90"
                                        style="height: {Math.max(8, (sale.total / maxSale) * 100)}%"
                                    >
                                        <!-- Tooltip -->
                                        <div class="absolute -top-14 left-1/2 -translate-x-1/2 bg-gray-800 text-white text-[10px] font-bold px-2 py-1.5 rounded-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
                                            {formatCurrencyIDR(sale.total)}
                                            <br/><span class="font-normal text-gray-300">{sale.transactions} transaksi</span>
                                            <div class="absolute top-full left-1/2 -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-800"></div>
                                        </div>
                                    </div>
                                </div>
                                <p class="text-[10px] text-gray-400 font-medium text-center">{sale.month}</p>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>

            <!-- Low Stock Items -->
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-100">
                <div class="flex items-center justify-between mb-5">
                    <div>
                        <h2 class="font-black text-gray-800">Limit Stok Barang</h2>
                        <p class="text-xs text-gray-400">Stok di bawah 10 unit</p>
                    </div>
                    {#if summary.low_stock_count > 0}
                        <span class="h-6 w-6 bg-red-500 text-white text-xs font-black rounded-full flex items-center justify-center animate-pulse">{summary.low_stock_count}</span>
                    {/if}
                </div>

                {#if lowStockItems.length === 0}
                    <div class="flex flex-col items-center justify-center h-48 text-gray-300">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <p class="text-sm">Semua stok aman ✓</p>
                    </div>
                {:else}
                    <div class="space-y-2 max-h-64 overflow-y-auto pr-1">
                        {#each lowStockItems as item}
                            <div class="flex items-center gap-3 p-2.5 rounded-xl hover:bg-gray-50 transition-colors">
                                <div class="h-9 w-9 rounded-xl {getStockColor(item.qty_stock)} flex items-center justify-center text-sm font-black shrink-0">
                                    {item.qty_stock}
                                </div>
                                <div class="flex-1 min-w-0">
                                    <p class="text-sm font-bold text-gray-800 truncate">{item.name}</p>
                                    <p class="text-xs text-gray-400">{item.type_unit || '-'} · {item.barcode || 'no barcode'}</p>
                                </div>
                                <span class="text-[10px] font-black px-2 py-0.5 rounded-full {getStockColor(item.qty_stock)}">{getStockBadge(item.qty_stock)}</span>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>
        </div>

        <!-- Bottom Stats Row -->
        <div class="grid grid-cols-2 xl:grid-cols-3 gap-4">
            <div class="bg-white rounded-2xl p-5 shadow-sm border border-gray-100 flex items-center gap-4">
                <div class="p-3 bg-blue-100 text-blue-600 rounded-xl">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                    </svg>
                </div>
                <div>
                    <p class="text-xs text-gray-400 font-medium">Total Jenis Barang</p>
                    <p class="text-2xl font-black text-gray-800">{summary.total_items}</p>
                </div>
            </div>
            <div class="bg-white rounded-2xl p-5 shadow-sm border border-gray-100 flex items-center gap-4">
                <div class="p-3 bg-pink-100 text-pink-600 rounded-xl">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                </div>
                <div>
                    <p class="text-xs text-gray-400 font-medium">Total Pembelian Bulan Ini</p>
                    <p class="text-xl font-black text-gray-800">{formatCurrencyIDR(summary.total_purchase_month)}</p>
                </div>
            </div>
            <div class="bg-white rounded-2xl p-5 shadow-sm border border-gray-100 flex items-center gap-4 col-span-2 xl:col-span-1">
                <div class="p-3 bg-yellow-100 text-yellow-600 rounded-xl">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                    </svg>
                </div>
                <div>
                    <p class="text-xs text-gray-400 font-medium">Barang Stok Menipis</p>
                    <p class="text-2xl font-black {summary.low_stock_count > 0 ? 'text-orange-500' : 'text-green-600'}">{summary.low_stock_count} Barang</p>
                </div>
            </div>
        </div>
    {/if}
</div>
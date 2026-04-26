<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, tick } from "svelte";
    import Table from "$lib/component/table/Table.svelte";
    import Purchase from './Purchase.svelte';
    import { showToast } from "$lib/store/toast.js";

    let data = $state([]);
    let loading = $state(true);
    let searchDb = $state("");
    
    // Pagination state
    let currentPage = $state(1);
    let totalPages = $state(1);
    let totalItems = $state(0);
    let pageSize = $state(10);

    let headerTable = [
        { name: 'Tanggal', value: 'created_at', type_data: 'date' },
        { name: 'Kode', value: 'code' },
        { name: 'Supplier', value: 'supplier_name' },
        { name: 'Total Item', value: 'total_item' },
        { name: 'Total Harga', value: 'total', type_data: 'parseIDR' },
        { name: 'Dibayar', value: 'payment', type_data: 'parseIDR' },
    ];

    async function fetchData() {
        loading = true;
        try {
            const result = await invoke("purchase_list", {
                filter: {
                    search: searchDb,
                    page: currentPage,
                    page_size: pageSize
                }
            });

            data = result.results;
            totalPages = result.total_page;
            totalItems = result.count;
            currentPage = result.page;
        } catch (err) {
            console.error(err);
            showToast("Gagal mengambil data pembelian", "error");
        } finally {
            loading = false;
        }
    }

    function handlePageChange(newPage) {
        currentPage = newPage;
        fetchData();
    }

    onMount(fetchData);

    let openModalAdd = $state(false);
    let openModalRemove = $state(false);
    let idData = $state(0);

    function handlerAdd() {
        openModalAdd = !openModalAdd;
        fetchData();
    }

    function handleSearch(e) {
        if (e.key === "Enter") {
            currentPage = 1;
            fetchData();
        }
    }
</script>

<div class="w-full">
    {#if loading && data.length === 0}
        <div class="flex w-full h-[80vh] bg-white rounded-2xl justify-center items-center shadow-sm">
            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-brand"></div>
        </div>
    {:else}
        <div class="flex justify-between p-4 bg-white/50 backdrop-blur-md sticky top-0 z-20">
            <div class="flex gap-4 items-center">
                <h1 class="text-2xl font-black text-gray-800">Daftar Pembelian</h1>
                <button onclick={handlerAdd} class="bg-brand text-white px-6 py-2.5 rounded-xl font-bold hover:bg-brand-strong shadow-lg shadow-brand/20 transition-all flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" /></svg>
                    Restock Barang
                </button>
            </div>

            <div class="relative w-96">
                <input
                    bind:value={searchDb}
                    onkeydown={handleSearch}
                    type="text"
                    placeholder="Cari No. Pembelian..."
                    class="w-full pl-10 pr-4 py-2.5 bg-white border border-gray-200 rounded-xl focus:ring-2 focus:ring-brand focus:border-transparent outline-none transition-all shadow-sm"
                />
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0 1 14 0z" /></svg>
            </div>
        </div>

        <div class="p-4">
            <Table 
                headers={headerTable} 
                data={data} 
                page={currentPage}
                totalPages={totalPages}
                totalItems={totalItems}
                onPageChange={handlePageChange}
                on:detail={(e) => { /* TODO: Modal Detail Purchase */ }}
                on:update={(e) => { /* Purchase usually not updated once saved */ }}
                on:remove={(e) => { /* TODO: Modal Remove Purchase */ }}
            />
        </div>
    {/if}
</div>

{#if openModalAdd}
    <Purchase open={openModalAdd} on:close={handlerAdd} />
{/if}

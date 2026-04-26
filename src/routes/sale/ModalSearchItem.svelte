<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, tick } from "svelte";

    let { open = false, onselect, onclose } = $props();

    let search = $state("");
    let items = $state([]);
    let loading = $state(false);
    let selectedIndex = $state(0);
    let searchInput;

    async function fetchItems() {
        loading = true;
        try {
            const res = await invoke("items_get", {
                filter: {
                    search: search,
                    page: 1,
                    page_size: 50
                }
            });
            items = res.results;
            selectedIndex = 0;
        } catch (err) {
            console.error("Failed to fetch items:", err);
        } finally {
            loading = false;
        }
    }

    function handleSelect(item) {
        if (onselect) onselect(item);
        if (onclose) onclose();
    }

    function handleKeydown(e) {
        if (e.key === "ArrowDown") {
            e.preventDefault();
            selectedIndex = Math.min(selectedIndex + 1, items.length - 1);
        } else if (e.key === "ArrowUp") {
            e.preventDefault();
            selectedIndex = Math.max(selectedIndex - 1, 0);
        } else if (e.key === "Enter") {
            if (items[selectedIndex]) {
                handleSelect(items[selectedIndex]);
            }
        } else if (e.key === "Escape") {
            if (onclose) onclose();
        }
    }

    $effect(() => {
        if (open) {
            tick().then(() => searchInput?.focus());
            fetchItems();
        }
    });

    function formatCurrency(val) {
        return new Intl.NumberFormat("id-ID", {
            style: "currency",
            currency: "IDR",
            minimumFractionDigits: 0
        }).format(val || 0);
    }
</script>

{#if open}
    <div class="fixed inset-0 bg-black/50 z-[10001] flex items-center justify-center p-4" onclick={onclose}>
        <div class="bg-white w-full max-w-4xl rounded-2xl shadow-2xl overflow-hidden flex flex-col max-h-[80vh]" onclick={e => e.stopPropagation()}>
            <!-- Header -->
            <div class="p-6 border-b flex justify-between items-center bg-gray-50">
                <h2 class="text-2xl font-bold text-gray-800">Cari Barang</h2>
                <button onclick={onclose} class="p-2 hover:bg-gray-200 rounded-full transition-colors">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
            </div>

            <!-- Search Area -->
            <div class="p-6 border-b">
                <div class="relative">
                    <input
                        bind:this={searchInput}
                        bind:value={search}
                        oninput={() => { selectedIndex = 0; fetchItems(); }}
                        onkeydown={handleKeydown}
                        placeholder="Ketik nama atau barcode barang... (Enter untuk pilih)"
                        class="w-full pl-12 pr-4 py-4 bg-gray-100 border-2 border-transparent focus:border-brand focus:bg-white rounded-xl outline-none transition-all text-lg"
                    />
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
                </div>
            </div>

            <!-- List Area -->
            <div class="flex-1 overflow-y-auto p-2">
                {#if loading && items.length === 0}
                    <div class="flex justify-center py-20">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-brand"></div>
                    </div>
                {:else if items.length === 0}
                    <div class="text-center py-20 text-gray-500">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 opacity-20" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" /></svg>
                        <p class="text-xl">Barang tidak ditemukan</p>
                    </div>
                {:else}
                    <table class="w-full border-collapse">
                        <thead class="sticky top-0 bg-white shadow-sm">
                            <tr class="text-left text-gray-500 text-sm uppercase tracking-wider">
                                <th class="p-4 font-semibold">Barcode</th>
                                <th class="p-4 font-semibold">Nama Barang</th>
                                <th class="p-4 font-semibold">Satuan</th>
                                <th class="p-4 font-semibold text-right">Harga Jual</th>
                                <th class="p-4 font-semibold text-right">Stok</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each items as item, i}
                                <tr 
                                    onclick={() => handleSelect(item)}
                                    class="border-b last:border-0 cursor-pointer transition-colors {i === selectedIndex ? 'bg-brand/10 border-brand' : 'hover:bg-gray-50'}"
                                >
                                    <td class="p-4 font-mono text-sm">{item.barcode || '-'}</td>
                                    <td class="p-4 font-bold">{item.name}</td>
                                    <td class="p-4"><span class="px-2 py-1 bg-gray-100 rounded text-xs uppercase">{item.type_unit}</span></td>
                                    <td class="p-4 text-right font-bold text-brand">{formatCurrency(item.price_sell)}</td>
                                    <td class="p-4 text-right">
                                        <span class={item.qty > 0 ? 'text-green-600' : 'text-red-500'}>
                                            {item.qty}
                                        </span>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                {/if}
            </div>

            <!-- Footer -->
            <div class="p-4 bg-gray-50 border-t flex justify-between items-center text-sm text-gray-500">
                <div>Ditemukan {items.length} barang</div>
                <div class="flex gap-4">
                    <span><kbd class="px-2 py-1 bg-white border rounded shadow-sm">↑↓</kbd> Navigasi</span>
                    <span><kbd class="px-2 py-1 bg-white border rounded shadow-sm">Enter</kbd> Pilih</span>
                    <span><kbd class="px-2 py-1 bg-white border rounded shadow-sm">Esc</kbd> Tutup</span>
                </div>
            </div>
        </div>
    </div>
{/if}

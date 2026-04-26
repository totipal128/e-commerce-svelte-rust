<script>
    import { createEventDispatcher } from "svelte";
    import { tick } from "svelte";
    import ModalAdd from "./ModalAddPurchase.svelte";
    import ModalSearchItem from "../sale/ModalSearchItem.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { showToast } from "$lib/store/toast.js";

    const dispatch = createEventDispatcher();
    let { open = false } = $props();

    let listSuppliers = $state([]);
    let codePurchase = $state("");
    let loading = $state(true);
    let openModalAdd = $state(false);
    let openModalSearch = $state(false);
    let indexRowSearch = $state(null);
    let dataByBarcode = $state(null);
    let refs = $state([]);

    let result_data = $state({
        code: null,
        supplier_id: null,
        discount: 0,
        ppn: 0,
        total_item: 0,
        total: 0,
        payment: 0,
        items: [
            {
                code: null,
                items_id: null,
                items_name: null,
                items_unit: null,
                items_price: null,
                total: null,
                qty: null,
            },
        ],
    });

    async function fetchData() {
        try {
            const suppliers = await invoke("supplier_list");
            listSuppliers = suppliers.results;
            codePurchase = await invoke("purchase_get_code");
        } catch (err) {
            console.error(err);
        } finally {
            loading = false;
        }
    }

    onMount(fetchData);

    async function getByBarcodeDetail(barcode) {
        loading = true;
        try {
            const result = await invoke("get_items_by_barcode", { barcode });
            dataByBarcode = result.data;
        } catch (err) {
            console.error(err);
        } finally {
            loading = false;
        }
    }

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/[^0-9]/g, ""));
    }

    function calculateTotal(index) {
        const item = result_data.items[index];
        if (!item || !item.items_id) return;
        item.total = parseNumber(item.items_price) * parseNumber(item.qty);
        result_data.total = result_data.items.reduce((acc, i) => acc + (parseNumber(i.total) || 0), 0);
        result_data.total_item = result_data.items.reduce((acc, i) => acc + (parseNumber(i.qty) || 0), 0);
    }

    async function handleCodeInput(e, code, index_row, index_cell) {
        if (!code) return;
        const dupIndex = result_data.items.findIndex((item, i) => item.code === code && i !== index_row);
        if (dupIndex !== -1) {
            result_data.items[dupIndex].qty += 1;
            calculateTotal(dupIndex);
            result_data.items[index_row] = { code: null, items_id: null, items_name: null, items_unit: null, items_price: null, total: null, qty: null };
            refs[index_row]?.focus();
            return;
        }

        await getByBarcodeDetail(code);
        if (dataByBarcode == null) {
            indexRowSearch = index_row;
            openModalSearch = true;
            return;
        }

        result_data.items[index_row].items_id = dataByBarcode.id;
        result_data.items[index_row].items_name = dataByBarcode.name;
        result_data.items[index_row].items_unit = dataByBarcode.type_unit;
        result_data.items[index_row].items_price = dataByBarcode.price_buy; // Use price_buy for purchase
        result_data.items[index_row].qty = 1;
        calculateTotal(index_row);

        if (result_data.items.length - 1 === index_row) {
            result_data.items.push({ code: null, items_id: null, items_name: null, items_unit: null, items_price: null, total: null, qty: null });
        }
        dataByBarcode = null;
        tick().then(() => refs[index_row + 1]?.focus());
    }

    function handleItemSelected(item) {
        if (indexRowSearch !== null) {
            const dupIndex = result_data.items.findIndex(it => it.items_id === item.id);
            if (dupIndex !== -1) {
                result_data.items[dupIndex].qty += 1;
                calculateTotal(dupIndex);
                result_data.items[indexRowSearch].code = null;
                tick().then(() => refs[indexRowSearch]?.focus());
            } else {
                result_data.items[indexRowSearch].code = item.barcode;
                result_data.items[indexRowSearch].items_id = item.id;
                result_data.items[indexRowSearch].items_name = item.name;
                result_data.items[indexRowSearch].items_unit = item.type_unit;
                result_data.items[indexRowSearch].items_price = item.price_buy;
                result_data.items[indexRowSearch].qty = 1;
                calculateTotal(indexRowSearch);
                if (result_data.items.length - 1 === indexRowSearch) {
                    result_data.items.push({ code: null, items_id: null, items_name: null, items_unit: null, items_price: null, total: null, qty: null });
                }
                tick().then(() => refs[indexRowSearch + 1]?.focus());
            }
            openModalSearch = false;
        }
    }

    async function keydown(e, r, c, type) {
        if (e.key === "Enter") {
            if (type === "qty") {
                calculateTotal(r);
                tick().then(() => refs[r + 1]?.focus());
                return;
            }
            handleCodeInput(e, e.target.value, r, c);
        }
    }

    function savekeydown(e) {
        if (e.key === "F8") {
            if (result_data.total <= 0) {
                showToast("Masukkan barang terlebih dahulu.", "error");
                return;
            }
            result_data.code = codePurchase;
            openModalAdd = true;
        }
        if (e.key === "Escape") dispatch("close");
    }

    function ensureRef(r, c) {
        // Removed to avoid state_unsafe_mutation
    }

    function handlerCloseModalSave() {
        openModalAdd = false;
    }

    function handleSaveSuccess() {
        openModalAdd = false;
        dispatch("close");
    }

    function formatDate(date) {
        const d = new Date(date);
        return `${d.getDate().toString().padStart(2, '0')}-${(d.getMonth()+1).toString().padStart(2, '0')}-${d.getFullYear()}`;
    }

    let headers = [
        { name: "Code Barang", value: "code", type: "input-code" },
        { name: "Nama Barang", value: "items_name", type: "text" },
        { name: "Satuan", value: "items_unit", type: "text" },
        { name: "Harga Beli", value: "items_price", type: "input-price" },
        { name: "QTY", value: "qty", type: "input-number" },
        { name: "Total", value: "total", type: "parse-number" },
    ];
</script>

{#if open}
    <div class="fixed inset-0 bg-gray-100 z-[100] flex flex-col p-4 overflow-auto" onkeydown={savekeydown} tabindex="0">
        <!-- Header Section -->
        <div class="flex justify-between items-center bg-white p-6 rounded-2xl shadow-sm mb-4">
            <div class="flex gap-8">
                <div class="space-y-2">
                    <p class="text-sm text-gray-500">No. Pembelian</p>
                    <p class="text-lg font-bold">{codePurchase}</p>
                </div>
                <div class="space-y-2">
                    <p class="text-sm text-gray-500">Supplier</p>
                    <select bind:value={result_data.supplier_id} class="bg-gray-50 border rounded-lg px-4 py-2 focus:ring-brand focus:border-brand">
                        <option value={null}>-- Pilih Supplier --</option>
                        {#each listSuppliers as s}
                            <option value={s.id}>{s.name}</option>
                        {/each}
                    </select>
                </div>
                <div class="space-y-2">
                    <p class="text-sm text-gray-500">Tanggal</p>
                    <p class="text-lg font-bold">{formatDate(new Date())}</p>
                </div>
            </div>
            <div class="text-right">
                <p class="text-sm text-gray-500">Total Pembelian</p>
                <p class="text-5xl font-black text-brand">
                    {result_data.total.toLocaleString("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 })}
                </p>
            </div>
        </div>

        <!-- Table Section -->
        <div class="flex-1 bg-white rounded-2xl shadow-sm overflow-hidden flex flex-col">
            <div class="overflow-y-auto flex-1">
                <table class="w-full text-left">
                    <thead class="bg-gray-50 sticky top-0 z-10 border-b">
                        <tr>
                            <th class="p-4 font-bold text-gray-600">#</th>
                            {#each headers as header}
                                <th class="p-4 font-bold text-gray-600">{header.name}</th>
                            {/each}
                            <th class="p-4 font-bold text-gray-600">Aksi</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each result_data.items as item, i (i)}
                            <tr class="border-b hover:bg-gray-50 transition-colors">
                                <td class="p-4 text-gray-400">{i + 1}</td>
                                {#each headers as header, hi}
                                    <td class="p-2">
                                        {#if header.type === 'input-code'}
                                            <input
                                                bind:this={refs[i]}
                                                bind:value={item[header.value]}
                                                onkeydown={(e) => keydown(e, i, hi)}
                                                class="w-full bg-gray-50 border border-transparent focus:border-brand focus:bg-white rounded-lg p-2 font-mono text-sm"
                                                placeholder="Scan/Ketik..."
                                            />
                                        {:else if header.type === 'input-number'}
                                            <input
                                                type="number"
                                                bind:value={item[header.value]}
                                                oninput={() => calculateTotal(i)}
                                                onkeydown={(e) => keydown(e, i, hi, 'qty')}
                                                class="w-20 bg-gray-50 border border-transparent focus:border-brand focus:bg-white rounded-lg p-2 text-center font-bold"
                                            />
                                        {:else if header.type === 'input-price'}
                                             <input
                                                type="number"
                                                bind:value={item[header.value]}
                                                oninput={() => calculateTotal(i)}
                                                class="w-32 bg-gray-50 border border-transparent focus:border-brand focus:bg-white rounded-lg p-2 text-right font-bold text-blue-600"
                                            />
                                        {:else if header.type === 'parse-number'}
                                            <div class="text-right font-bold">
                                                {parseNumber(item[header.value]).toLocaleString('id-ID')}
                                            </div>
                                        {:else}
                                            <div class="p-2 font-medium">{item[header.value] || '-'}</div>
                                        {/if}
                                    </td>
                                {/each}
                                <td class="p-2">
                                    <button onclick={() => { result_data.items.splice(i, 1); result_data.items = result_data.items; calculateTotal(); }} class="p-2 text-red-500 hover:bg-red-50 rounded-full">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg>
                                    </button>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Footer Actions -->
        <div class="mt-4 flex justify-between items-center p-4 bg-white rounded-2xl shadow-sm">
            <div class="flex gap-4 text-sm text-gray-500">
                <span><kbd class="px-2 py-1 bg-gray-100 border rounded">F8</kbd> Simpan Transaksi</span>
                <span><kbd class="px-2 py-1 bg-gray-100 border rounded">Esc</kbd> Batalkan</span>
            </div>
            <div class="flex gap-3">
                <button onclick={() => dispatch('close')} class="px-6 py-3 border rounded-xl hover:bg-gray-50 font-bold transition-colors">Batal</button>
                <button onclick={() => (openModalAdd = true)} class="px-8 py-3 bg-brand text-white rounded-xl hover:bg-brand-strong font-bold shadow-lg shadow-brand/20 transition-all">
                    Simpan (F8)
                </button>
            </div>
        </div>
    </div>
{/if}

{#if openModalAdd}
    <ModalAdd
        open={openModalAdd}
        res={result_data}
        onclose={handlerCloseModalSave}
        onsave={handleSaveSuccess}
    />
{/if}

{#if openModalSearch}
    <ModalSearchItem 
        open={openModalSearch} 
        onselect={handleItemSelected} 
        onclose={() => openModalSearch = false} 
    />
{/if}
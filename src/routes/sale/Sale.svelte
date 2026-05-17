<script>
    import { createEventDispatcher } from "svelte";
    import { tick } from "svelte";
    import ModalAdd from "./ModalAddSale.svelte";
    import ModalSearchItem from "./ModalSearchItem.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { showToast } from "$lib/store/toast.js";

    let { open = false, onclose } = $props();

    let listDataConsumer = $state([]);
    let codeSale = $state("");
    let loading = $state(true);
    let openModalAdd = $state(false);
    let openModalSearch = $state(false);
    let indexRowSearch = $state(null);
    let dataByBarcode = $state(null);
    let refs = $state([]);

    let result_data = $state({
        code: null,
        customer_id: null,
        discount: 0,
        ppn: 0,
        total_item: 0,
        total: 0,
        change: 0,
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
                stock: 0,
                available_units: [],
            },
        ],
    });

    async function fetchData() {
        try {
            const consumers = await invoke("consumer_list");
            listDataConsumer = consumers.results;
            codeSale = await invoke("sale_get_code_txr");
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

        const qtyInput = Number(item.qty) || 0;
        const stockAvailable = Number(item.stock) || 0;

        if (qtyInput > stockAvailable && stockAvailable > 0) {
            showToast(
                `Perhatian: Stok ${item.items_name} tidak mencukupi! Tersedia: ${stockAvailable}`,
                "error",
            );
            item.qty = stockAvailable;
        } else if (stockAvailable <= 0) {
            // If stock is 0 or couldn't be loaded, we briefly allow input but maybe warn?
            // Or just trust the backend check on scan.
        }

        item.total = parseNumber(item.items_price) * parseNumber(item.qty);
        result_data.total = result_data.items.reduce(
            (acc, i) => acc + (parseNumber(i.total) || 0),
            0,
        );
        result_data.total_item = result_data.items.reduce(
            (acc, i) => acc + (parseNumber(i.qty) || 0),
            0,
        );
    }

    async function handleCodeInput(e, code, index_row, index_cell) {
        if (!code) return;
        const dupIndex = result_data.items.findIndex(
            (item, i) => item.code === code && i !== index_row,
        );
        if (dupIndex !== -1) {
            result_data.items[dupIndex].qty += 1;
            calculateTotal(dupIndex);
            result_data.items[index_row] = {
                code: null,
                items_id: null,
                items_name: null,
                items_unit: null,
                items_price: null,
                total: null,
                qty: null,
                stock: 0,
            };
            refs[index_row]?.focus();
            return;
        }

        await getByBarcodeDetail(code);
        if (dataByBarcode == null) {
            indexRowSearch = index_row;
            openModalSearch = true;
            return;
        }

        let available_units = [];
        try {
            const resPrices = await invoke("get_items_price__by_items_id", { item_id: dataByBarcode.id });
            available_units = resPrices.data || [];
        } catch (err) {
            console.error(err);
        }

        const hasMainUnit = available_units.some(u => u.type_unit === dataByBarcode.type_unit);
        if (!hasMainUnit && dataByBarcode.type_unit) {
            available_units.unshift({
                barcode: dataByBarcode.barcode,
                type_unit: dataByBarcode.type_unit,
                price_buy: dataByBarcode.price_buy || 0,
                price_sell: dataByBarcode.price_sell,
                qty: dataByBarcode.qty,
            });
        }

        result_data.items[index_row].items_id = dataByBarcode.id;
        result_data.items[index_row].items_name = dataByBarcode.name;
        result_data.items[index_row].items_unit = dataByBarcode.type_unit;
        result_data.items[index_row].items_price = dataByBarcode.price_sell;
        result_data.items[index_row].qty = 1;
        result_data.items[index_row].stock = dataByBarcode.qty;
        result_data.items[index_row].available_units = available_units;
        calculateTotal(index_row);

        if (result_data.items.length - 1 === index_row) {
            result_data.items.push({
                code: null,
                items_id: null,
                items_name: null,
                items_unit: null,
                items_price: null,
                total: null,
                qty: null,
                stock: 0,
                available_units: [],
            });
        }
        dataByBarcode = null;
        tick().then(() => refs[index_row + 1]?.focus());
    }

    async function handleItemSelected(item) {
        if (indexRowSearch !== null) {
            const dupIndex = result_data.items.findIndex(
                (it) => it.items_id === item.id,
            );
            if (dupIndex !== -1) {
                result_data.items[dupIndex].qty += 1;
                calculateTotal(dupIndex);
                result_data.items[indexRowSearch].code = null;
                tick().then(() => refs[indexRowSearch]?.focus());
            } else {
                let available_units = [];
                try {
                    const resPrices = await invoke("get_items_price__by_items_id", { item_id: item.id });
                    available_units = resPrices.data || [];
                } catch (err) {
                    console.error(err);
                }

                const hasMainUnit = available_units.some(u => u.type_unit === item.type_unit);
                if (!hasMainUnit && item.type_unit) {
                    available_units.unshift({
                        barcode: item.barcode,
                        type_unit: item.type_unit,
                        price_buy: item.price_buy || 0,
                        price_sell: item.price_sell,
                        qty: item.qty_stock || 0,
                    });
                }

                result_data.items[indexRowSearch].code = item.barcode;
                result_data.items[indexRowSearch].items_id = item.id;
                result_data.items[indexRowSearch].items_name = item.name;
                result_data.items[indexRowSearch].items_unit = item.type_unit;
                result_data.items[indexRowSearch].items_price = item.price_sell;
                result_data.items[indexRowSearch].qty = 1;
                result_data.items[indexRowSearch].stock = item.qty_stock || 0;
                result_data.items[indexRowSearch].available_units = available_units;
                calculateTotal(indexRowSearch);
                if (result_data.items.length - 1 === indexRowSearch) {
                    result_data.items.push({
                        code: null,
                        items_id: null,
                        items_name: null,
                        items_unit: null,
                        items_price: null,
                        total: null,
                        qty: null,
                        stock: 0,
                        available_units: [],
                    });
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

    function handlePreparePayment() {
        if (result_data.total <= 0) {
            showToast("Masukkan barang terlebih dahulu.", "error");
            tick().then(() => refs[0]?.focus());
            return;
        }
        result_data.code = codeSale;
        openModalAdd = true;
    }

    function savekeydown(e) {
        if (e.key === "F8") {
            handlePreparePayment();
        }
        if (e.key === "Escape") onclose();
    }

    function handlerCloseModalSave() {
        openModalAdd = false;
        onclose(); // Close sale form and refresh list
    }

    function handleSaveSuccess() {
        // Do NOT close modal here — let user see print button first.
        // Modal closes only when user clicks "Selesai" (which triggers onclose).
    }

    function formatDate(date) {
        const d = new Date(date);
        return `${d.getDate().toString().padStart(2, "0")}-${(d.getMonth() + 1).toString().padStart(2, "0")}-${d.getFullYear()}`;
    }

    function handleUnitChange(index, selectedUnit) {
        const item = result_data.items[index];
        if (!item || !item.available_units) return;

        const selectedPriceRecord = item.available_units.find(
            (u) => u.type_unit === selectedUnit
        );
        if (selectedPriceRecord) {
            item.items_unit = selectedUnit;
            item.items_price = selectedPriceRecord.price_sell;
            item.stock = selectedPriceRecord.qty;
            item.code = selectedPriceRecord.barcode || item.code;
            calculateTotal(index);
        }
    }

    let headers = [
        { name: "Code Barang", value: "code", type: "input-code" },
        { name: "Nama Barang", value: "items_name", type: "text" },
        { name: "Satuan", value: "items_unit", type: "select-unit" },
        { name: "Harga", value: "items_price", type: "parse-number" },
        { name: "QTY", value: "qty", type: "input-number" },
        { name: "Total", value: "total", type: "parse-number" },
    ];
</script>

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
        onclose={() => (openModalSearch = false)}
    />
{/if}

{#if open}
    <div
        class="fixed inset-0 bg-gray-100 z-[100] flex flex-col p-4 overflow-auto"
        onkeydown={savekeydown}
        tabindex="0"
    >
        <!-- Header Section -->
        <div
            class="flex justify-between items-center bg-white p-6 rounded-2xl shadow-sm mb-4"
        >
            <div class="flex gap-8">
                <div class="space-y-2">
                    <p class="text-sm text-gray-500">No. Transaksi</p>
                    <p class="text-lg font-bold">{codeSale}</p>
                </div>
                <div class="space-y-2">
                    <p class="text-sm text-gray-500">Pelanggan</p>
                    <select
                        bind:value={result_data.customer_id}
                        class="bg-gray-50 border rounded-lg px-4 py-2 focus:ring-brand focus:border-brand"
                    >
                        <option value={null}>-- Umum --</option>
                        {#each listDataConsumer as c}
                            <option value={c.id}>{c.name}</option>
                        {/each}
                    </select>
                </div>
                <div class="space-y-2">
                    <p class="text-sm text-gray-500">Tanggal</p>
                    <p class="text-lg font-bold">{formatDate(new Date())}</p>
                </div>
            </div>
            <div class="text-right">
                <p class="text-sm text-gray-500">Total Belanja</p>
                <p class="text-5xl font-black text-brand">
                    {result_data.total.toLocaleString("id-ID", {
                        style: "currency",
                        currency: "IDR",
                        minimumFractionDigits: 0,
                    })}
                </p>
            </div>
        </div>

        <!-- Table Section -->
        <div
            class="flex-1 bg-white rounded-2xl shadow-sm overflow-hidden flex flex-col"
        >
            <div class="overflow-y-auto flex-1">
                <table class="w-full text-left">
                    <thead class="bg-gray-50 sticky top-0 z-10 border-b">
                        <tr>
                            <th class="p-4 font-bold text-gray-600">#</th>
                            {#each headers as header}
                                <th class="p-4 font-bold text-gray-600"
                                    >{header.name}</th
                                >
                            {/each}
                            <th class="p-4 font-bold text-gray-600">Aksi</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each result_data.items as item, i (i)}
                            <tr
                                class="border-b hover:bg-gray-50 transition-colors"
                            >
                                <td class="p-4 text-gray-400">{i + 1}</td>
                                {#each headers as header, hi}
                                    <td class="p-2">
                                        {#if header.type === "input-code"}
                                            <input
                                                bind:this={refs[i]}
                                                bind:value={item[header.value]}
                                                onkeydown={(e) =>
                                                    keydown(e, i, hi)}
                                                class="w-full bg-gray-50 border border-transparent focus:border-brand focus:bg-white rounded-lg p-2 font-mono text-sm"
                                                placeholder="Scan/Ketik..."
                                            />
                                        {:else if header.type === "select-unit"}
                                            {#if item.available_units && item.available_units.length > 0}
                                                <select
                                                    value={item.items_unit}
                                                    onchange={(e) => handleUnitChange(i, e.target.value)}
                                                    class="w-full bg-blue-50 border border-blue-200 text-blue-800 focus:border-brand focus:bg-white rounded-lg p-2 font-bold outline-none cursor-pointer"
                                                >
                                                    {#each item.available_units as u}
                                                        <option value={u.type_unit}>{u.type_unit}</option>
                                                    {/each}
                                                </select>
                                            {:else}
                                                <select
                                                    bind:value={item.items_unit}
                                                    class="w-full bg-gray-50 border border-gray-200 text-gray-400 rounded-lg p-2 font-bold outline-none cursor-not-allowed"
                                                    disabled
                                                >
                                                    <option value={item.items_unit}>{item.items_unit || "-"}</option>
                                                </select>
                                            {/if}
                                        {:else if header.type === "input-number"}
                                            <input
                                                type="number"
                                                bind:value={item[header.value]}
                                                oninput={() =>
                                                    calculateTotal(i)}
                                                onkeydown={(e) =>
                                                    keydown(e, i, hi, "qty")}
                                                class="w-20 bg-gray-50 border border-transparent focus:border-brand focus:bg-white rounded-lg p-2 text-center font-bold"
                                            />
                                        {:else if header.type === "parse-number"}
                                            <div class="text-right font-bold">
                                                {parseNumber(
                                                    item[header.value],
                                                ).toLocaleString("id-ID")}
                                            </div>
                                        {:else}
                                            <div class="p-2 font-medium">
                                                {item[header.value] || "-"}
                                            </div>
                                        {/if}
                                    </td>
                                {/each}
                                <td class="p-2">
                                    <button
                                        onclick={() => {
                                            const removedItem =
                                                result_data.items[i];
                                            result_data.items.splice(i, 1);
                                            result_data.items =
                                                result_data.items;
                                            calculateTotal();
                                            if (removedItem.items_name) {
                                                showToast(
                                                    `Menghapus ${removedItem.items_name} dari daftar`,
                                                    "info",
                                                );
                                            }
                                        }}
                                        class="p-2 text-red-500 hover:bg-red-50 rounded-full"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="h-6 w-6"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                            ><path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                            /></svg
                                        >
                                    </button>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Footer Actions -->
        <div
            class="mt-4 flex justify-between items-center p-4 bg-white rounded-2xl shadow-sm"
        >
            <div class="flex gap-4 text-sm text-gray-500">
                <span
                    ><kbd class="px-2 py-1 bg-gray-100 border rounded">F8</kbd> Simpan
                    Transaksi</span
                >
                <span
                    ><kbd class="px-2 py-1 bg-gray-100 border rounded">Esc</kbd>
                    Batalkan</span
                >
            </div>
            <div class="flex gap-3">
                <button
                    onclick={() => onclose()}
                    class="px-6 py-3 border rounded-xl hover:bg-gray-50 font-bold transition-colors"
                    >Batal</button
                >
                <button
                    onclick={handlePreparePayment}
                    disabled={result_data.total <= 0}
                    class="px-8 py-3 bg-brand text-white rounded-xl hover:bg-brand-strong font-bold shadow-lg shadow-brand/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-gray-400"
                >
                    Bayar & Simpan (F8)
                </button>
            </div>
        </div>
    </div>
{/if}

<script>
    import { invoke } from "@tauri-apps/api/core";
    import { tick } from "svelte";
    import Receipt from "$lib/component/Receipt.svelte";
    import { showToast } from "$lib/store/toast.js";

    let { res, open = false, onclose, onsave } = $props();

    let focus_data;
    let loading = $state(false);
    let successfullySaved = $state(false);

    // ===== PAYMENT LOGIC =====
    function addPayment(amount) {
        res.payment = (Number(res.payment) || 0) + amount;
        calcChange();
    }

    function calcChange() {
        res.change = (Number(res.payment) || 0) - res.total;
    }

    function setExact() {
        res.payment = res.total;
        res.change = 0;
    }

    // ===== SAVE & PRINT =====
    async function handlerSave() {
        loading = true;
        try {
            const cleanData = {
                ...res,
                items: res.items.filter(i => i.items_id != null)
            };
            await invoke("sale_create", { data: cleanData });
            successfullySaved = true;
            showToast("Transaksi berhasil disimpan!", "success");
            if (onsave) onsave();
        } catch (err) {
            console.error(err);
            showToast("Gagal menyimpan: " + err, "error");
        } finally {
            loading = false;
        }
    }

    function handlePrint() {
        window.print();
    }

    function handlerClose() {
        if (!successfullySaved) {
            res.payment = 0;
            res.change = 0;
        }
        successfullySaved = false;
        if (onclose) onclose();
    }

    function handleKeydown(e) {
        if (e.key === "Enter" && !successfullySaved && !loading) handlerSave();
        if (e.key === "Escape") handlerClose();
        if (e.key === "F12" && successfullySaved) handlePrint();
    }

    $effect(async () => {
        if (open) {
            await tick();
            calcChange();
            focus_data?.focus();
        }
    });

    let canPay = $derived(!loading && res.change >= 0 && res.payment > 0);
</script>

<!-- Area Khusus Print (Selalu Ada tapi tersembunyi kecuali saat print) -->
<div id="receipt-print-area" class="hidden print:block print:absolute print:top-0 print:left-0 print:w-full">
    <Receipt data={res} />
</div>

{#if open}
    <div 
        class="fixed inset-0 bg-black/70 z-[1000] flex items-center justify-center p-4 backdrop-blur-sm" 
        onkeydown={handleKeydown} 
        tabindex="-1"
    >
        <!-- Prevent modal closing when clicking inside -->
        <div 
            class="bg-white w-full max-w-md rounded-3xl shadow-2xl overflow-hidden flex flex-col" 
            onclick={e => e.stopPropagation()}
        >
            {#if !successfullySaved}
                <!-- ========== FASE 1: INPUT PEMBAYARAN ========== -->
                
                <!-- Header -->
                <div class="p-6 bg-brand text-white text-center relative">
                    <button onclick={handlerClose} class="absolute top-4 right-4 p-1.5 bg-white/20 hover:bg-white/30 rounded-full transition-all">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/></svg>
                    </button>
                    <p class="text-white/70 text-xs uppercase tracking-widest font-bold mb-1">Total Tagihan</p>
                    <h2 class="text-4xl font-black">
                        {res.total.toLocaleString("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 })}
                    </h2>
                </div>

                <!-- Body -->
                <div class="p-6 space-y-5">
                    <div>
                        <label for="payment-input" class="block text-xs font-bold text-gray-500 uppercase tracking-wider mb-2">Jumlah Bayar</label>
                        <div class="relative">
                            <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 font-bold text-lg">Rp</span>
                            <input
                                bind:this={focus_data}
                                bind:value={res.payment}
                                oninput={calcChange}
                                type="number"
                                id="payment-input"
                                class="w-full pl-12 pr-20 py-4 bg-gray-50 border-2 border-gray-100 focus:border-brand focus:bg-white rounded-2xl text-2xl font-black text-brand outline-none transition-all"
                                placeholder="0"
                            />
                            <button
                                onclick={setExact}
                                class="absolute right-2 top-1/2 -translate-y-1/2 bg-brand/10 text-brand px-3 py-1.5 rounded-xl font-bold text-sm"
                            >
                                PAS
                            </button>
                        </div>
                    </div>

                    <div class="grid grid-cols-4 gap-2">
                        {#each [10000, 20000, 50000, 100000] as amt}
                            <button
                                onclick={() => addPayment(amt)}
                                class="py-2 bg-gray-50 border border-gray-200 hover:bg-brand/10 hover:text-brand rounded-xl font-bold text-xs transition-all"
                            >
                                +{(amt/1000)}rb
                            </button>
                        {/each}
                    </div>

                    <div class="flex justify-between items-center p-4 rounded-2xl border-2 {res.change < 0 ? 'border-red-100 bg-red-50' : 'border-green-100 bg-green-50'}">
                        <span class="text-sm font-bold {res.change < 0 ? 'text-red-500' : 'text-gray-500'}">Kembalian</span>
                        <span class="text-2xl font-black {res.change < 0 ? 'text-red-500' : 'text-green-600'}">
                            {(res.change || 0).toLocaleString("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 })}
                        </span>
                    </div>
                </div>

                <!-- Footer -->
                <div class="p-4 bg-gray-50 border-t flex gap-3">
                    <button onclick={handlerClose} class="px-5 py-3.5 border rounded-2xl font-bold text-gray-500">Batal</button>
                    <button
                        onclick={handlerSave}
                        disabled={!canPay}
                        class="flex-1 py-3.5 bg-brand text-white rounded-2xl font-black shadow-lg disabled:opacity-40 transition-all"
                    >
                        {#if loading}
                            Menyimpan...
                        {:else}
                            Bayar & Simpan (Enter)
                        {/if}
                    </button>
                </div>

            {:else}
                <!-- ========== FASE 2: SUKSES & CETAK STRUK ========== -->
                
                <div class="p-6 text-center bg-green-500 text-white">
                    <div class="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-3">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-9 w-9" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3"><path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/></svg>
                    </div>
                    <h2 class="text-xl font-black">Pembayaran Berhasil!</h2>
                    <p class="text-sm opacity-80">Kode Transaksi: {res.code}</p>
                </div>

                <div class="p-6 space-y-4">
                    <div class="p-4 bg-blue-50 border-2 border-blue-100 rounded-2xl flex justify-between items-center">
                        <span class="text-sm font-bold text-blue-600">Kembalian</span>
                        <span class="text-2xl font-black text-blue-700">{(res.change || 0).toLocaleString("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 })}</span>
                    </div>
                    
                    <div class="max-h-48 overflow-y-auto bg-gray-50 border border-dashed rounded-2xl p-4">
                        <Receipt data={res} />
                    </div>
                </div>

                <div class="p-4 bg-gray-50 border-t flex gap-3">
                    <button onclick={handlerClose} class="px-6 py-3.5 border rounded-2xl font-bold text-gray-500">Selesai</button>
                    <button
                        onclick={handlePrint}
                        class="flex-1 py-3.5 bg-gray-800 text-white rounded-2xl font-black flex items-center justify-center gap-2"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9V2h12v7M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2M6 14h12v8H6z"/></svg>
                        Cetak Struk (F12)
                    </button>
                </div>
            {/if}
        </div>
    </div>
{/if}

<style>
    @media print {
        /* Sembunyikan SEMUA elemen ROOT agar bersih */
        :global(body > *:not(#receipt-print-area)) {
            display: none !important;
        }
        
        #receipt-print-area {
            display: block !important;
            position: absolute !important;
            top: 0 !important;
            left: 0 !important;
            width: 72mm !important;
            padding: 0 !important;
            margin: 0 !important;
        }

        @page {
            size: 80mm auto;
            margin: 0;
        }
    }
</style>

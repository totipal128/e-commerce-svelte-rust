<script>
    import { invoke } from "@tauri-apps/api/core";
    import { tick } from "svelte";
    import { showToast } from "$lib/store/toast.js";

    let { open = false, res, onclose, onsave } = $props();

    let paymentInput;
    let loading = $state(false);

    $effect(() => {
        if (open) {
            tick().then(() => paymentInput?.focus());
        }
    });

    async function handleSave() {
        if (res.total <= 0) {
            showToast("Total pembelian tidak boleh kosong", "error");
            return;
        }

        loading = true;
        try {
            // Clean up empty items
            const filteredItems = res.items.filter(i => i.items_id != null);
            const payload = {
                ...res,
                items: filteredItems
            };

            await invoke("purchase_create", { data: payload });
            showToast("Berhasil menyimpan pembelian!", "success");
            if (onsave) onsave();
        } catch (err) {
            console.error(err);
            showToast("Gagal menyimpan pembelian: " + err, "error");
        } finally {
            loading = false;
        }
    }

    function handleKeydown(e) {
        if (e.key === "Enter") handleSave();
        if (e.key === "Escape") onclose();
    }
</script>

{#if open}
    <div class="fixed inset-0 bg-black/60 z-[1000] flex items-center justify-center p-4 backdrop-blur-sm" onkeydown={handleKeydown}>
        <div class="bg-white w-full max-w-lg rounded-3xl shadow-2xl overflow-hidden flex flex-col" onclick={e => e.stopPropagation()}>
            <!-- Header -->
            <div class="p-8 bg-brand text-white text-center">
                <p class="text-brand-soft text-sm uppercase tracking-widest font-bold mb-2">Konfirmasi Pembelian</p>
                <h2 class="text-4xl font-black">
                    {res.total.toLocaleString("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 })}
                </h2>
            </div>

            <!-- Body -->
            <div class="p-8 space-y-6">
                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-1">
                        <p class="text-xs text-gray-500 font-bold uppercase">Kode Transaksi</p>
                        <p class="font-mono text-gray-800">{res.code}</p>
                    </div>
                    <div class="space-y-1 text-right">
                        <p class="text-xs text-gray-500 font-bold uppercase">Total Barang</p>
                        <p class="text-gray-800 font-bold">{res.total_item} Item</p>
                    </div>
                </div>

                <hr class="border-dashed" />

                <div class="space-y-4">
                    <div>
                        <label for="payment" class="block text-sm font-bold text-gray-600 mb-2">Jumlah Bayar / Hutang?</label>
                        <div class="relative">
                            <span class="absolute left-4 top-1/2 -translate-y-1/2 font-bold text-gray-400">Rp</span>
                            <input
                                bind:this={paymentInput}
                                bind:value={res.payment}
                                type="number"
                                id="payment"
                                class="w-full pl-12 pr-4 py-4 bg-gray-50 border-2 border-transparent focus:border-brand focus:bg-white rounded-2xl text-2xl font-black text-brand outline-none transition-all"
                                placeholder="0"
                            />
                        </div>
                        <p class="text-xs text-gray-400 mt-2 italic">*Kosongkan jika sistem hutang ke supplier</p>
                    </div>
                </div>
            </div>

            <!-- Footer -->
            <div class="p-6 bg-gray-50 border-t flex gap-3">
                <button
                    onclick={onclose}
                    class="flex-1 py-4 border-2 border-gray-200 rounded-2xl font-bold text-gray-500 hover:bg-gray-100 transition-colors"
                >
                    Kembali
                </button>
                <button
                    onclick={handleSave}
                    disabled={loading}
                    class="flex-[2] py-4 bg-brand text-white rounded-2xl font-bold shadow-lg shadow-brand/30 hover:bg-brand-strong transition-all flex items-center justify-center gap-2"
                >
                    {#if loading}
                        <div class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                        Menyimpan...
                    {:else}
                        Simpan Pembelian (Enter)
                    {/if}
                </button>
            </div>
        </div>
    </div>
{/if}

<script>
    import Receipt from "$lib/component/Receipt.svelte";
    import { createEventDispatcher } from "svelte";

    let { data = {}, open = false, onclose } = $props();

    function handlePrint() {
        window.print();
    }
</script>

{#if open}
    <div class="fixed inset-0 bg-black/80 z-[10000] flex items-center justify-center p-4 backdrop-blur-md">
        <div class="bg-gray-100 p-8 rounded-[2rem] shadow-2xl max-w-lg w-full flex flex-col items-center">
            <!-- Header Icon -->
            <div class="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-4 shadow-inner">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                </svg>
            </div>
            
            <h2 class="text-2xl font-black text-gray-800 mb-1">Transaksi Berhasil!</h2>
            <p class="text-gray-500 mb-8">Struk siap dicetak</p>

            <!-- Receipt Container (Limited height in preview) -->
            <div class="bg-white shadow-2xl p-4 rounded-2xl w-full max-h-[50vh] overflow-y-auto mb-8 border border-gray-200">
                <Receipt {data} />
            </div>

            <!-- Actions -->
            <div class="flex gap-4 w-full">
                <button 
                    onclick={() => onclose()} 
                    class="flex-1 px-6 py-4 bg-white border-2 border-gray-200 text-gray-600 rounded-2xl font-bold hover:bg-gray-50 transition-all"
                >
                    Tutup
                </button>
                <button 
                    onclick={handlePrint} 
                    class="flex-[2] px-6 py-4 bg-brand text-white rounded-2xl font-bold shadow-lg shadow-brand/30 hover:bg-brand-strong transition-all flex items-center justify-center gap-2"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M6 9V2h12v7M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2M6 14h12v8H6z"/>
                    </svg>
                    Cetak Struk (Enter)
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    /* CSS for Print is handled within Receipt.svelte */
</style>

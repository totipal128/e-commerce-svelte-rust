<script>
    import { formatCurrencyIDR } from '$lib/helpers/parse_number_format_currency';
    import { FormatDate } from '$lib/helpers/formated_date';

    let { data = {} } = $props();

    function formatTime(date) {
        if (!date) return "--:--";
        const d = new Date(date);
        return d.toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' });
    }
</script>

<div class="receipt-container p-2 bg-white text-black font-mono text-[11px] leading-tight w-[72mm] mx-auto print:w-[72mm] print:m-0 print:p-0">
    <!-- Header -->
    <div class="text-center mb-4">
        <h1 class="text-lg font-bold uppercase">TOKO E-COMMERCE</h1>
        <p class="text-xs">Jl. Contoh No. 123, Kota Anda</p>
        <p class="text-xs">Telp: 0812-3456-7890</p>
    </div>

    <!-- Info -->
    <div class="flex justify-between text-[10px] mb-2 border-b border-dashed border-black pb-2">
        <div>
            <p>{data.code || 'TRX-DEFAULT'}</p>
            <p>{data.consumer || 'Pelanggan Umum'}</p>
        </div>
        <div class="text-right">
            <p>{FormatDate(data.created_at || new Date())}</p>
            <p>{formatTime(data.created_at || new Date())}</p>
        </div>
    </div>

    <!-- Items -->
    <div class="mb-4">
        <table class="w-full text-[10px]">
            <thead>
                <tr class="border-b border-dashed border-black">
                    <th class="text-left py-1">Item</th>
                    <th class="text-center py-1">Qty</th>
                    <th class="text-right py-1">Total</th>
                </tr>
            </thead>
            <tbody>
                {#if data.items}
                    {#each data.items as item}
                        {#if item.items_id}
                            <tr>
                                <td class="py-1">
                                    <div class="font-bold">{item.items_name || 'Tanpa Nama'}</div>
                                    <span class="text-[9px]">@{formatCurrencyIDR(item.items_price)} x {item.qty}</span>
                                </td>
                                <td class="text-center py-1">{item.qty || 0}</td>
                                <td class="text-right py-1">{formatCurrencyIDR(item.total)}</td>
                            </tr>
                        {/if}
                    {/each}
                {/if}
            </tbody>
        </table>
    </div>

    <!-- Summary -->
    <div class="border-t border-dashed border-black pt-2 text-[10px] space-y-1">
        <div class="flex justify-between">
            <span>Total Item:</span>
            <span>{data.total_item || 0}</span>
        </div>
        <div class="flex justify-between font-bold text-sm pt-1 border-t border-gray-200">
            <span>TOTAL:</span>
            <span>{formatCurrencyIDR(data.total || 0)}</span>
        </div>
        <div class="flex justify-between">
            <span>Bayar:</span>
            <span>{formatCurrencyIDR(data.payment || 0)}</span>
        </div>
        <div class="flex justify-between">
            <span>Kembali:</span>
            <span>{formatCurrencyIDR(data.change || 0)}</span>
        </div>
    </div>

    <!-- Footer -->
    <div class="text-center mt-6 text-[10px] italic">
        <p>Terima Kasih Atas Kunjungan Anda</p>
        <p>Barang yang sudah dibeli tidak dapat ditukar/dikembalikan</p>
    </div>
</div>

<style>
    @media print {
        @page {
            size: 80mm auto;
            margin: 0;
        }
        :global(body) {
            background: white !important;
            margin: 0 !important;
            padding: 0 !important;
        }
        :global(body *) {
            visibility: hidden !important;
        }
        .receipt-container, .receipt-container * {
            visibility: visible !important;
        }
        .receipt-container {
            position: absolute !important;
            top: 0 !important;
            left: 0 !important;
            width: 72mm !important;
            padding: 4mm !important;
            margin: 0 !important;
            display: block !important;
            background: white !important;
            color: black !important;
        }
    }
</style>

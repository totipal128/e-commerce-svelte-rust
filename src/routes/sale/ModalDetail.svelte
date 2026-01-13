<script>
    import {createEventDispatcher, onMount, tick} from 'svelte';
    import {invoke} from "@tauri-apps/api/core";

    const dispatch = createEventDispatcher();
    let relationItems = [];
    let loading = $state(false);
    let get_detail_data = $state(
        {
            code: null,
            customer_id: null,
            PPN: 0,
            discount: 0,
            total_item: 0,
            total: 0,
            change: 0,
            payment: null,
            items: [
                {
                    code: null,
                    items_id: null,
                    items_name: null,
                    items_unit: 0,
                    items_price: 0,
                    total: 0,
                    qty: 0
                }
            ],
        },
    );
    let {
        id = 0
    } = $props()

    async function detail_data() {
        loading = true
        try {
            const result = await invoke("sale_by_id", {
                id: id
            })

            get_detail_data = result.data;
            console.log(get_detail_data);

        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
            console.log("Berhasil mengambil data")
        }

    }

    onMount(detail_data)

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/\./g, ""));
    };

    function handlerClose() {
        dispatch("close", false);
    }
</script>

<div class="absolute bg-black w-screen h-screen z-11 top-0" style="opacity: 0.95">
</div>
<div class="absolute inset-0 flex items-center justify-center z-[11] " on:click={handlerClose}>
	<div class=" bg-white min-w-200  rounded-2xl" on:click|stopPropagation>
		<div class="relative flex items-center justify-center py-6 ">
			<span class="text-3xl font-semibold">Detail Data</span>

			<button
					on:click={handlerClose}
					class="absolute right-4 top-4 hover:bg-gray-200 p-3 rounded-full"
			>
				X
			</button>
		</div>

		<!-- CONTENT -->
		<div class="p-3 text-2xl">
			<table class="p-3">
				<tbody>
				<tr class="p-2">
					<td class="p-3">Kode Penjualan</td>
					<td>:</td>
					<td class="p-3">{get_detail_data.code}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Kostomer</td>
					<td>:</td>
					<td class="p-3">Example</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Total</td>
					<td>:</td>
					<td class="p-3">{
                        parseNumber(get_detail_data.total).toLocaleString('id-ID', {
                            style: 'currency',
                            currency: 'IDR'
                        })}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Pembayaran</td>
					<td>:</td>
					<td class="p-3">{
                        parseNumber(get_detail_data.payment).toLocaleString('id-ID', {
                            style: 'currency',
                            currency: 'IDR'
                        })}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Kembalian</td>
					<td>:</td>
					<td class="p-3">{
                        parseNumber(get_detail_data.change).toLocaleString('id-ID', {
                            style: 'currency',
                            currency: 'IDR'
                        })}</td>
				</tr>
				</tbody>
			</table>
			<hr style="border: 1px solid #ccc;">


			<div class="pl-3 pt-7 min-h-50">
				<!--				<p class="text-lg mb-3">List Barang </p>-->
				<div class="flex justify-center">
					<table class="relative text-sm text-left rtl:text-right text-body w-full">
						<thead class="sticky top-0 text-sm text-body bg-neutral-secondary-soft border-b rounded-base border-default">
						<tr>
							<th scope="col" class="px-6 py-3 font-medium">
								Nama Barang
							</th>
							<th scope="col" class="px-6 py-3 font-medium">
								Satuan Barang
							</th>
							<th scope="col" class="px-6 py-3 font-medium">
								Harga
							</th>
							<th scope="col" class="px-6 py-3 font-medium">
								Jumlah
							</th>
							<th scope="col" class="px-6 py-3 font-medium">
								Total
							</th>
						</tr>
						</thead>
						<tbody>
						{#each get_detail_data.items as i, index}
							<tr class="bg-neutral-primary border-b border-default">
								<td class="px-6 py-4">
									{i.items_name}
								</td>
								<td class="px-6 py-4">
									{i.items_unit}
								</td>
								<td class="px-6 py-4">
									{parseNumber(i.items_price).toLocaleString('id-ID', {
                                        style: 'currency',
                                        currency: 'IDR'
                                    })}
								</td>
								<td class="px-6 py-4">
									{i.qty}
								</td>
								<td class="px-6 py-4">
									{parseNumber(i.total).toLocaleString('id-ID', {
                                        style: 'currency',
                                        currency: 'IDR'
                                    })}
								</td>
							</tr>
						{/each}

						</tbody>
					</table>
				</div>

			</div>
		</div>

		<!-- FOOTER -->
		<div class=" p-4 flex justify-center gap-3">
			<button
					on:click={handlerClose}
					class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300"
			>
				Close
			</button>
		</div>
	</div>
</div>
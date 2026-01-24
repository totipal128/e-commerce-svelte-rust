<script>
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    let {
        open = false,
        id = 0
    } = $props();

    let detailData = $state(null)

    let loading = $state(true)
    let priceItem = {
        barcode: "",
        unit: "",
        parent_type_unit: "",
        price_buy: 0,
        price_sell: 0,
        content: 0,
    }
    let itemsData = $state({
        barcode: "",
        name: "",
        type_unit: 0,
        items_category_id: null,
        qty_stock: 0,
        price: [
            priceItem
        ],

    })


    async function dataDetail() {
        loading = true
        try {
            const result = await invoke("get_items_by_id", {
                id: id
            })

            itemsData = result.data

        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }

    }

    //
    onMount(dataDetail)

    function closeModal() {
        dispatch('close');
    }

</script>

{#if open}
	<!-- Overlay -->
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black" style="opacity: 0.9" on:click={closeModal}>
		<!-- ModalCreate -->
		<div class="relative  bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6">
			<!-- Close button -->
			<button on:click={closeModal} class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<!-- Content -->
			<div class="relative h-full">
				<!--				<svg class="mx-auto mb-4 text-fg-disabled w-12 h-12" viewBox="0 0 24 24" fill="none">-->
				<!--					<path stroke="currentColor" stroke-width="2"-->
				<!--						  d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0Z"/>-->

				<!--				</svg>-->

				<div class="mx-auto mb-4 text-fg text-2xl text-center w-50 h-12">
					Detail Data

					<div class="h-[2px] bg-gray-400 my-2"></div>

				</div>


			</div>

			<table class="p-3">
				<tbody>
				<tr class="p-2">
					<td class="p-3">Barcode</td>
					<td>:</td>
					<td class="p-3">{itemsData.barcode}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Nama Barang</td>
					<td>:</td>
					<td class="p-3">{itemsData.name}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Jumlah</td>
					<td>:</td>
					<td class="p-3">{itemsData.qty_stock}</td>
				</tr>
				</tbody>
			</table>

			<div class="pl-3 pt-7">
				Advanced Detail
				<table class="relative text-sm text-left rtl:text-right text-body">
					<thead class="sticky top-0 text-sm text-body bg-neutral-secondary-soft border-b rounded-base border-default">
					<tr>
						<th scope="col" class="px-6 py-3 font-medium">
							Barcode
						</th>
						<th scope="col" class="px-6 py-3 font-medium">
							Satuan Barang
						</th>
						<th scope="col" class="px-6 py-3 font-medium">
							Induk Satuan Barang
						</th>
						<th scope="col" class="px-6 py-3 font-medium">
							Harga Beli
						</th>
						<th scope="col" class="px-6 py-3 font-medium">
							Harga Jual
						</th>
						<th scope="col" class="px-6 py-3 font-medium">
							Isi
						</th>
					</tr>
					</thead>
					<tbody>
					{#each itemsData.price as price, index}
						<tr class="bg-neutral-primary border-b border-default">
							<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
								{price.barcode}
							</th>
							<td class="px-6 py-4">
								{price.type_unit}
							</td>
							<td>
								{price.parent_type_unit}
							</td>

							<td class="px-6 py-4">
								{price.price_buy}
							</td>
							<td class="px-6 py-4">
								{price.price_buy}
							</td>
							<td class="px-6 py-4">
								{price.content}
							</td>
						</tr>
					{/each}

					</tbody>
				</table>
			</div>
		</div>
	</div>
{/if}

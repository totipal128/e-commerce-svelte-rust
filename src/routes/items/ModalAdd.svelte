<script>
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    let {
        open = false,
        id = 0
    } = $props();

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
    let data = $state([])

    async function saveData(data) {
        loading = true
        try {
            const result = await invoke("items_create", {
                data: data
            })

            data = result.results

        } catch (err) {
            console.log("err", err)
        } finally {
            closeModal()
            loading = false;
        }

    }

    //
    // onMount(fetchData)

    function closeModal(confirm) {
        let c = false
        if (typeof (confirm) !== "boolean") {
            c = true
        }

        dispatch('close', {confirm: c});
    }

    function removeRowPrice(i) {
        itemsData.price.splice(i)
    }

    function addRowPrice() {
        itemsData.price.push(priceItem)
    }

    function save(e) {
        itemsData.price[0].barcode = itemsData.barcode
        itemsData.price[0].unit = itemsData.type_unit
        saveData(itemsData)
    }
</script>

{#if open}
	<!-- Overlay -->
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black" style="opacity: 0.9" on:click={closeModal}>
		<!-- ModalCreate -->
		<div class="relative  bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6 w-300" on:click|stopPropagation>
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
					Tambah Data

					<div class="h-[2px] bg-gray-400 my-2"></div>

				</div>


				<form class="relative overflow-y-auto" on:submit|preventDefault={save}>
					<div class="max-w-sm mx-auto space-y-4">
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Barcode</label>
							<input bind:value={itemsData.barcode} type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""
								   required/>
						</div>
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Nama Barang</label>
							<input bind:value={itemsData.name} type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""
								   required/>
						</div>

						<label for="countries" class="block mb-2.5 text-sm font-medium text-heading">Kategory</label>
						<select id="countries" bind:value={itemsData.items_category_id} class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body">
							<option selected value={null}>--//--</option>
							<option value={1}>United States</option>
							<option value={2}>Canada</option>
							<option value={3}>France</option>
							<option value={4}>Germany</option>
						</select>

						<label for="countries" class="block mb-2.5 text-sm font-medium text-heading">Satuan Barang</label>
						<select bind:value={itemsData.type_unit} id="countries" class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body">
							<option selected value={null}>Choose a country</option>
							<option value="PCS">PCS</option>
							<option value="BOX">BOX</option>
							<option value="GANTUNG">GANTUNG</option>
						</select>

						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Stok</label>
							<input bind:value={itemsData.qty_stock} type="number" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
								   placeholder="" required/>
						</div>
					</div>

					<div class="relative overflow-x-auto overflow-y-auto max-h-[40vh] bg-neutral-primary-soft shadow-xs rounded-base border border-default mt-10">
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
								<th scope="col" class="px-6 py-3 font-medium">
									#
								</th>
							</tr>
							</thead>
							<tbody>
							{#each itemsData.price as price, index}
								<tr class="bg-neutral-primary border-b border-default">
									<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
										{#if index == 0}
											<input bind:value={itemsData.barcode} type="text" id="visitors"
												   class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
												   placeholder="" required/>
										{:else }
											<input bind:value={itemsData.price[index].barcode} type="text" id="visitors"
												   class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
												   placeholder="" required/>
										{/if}
									</th>
									<td class="px-6 py-4">
										{#if index == 0}
											<select bind:value={itemsData.type_unit} id="countries" class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body">
												<option selected>Choose a country</option>
												<option value="PCS">PCS</option>
												<option value="BOX">BOX</option>
												<option value="GANTUNG">GANTUNG</option>
											</select>
										{:else}
											<select bind:value={itemsData.price[index].unit} id="countries" class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body">
												<option selected>Choose a country</option>
												<option value="PCS">PCS</option>
												<option value="BOX">BOX</option>
												<option value="GANTUNG">GANTUNG</option>
											</select>
										{/if}

									</td>
									<td>
										<select bind:value={itemsData.price[index].parent_type_unit} id="countries"
												class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body">
											<option selected>Choose a country</option>
											<option value="PCS">PCS</option>
											<option value="BOX">BOX</option>
											<option value="GANTUNG">GANTUNG</option>
										</select>
									</td>

									<td class="px-6 py-4">
										<input bind:value={itemsData.price[index].price_buy} type="number" id="visitors"
											   class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="" required/>
									</td>
									<td class="px-6 py-4">
										<input bind:value={itemsData.price[index].price_sell} type="number" id="visitors"
											   class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="" required/>
									</td>
									<td class="px-6 py-4">
										<input bind:value={itemsData.price[index].content} type="number" id="visitors"
											   class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="" required/>
									</td>
									<td>
										{#if index != 0}
											<button on:click="{(e) => removeRowPrice(index)}" type="button"
													class="text-white bg-danger box-border border border-transparent hover:bg-danger-strong focus:ring-4 focus:ring-danger-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
												<img src="/icon/trash.svg" class="h-6 w-6 p-1 justify-center" alt="Tauri Logo"/>
											</button>
										{/if}

									</td>
								</tr>
							{/each}

							</tbody>
						</table>
						<button
								on:click={addRowPrice}
								class="text-white bg-gray-400 box-border border border-transparent hover:bg-brand-strong focus:ring-4 mt-2 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
							Tambah
						</button>
					</div>


					<div class="flex gap-4 justify-center mt-5">

						<button
								class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
							Simpan
						</button>

						<button
								on:click={closeModal}
								class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base">
							Batal
						</button>
					</div>


				</form>


			</div>
		</div>
	</div>
{/if}

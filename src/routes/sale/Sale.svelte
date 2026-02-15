<script>
    import {createEventDispatcher} from 'svelte';
    import items from '$lib/data/items.json';
    import {tick} from "svelte";
    import ModalAdd from "./ModalAddSale.svelte"
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";


    const dispatch = createEventDispatcher();
    let {
        open = false,
    } = $props()

	let listDataConsumer = $state([])
	async function fetchListConsumer() {
        try {
			let result = await invoke("consumer_list")
			listDataConsumer = result.results
            codeSale = await invoke("sale_get_code_txr")
        }
        catch(err) {
            console.error(err)
		}
        finally {
			loading = false
        }
    }

    let width = window.innerWidth;
    let height = window.innerHeight;
    let height35 = (height*0.3)+(height*0.05);
    let height65 = (height*0.6)+(height*0.05);

    // console.log("Screen W", width)
    // console.log("Screen H", height)
    // console.log("Screen H", height35)
    // console.log("Screen H", height-height35)

    let codeSale = $state("");
    let dataByBarcode = $state(null);
    let loading = $state(true);
    let result_data = $state({
        code: null,
        customer_id: null,
        // PPN: 0,
        discount: 0,
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
                qty: null
            }
        ],
    });

    async function get_code_txr() {
        loading = true
        try {
            codeSale = await invoke("sale_get_code_txr")
            console.log(codeSale, "=====")

        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }
    }
	console.log(codeSale)


    onMount(get_code_txr)
    onMount(fetchListConsumer)

    async function getByBarcodeDetail(barcode) {
        loading = true
        try {
            const result = await invoke("get_items_by_barcode", {
                barcode: barcode
            })

            dataByBarcode = result.data

        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }

    }

    let headers = [
        {name: 'Code Barang', value: 'code', type: 'input-code'},
        {name: 'Nama Barang', value: 'items_name', type: 'text1'},
        {name: 'Satuan', value: 'items_unit', type: 'select1'},
        {name: 'Harga', value: 'items_price', type: 'parse-number'},
        {name: 'QTY', value: 'qty', type: 'input-number'},
        {name: 'Total', value: 'total', type: 'parse-number'},
    ];
    let optionSelectItem = [
        {name: "pcs"},
        {name: "box"},
        {name: "gt"},
        {name: "gt"},
    ]
    let dateN = new Date();
    // let total = $state(0);
    let index_row = $state(0);
    let onFocus = {row: null, cell: null};
    let refs = [];
    let openModalAdd = $state(false)


    function formatDate(date, format = "yyyy-mm-dd") {
        const d = new Date(date);
        const day = String(d.getDate()).padStart(2, "0");
        const month = String(d.getMonth() + 1).padStart(2, "0");
        const year = d.getFullYear();

        if (format === "dd-mm-yyyy") {
            return `${day}-${month}-${year}`;
        }
        return `${year}-${month}-${day}`;
    }

    function close() {
        dispatch('close');
    }

    async function refresh() {
        result_data.items = [
            {
                code: null,
                items_id: null,
                items_name: null,
                items_unit: 0,
                items_price: 0,
                total: 0,
                qty: 0
            }
        ];
        result_data.total = 0

        await tick(); // tunggu DOM siap
        refs[0]?.[0]?.focus();
        get_code_txr()
        console.log("1 ===================", codeSale)
    }

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/\./g, ""));
    };

    function removeIndexTable(i) {
        result_data.items.splice(i, 1);

        result_data.total = result_data.items.reduce((acc, item) => acc + item.total, 0)
    }

    function selectUnitItem(e) {
        console.log(e)
    }

    function ensureRef(r, c) {
        if (!refs[r]) refs[r] = [];
        if (!refs[r][c]) refs[r][c] = null;
    }

    function savekeydown(e) {
        if (e.key === "F8" || e === "F8") {
            if (result_data.total == 0) {
                refs[0]?.[0]?.focus();
                return;
            }

            result_data.total = result_data.total;
            result_data.code = codeSale;
            result_data.items = result_data.items;

            console.log(result_data)

            openModalAdd = !openModalAdd
            return;
        }

        if (e.key === 'Escape' && !openModalAdd) {
            close();
            return;
        }
    }


    async function keydown(e, r, c, type) {
        if (e.key === "Enter") {
            if (c === 4) {
                e.preventDefault();

                onFocus.row = r + 1;
                onFocus.cell = 0;

                await tick(); // tunggu DOM siap
                refs[r + 1]?.[0]?.focus();
            }

            if (type === "qty") {
                await getByBarcodeDetail(result_data.items[index_row].code)

                if (dataByBarcode.qty < result_data.items[index_row].qty) {
                    result_data.items[index_row].qty = dataByBarcode.qty
                }

                result_data.items[index_row].total = parseNumber(result_data.items[index_row].items_price) * parseNumber(result_data.items[index_row].qty);
                result_data.total = result_data.items.reduce((acc, item) => acc + item.total, 0);
                result_data.total_item += result_data.items[index_row].qty;
                return
            }

            handleCodeInput(e, e.target.value, r, c)
        }
    }

    async function handleCodeInput(e, code, index_row, index_cell) {
        if (!code) return;

        const dupIndex = result_data.items.findIndex(
            (item, i) => item.code === code && i !== index_row
        );

        // ✅ Jika code sudah ada → tambah qty

        if (dupIndex !== -1) {

            result_data.items[dupIndex].qty += 1;
            result_data.items[dupIndex].total =
                parseNumber(result_data.items[dupIndex].items_price) *
                parseNumber(result_data.items[dupIndex].qty);

            result_data.total = result_data.items.reduce((acc, item) => acc + item.total, 0);

            // result_data.total_item += result_data.items[index_row].qty

            // reset baris aktif
            result_data.items[index_row] = {
                code: null,
                items_id: null,
                items_name: null,
                items_unit: null,
                items_price: null,
                total: null,
                qty: null
            };

            return;
        } else {
            if (result_data.items.length - 1 === index_row) {
                result_data.items.push(
                    {
                        code: null,
                        items_id: null,
                        items_name: null,
                        items_unit: null,
                        items_price: null,
                        total: null,
                        qty: null
                    }
                )
            }

            if (index_row !== null) {
                await getByBarcodeDetail(result_data.items[index_row].code)

                if (dataByBarcode == null) {
                    e.preventDefault();

                    result_data.items[index_row].code = null
                    onFocus.row = index_row;
                    onFocus.cell = index_cell;

                    await tick(); // tunggu DOM siap
                    refs[index_row]?.[index_cell]?.focus();
                    return
                }
                ;

                result_data.items[index_row].items_id = dataByBarcode.id;
                result_data.items[index_row].items_name = dataByBarcode.name;
                result_data.items[index_row].items_unit = dataByBarcode.type_unit;
                result_data.items[index_row].items_price = dataByBarcode.price_sell;
                result_data.items[index_row].qty = 1;
                result_data.items[index_row].total = parseNumber(dataByBarcode.price_sell) * parseNumber(result_data.items[index_row].qty);

                result_data.total_item += result_data.items[index_row].qty

                result_data.total = result_data.items.reduce((acc, item) => acc + item.total, 0);

                dataByBarcode = null
                e.preventDefault();

                onFocus.row = index_row + 1;
                onFocus.cell = index_cell;

                await tick(); // tunggu DOM siap
                refs[index_row + 1]?.[index_cell]?.focus();
            }
        }


    }

    function handlerCloseModalSave(ex) {
        openModalAdd = false
    }


    $effect(async () => {
        if (open) {
            await tick();
            refs[0]?.[0]?.focus();
        }
    })
</script>
{#if openModalAdd}
	<ModalAdd open={openModalAdd} res={result_data}  on:close={handlerCloseModalSave} on:save={refresh}/>
{/if}

<div class="absolute z-2 h-full w-full bg-gray-300 z-10 top-0 left-0 p-3 overflow-x-auto " tabindex="0" onkeydown={(e)=> savekeydown(e)}>
	<div class="flex bg-gray-100  justify-center mb-2 rounded-2xl">
		<p class="p-3 text-2xl"> Penjualan Barang </p>
	</div>
	<div class="flex justify-between bg-white p-4 rounded-sm">
		<div class="bg-white p-1 pr-5 rounded-sm border-1 border-solid">
			<table class="border-all border-default-medium">
				<tbody class="p-5">
				<tr class="border-b border-default-medium">
					<td class="p-2">No</td>
					<td class="p-2">:</td>
					<td class="p-2">
						<input
								value="{codeSale}"
								type="text"
								id="visitors"
								class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
								placeholder=""
								readonly/>
					</td>
				</tr>
				<tr class="border-b border-default-medium">
					<td class="p-2">Pelanggan</td>
					<td class="p-2">:</td>
					<td class="p-2">
						<select
								bind:value={result_data.customer_id}
								id="small"
								class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body mb-4">
							<option selected>Choose a country</option>
							{#each listDataConsumer as elm }
								<option value="{elm.id}">{elm.name}</option>
							{/each}
<!--							<option value="CA">Canada</option>-->
<!--							<option value="FR">France</option>-->
<!--							<option value="DE">Germany</option>-->
						</select>
					</td>
				</tr>
				<tr class="border-b border-default-medium">
					<td class="p-2">Tanggal</td>
					<td class="p-2">:</td>
					<td class="p-2">
						<input
								value="{formatDate(dateN, 'dd-mm-yyyy')}"
								type="text"
								id="visitors"
								class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
								placeholder=""
								disabled/>
					</td>
				</tr>
				</tbody>
			</table>
		</div>

		<div class="flex w-250 rounded-sm h-20 items-center pt-2 pl-20 mt-15 font-30" style="font-size: 50px">
			<p class="">Total : </p>
			<p class=""> {parseNumber(result_data.total).toLocaleString('id-ID', {
                style: 'currency',
                currency: 'IDR'
            })}</p>
		</div>
	</div>

	<div class="mr-5 w-full mt-5">
		<div class="table-auto md:table-fixed  bg-white h-max p-3 rounded-lg">
			<div class="relative overflow-x-auto bg-neutral-primary-soft shadow-xs rounded-base border border-default overflow-y-auto"
				 style="height: {height65}px">
				<table class="w-full text-sm text-left rtl:text-right text-body">
					<thead class="bg-neutral-secondary-soft border-b border-default sticky top-0">
					<tr>
						{#each headers as header}
							<th scope="col" class="px-6 py-3 font-medium">
								{header['name']}
							</th>
						{/each}
						<th scope="col" class="px-6 py-3 font-medium">
							Action
						</th>
					</tr>
					</thead>
					<tbody>
					{#each result_data.items as item, i (i)}
						<tr class="odd:bg-neutral-primary  border-b border-default">
							{#each headers as header, hi (hi)}
								<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
									{#if header['type'] === 'input-number'}
										<input
												bind:value={item[header.value]}
												oninput={() =>(index_row = i) }
												onkeydown={(e)=> keydown(e, i, hi, "qty")}

												type="number"
												id="visitors"
												size="10"
												class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-20 px-2.5 py-2 shadow-xs placeholder:text-body"
												placeholder=""
										/>
									{:else if header['type'] === 'input-code'}
										{@const _ = ensureRef(i, hi)}
										<input
												type="text"
												id="visitors"
												class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
												bind:value={item[header.value]}
												bind:this={refs[i][hi]}

												onkeydown={(e)=> keydown(e, i, hi)}
												placeholder=""
										/>
									{:else if header['type'] === 'select'}
										<select
												bind:value={item[header.value]}
												onchange={(e) => selectUnitItem(e.target.value)}
												id="small"
												class="block w-20 px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body mb-4">
											{#each optionSelectItem as os}
												<option value="{os.name}" class="p-3">{os.name}</option>

											{/each}
											<!--												<option selected>Choose a country</option>-->
											<!--												<option value="CA">Canada</option>-->
											<!--												<option value="FR">France</option>-->
											<!--												<option value="DE">Germany</option>-->
										</select>
									{:else if header['type'] === 'number'}
										<input
												bind:value={item[header.value]}
												type="number"
												id="visitors"
												size="10"
												class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
												placeholder=""
												readonly
										/>
									{:else if header['type'] === 'text'}
										<input
												bind:value={item[header.value]}
												type="text"
												id="visitors"
												size="10"
												class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
												placeholder=""
												readonly
										/>
									{:else if header['type'] === 'parse-number'}
										{parseNumber(item[header.value]).toLocaleString('id-ID', {
                                            style: 'currency',
                                            currency: 'IDR'
                                        })}
									{:else }
										{item[header.value]}

									{/if}
								</th>
							{/each}

							<td class="px-6 py-4">
								<button
										type="button"
										onclick="{() => removeIndexTable(i)}"
										class="text-white bg-danger box-border border border-transparent hover:bg-danger-strong focus:ring-4 focus:ring-danger-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
									<img src="/icon/trash.svg" class="h-6 w-6 p-1 justify-center"
										 alt="Tauri Logo"/>
								</button>

							</td>
						</tr>
					{/each}

					</tbody>
				</table>
			</div>

		</div>
	</div>

	<div class="flex justify-between mt-3">

		<div class="">
			<button onclick="{close}" type="button"
					class=" p-3 text-body bg-neutral-secondary-medium box-border border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading focus:ring-4 focus:ring-neutral-tertiary shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
				<div class="flex">
					<img src="/icon/angle-double-small-left.svg" class="w-5 h-5" alt="back"/> Kembali (esc)
				</div>
			</button>
			<button onclick="{refresh}" type="button"
					class=" p-3 text-body bg-neutral-secondary-medium box-border border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading focus:ring-4 focus:ring-neutral-tertiary shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
				<div class="flex">
					<img src="/icon/refresh.svg" class="w-5 h-5 pr-2" alt="back"/> Reset
				</div>
			</button>
		</div>
		<div class="order-last">

			{#if result_data.total == 0}
				<button
						class="text-white legend-mouseover-inactive bg-gray-500 box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none"
						type="button"
						onclick="{() => savekeydown('F8')}"
						disabled
				>
					<p class="text-sm">Simpan (F8)</p>
				</button>
			{:else }
				<button
						class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none"
						type="button"
						onclick="{() => savekeydown('F8')}"
				>
					<p class="text-sm">Simpan (F8)</p>
				</button>
			{/if}

		</div>
	</div>

</div>
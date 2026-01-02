<script>
    import {createEventDispatcher} from 'svelte';
    import items from '$lib/data/items.json';
    import {tick} from "svelte";
    import ModalAdd from "./ModalAddSale.svelte"
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";


    const dispatch = createEventDispatcher();
    let {
        codeSale = "CODE-01",
        open = false,
        data = [
            {code: "", name: "", unit: "", price: 0, qty: 1, total: 0},
        ]


    } = $props()

    let dataByBarcode = $state(null)
    let loading = $state(true)

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
        {name: 'Nama Barang', value: 'name', type: 'text1'},
        {name: 'Satuan', value: 'unit', type: 'select1'},
        {name: 'Harga', value: 'price', type: 'parse-number'},
        {name: 'QTY', value: 'qty', type: 'input-number'},
        {name: 'Total', value: 'total', type: 'parse-number'},
    ];
    let optionSelectItem = [
        {name: "pcs"},
        {name: "box"},
        {name: "gt"},
        {name: "gt"},
    ]
    let data_c = $state(structuredClone(data));
    let dateN = new Date();
    let total = $state(0);
    let index_row = $state(0);
    let unitItem = $state("");
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

    function refresh() {
        data_c = structuredClone(data);
        total = 0
    }

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/\./g, ""));
    };

    function removeIndexTable(i) {
        data_c.splice(i, 1);

        total = parseNumber(data_c.reduce((acc, item) => acc + item.total, 0)).toLocaleString('id-ID', {
            style: 'currency',
            currency: 'IDR'
        });
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
            openModalAdd = !openModalAdd
            return;
        }

        if (e.key === 'Escape' && !openModalAdd) {
            close();
            return;
        }
    }


    async function keydown(e, r, c) {
        if (e.key === "Enter") {
            if (c === 4) {
                e.preventDefault();

                onFocus.row = r + 1;
                onFocus.cell = 0;

                await tick(); // tunggu DOM siap
                refs[r + 1]?.[0]?.focus();
            }

            handleCodeInput(e, e.target.value, r, c)
        }
    }

    async function handleCodeInput(e, code, index_row, index_cell) {
        if (!code) return;

        const dupIndex = data_c.findIndex(
            (item, i) => item.code === code && i !== index_row
        );

        // ✅ Jika code sudah ada → tambah qty

        if (dupIndex !== -1) {

            data_c[dupIndex].qty += 1;
            data_c[dupIndex].total =
                parseNumber(data_c[dupIndex].price) *
                parseNumber(data_c[dupIndex].qty);

            total = parseNumber(data_c.reduce((acc, item) => acc + item.total, 0)).toLocaleString('id-ID', {
                style: 'currency',
                currency: 'IDR'
            });

            // reset baris aktif
            data_c[index_row] = {
                code: "",
                name: "",
                unit: "",
                price: 0,
                qty: 1,
                total: 0
            };

            return;
        } else {
            if (data_c.length - 1 === index_row) {
                data_c.push(
                    {code: "", name: "", unit: "", price: 0, qty: 1, total: 0}
                )
            }

            if (index_row !== null) {
                await getByBarcodeDetail(data_c[index_row].code)

                if (dataByBarcode == null) {
                    e.preventDefault();

                    data_c[index_row].code = null
                    onFocus.row = index_row;
                    onFocus.cell = index_cell;

                    await tick(); // tunggu DOM siap
                    refs[index_row]?.[index_cell]?.focus();
                    return
                }
                ;

                data_c[index_row].name = dataByBarcode.name;
                data_c[index_row].unit = dataByBarcode.type_unit;
                data_c[index_row].price = dataByBarcode.price_sell;
                data_c[index_row].total = parseNumber(dataByBarcode.price_sell) * parseNumber(data_c[index_row].qty);

                total = parseNumber(data_c.reduce((acc, item) => acc + item.total, 0)).toLocaleString('id-ID', {
                    style: 'currency',
                    currency: 'IDR'
                });

                dataByBarcode = null
                e.preventDefault();

                onFocus.row = index_row + 1;
                onFocus.cell = index_cell;

                await tick(); // tunggu DOM siap
                refs[index_row + 1]?.[index_cell]?.focus();
            }
        }


    }


    $effect(() => {
    })
</script>
{#if openModalAdd}
	<ModalAdd open={openModalAdd} on:close={() => savekeydown("F8")}/>
{/if}

{#if open}
	<div class="absolute h-full w-full bg-gray-300 z-10 top-0 left-0 p-3 " tabindex="0" onkeydown={(e)=> savekeydown(e)}>
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
						<td class="p-2">Consumer</td>
						<td class="p-2">:</td>
						<td class="p-2">
							<select
									id="small"
									class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body mb-4">
								<option selected>Choose a country</option>
								<option value="US">United States</option>
								<option value="CA">Canada</option>
								<option value="FR">France</option>
								<option value="DE">Germany</option>
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
				<p class=""> {total}</p>
			</div>
		</div>

		<div class="flex justify-between">

			<div class="">
				<button onclick="{close}" type="button"
						class=" p-3 m-5 text-body bg-neutral-secondary-medium box-border border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading focus:ring-4 focus:ring-neutral-tertiary shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
					<div class="flex">
						<img src="/icon/angle-double-small-left.svg" class="w-5 h-5" alt="back"/> Kembali (esc)
					</div>
				</button>
				<button onclick="{refresh}" type="button"
						class=" p-3 m-5 text-body bg-neutral-secondary-medium box-border border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading focus:ring-4 focus:ring-neutral-tertiary shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
					<div class="flex">
						<img src="/icon/refresh.svg" class="w-5 h-5 pr-2" alt="back"/> Refresh
					</div>
				</button>
			</div>
			<div class="order-last">
				<button
						class="p-3 m-5 text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium rounded-base text-sm px-4 py-2.5 text-center leading-5"
						type="button"
						onclick="{() => savekeydown('F8')}"
				>
					<p class="text-3xl">Simpan (F8)</p>
				</button>

			</div>
		</div>


		<div class="mr-5 w-full">
			<div class="table-auto md:table-fixed  bg-white h-max p-3 rounded-lg">


				<div class="relative overflow-x-auto bg-neutral-primary-soft shadow-xs rounded-base border border-default h-[65vh] overflow-y-auto">
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
						{#each data_c as item, i (i)}
							<tr class="odd:bg-neutral-primary  border-b border-default">
								{#each headers as header, hi (hi)}
									<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
										{#if header['type'] === 'input-number'}
											<input
													bind:value={item[header.value]}
													oninput={() =>(index_row = i) }
													onkeydown={(e)=> keydown(e, i, hi)}

													type="number"
													id="visitors"
													size="10"
													class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
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
	</div>
{/if}
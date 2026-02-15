<script>
    let height = window.innerHeight;

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/\./g, ""));
    };

    import {invoke} from "@tauri-apps/api/core";
    import {createEventDispatcher, onMount, tick} from 'svelte';

    const dispatch = createEventDispatcher();


    let {
        res,
        data_result =
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
        open = false,
    } = $props();


    let focus_data;
    let loading = $state(false);


    //
    async function saveSale(data) {
        loading = true
        try {
            const result = await invoke("sale_create", {
                data: data
            })

        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }

    }

    function handlerSave() {
        saveSale(res)
        dispatch("save");
        handlerClose()
    }

    function handlerAllTotalPayment() {
        res.payment = res.total;
        res.change = res.payment - res.total;
    }

    function handlerInputPaymaent() {
        res.change = res.payment - res.total;
    }

    function handlerClose() {
        res.payment = 0;
        res.change = res.payment - res.total;
        dispatch("close", false);
    }

    $effect(async () => {
        if (open) {
            await tick();
            res.change = res.payment - res.total;
            focus_data.focus();
        }
    })
</script>

<div on:click={handlerClose} class="absolute bg-black w-screen h-screen z-11 top-0" style="opacity: 0.95">
</div>
<div class="absolute inset-0 flex items-center justify-center z-[11] " on:click={handlerClose}>
	<div class=" bg-white min-w-200  rounded-2xl" on:click|stopPropagation>
		<div class="relative flex items-center justify-center py-6 ">
			<span class="text-3xl font-semibold">Simpan Data</span>

			<button
					on:click={handlerClose}
					class="absolute right-4 top-4 hover:bg-gray-200 p-3 rounded-full"
			>
				X
			</button>
		</div>

		<!-- CONTENT -->
		<div class="p-3 text-2xl">
			<table class="ml-20">
				<tbody>
				<tr>
					<td class="p-3">Total</td>
					<td class="p-2">:</td>
					<td class=" p-3">
						{parseNumber(res.total).toLocaleString('id-ID', {
                            style: 'currency',
                            currency: 'IDR'
                        })}
					</td>
				</tr>
				<tr>
					<td class="p-3">Kembalian</td>
					<td class="p-2">:</td>
					<td class=" p-3">
						{parseNumber(res.change).toLocaleString('id-ID', {
                            style: 'currency',
                            currency: 'IDR'
                        })}
					</td>
				</tr>
				<tr>
					<td class="p-3">Pembayaran</td>
					<td class="p-2">:</td>
					<td class=" p-3">
						<div class="flex shadow-xs rounded-base -space-x-0.5">
							<input
									bind:this={focus_data}
									bind:value={res.payment}
									on:input={handlerInputPaymaent}
									type="number" id="search-dropdown input-group-1" class="rounded-l-lg px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-lg focus:ring-brand focus:border-brand block w-full placeholder:text-body"
									placeholder="Masukkan Pembayaran"
									required
							>
							<button
									on:click={handlerAllTotalPayment}
									type="button"
									class="bg-blue-600 text-white hover:bg-blue-700 p-4 rounded-r-lg"
							>
								all
							</button>
						</div>
					</td>
				</tr>
				</tbody>
			</table>
		</div>

		<!-- FOOTER -->
		<div class=" p-4 flex justify-center gap-3">
			<button
					on:click={handlerClose}
					class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300"
			>
				Batal
			</button>
			<button
					on:click={handlerSave}
					class="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700">
				Simpan
			</button>
		</div>
	</div>
</div>
<script>
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    import {invoke} from "@tauri-apps/api/core";
    import {onMount, tick} from "svelte";

    let {
        data_sale = {
            code: null,
            customer_id: null,
            PPN: 0,
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
                    items_unit: 0,
                    items_price: 0,
                    total: 0,
                    qty: 0
                }
            ],
        },

        total_price = 0,
        open = false,
    } = $props();

    let payment = $state()
    let change = $state(0)
    let loading = $state(true)
    let data = $state([])

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/\./g, ""));
    };

    function hoverAllTotal() {
        payment = total_price;
        change = payment - total_price
        data_sale.change = change;
        data_sale.payment = payment;

        return
    }

    async function fetchData() {
        loading = true
        try {
            const result = await invoke("get_items_by_id", {
                id: id
            })

            data = result.data
        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }

    }

    async function deleteData() {
        loading = true
        try {
            const result = await invoke("items_delete11", {
                id: id
            })

            data = result.data

            console.log(result)
        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;

            closeModal(true)
        }

    }

    onMount(fetchData)


    function closeModal(confirm) {
        let c = false
        if (typeof (confirm) !== "boolean") {
            c = true
        }

        dispatch('close', {confirm: c});
    }

    async function confirmModal() {
        await deleteData()
    }

    function changeFn(v) {
        change = payment - total_price

        data_sale.change = change;
        data_sale.payment = payment;
    }

    let paymentThis;

    $effect(async () => {
        if (open) {
            await tick();
            paymentThis?.focus();
        }
    });

</script>

{#if open}
	<!-- Overlay -->
	<div
			class="fixed inset-0 z-50 flex items-center justify-center z-2
           bg-black" style="opacity: 0.9"
			on:click={closeModal}
	>
		<!-- Modal -->
		<div
				class="relative bg-neutral-primary-soft border border-default
             rounded-base shadow-sm p-4 md:p-6 w-full max-w-200"
				on:click|stopPropagation
		>
			<!-- Close button -->
			<button
					on:click={closeModal}
					class="absolute top-3 right-3 text-body
               hover:bg-neutral-tertiary rounded-base
               w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<!-- Content -->
			<div class="text-center">
				<p class="text-4xl font-medium text-gray-900 mb-10">
					Simpan Data
				</p>
				<!--				<svg class="mx-auto mb-4 text-fg-disabled w-12 h-12" viewBox="0 0 24 24" fill="none">-->
				<!--					<path stroke="currentColor" stroke-width="2"-->
				<!--						  d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0Z"/>-->
				<!--				</svg>-->

				<div class="text-2xl mb-10 text-left ml-10">
					<table>
						<tbody>
						<tr>
							<td class="p-3">Total</td>
							<td class="p-3">:</td>
							<td class="p-3">{parseNumber(total_price).toLocaleString('id-ID', {
                                style: 'currency',
                                currency: 'IDR'
                            })}</td>
						</tr>
						<tr>
							<td class="p-3">Kembalian</td>
							<td class="p-3">:</td>
							<td class="p-3">{parseNumber(change).toLocaleString('id-ID', {
                                style: 'currency',
                                currency: 'IDR'
                            })}</td>
						</tr>
						<tr>
							<td class="p-3">Bayar</td>
							<td class="p-3">:</td>
							<td class="p-3">
								<div class="flex shadow-xs rounded-base -space-x-0.5 h-15">
									<input
											bind:this={paymentThis}
											bind:value={payment}
											on:input={changeFn}

											type="search"
											id="search-dropdown input-group-1"
											class="rounded-l-xl text-2xl font-medium  px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading focus:ring-brand focus:border-brand block w-full placeholder:text-body"
											placeholder="Search for products"
											required>
									<button
											on:click={hoverAllTotal}
											type="button"
											class="inline-flex items-center  text-white bg-brand hover:bg-brand-strong box-border border border-transparent focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-e-base text-sm px-4 py-2.5 focus:outline-none"
									>
										All
									</button>
								</div>

							</td>
						</tr>

						</tbody>
					</table>
				</div>

				<div class="flex gap-4 justify-center">
					<button
							on:click={confirmModal}
							class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base">
						Ya
					</button>

					<button
							on:click={closeModal}
							class="bg-neutral-secondary-medium
                   hover:bg-neutral-tertiary-medium
                   px-4 py-2.5 rounded-base">
						Tidak
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

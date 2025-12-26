<script>
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    import Table from '$lib/component/table/Table.svelte'

    const headers = [
        {name: 'Code', value: 'barcode'},
        {name: 'Nama Barang', value: 'name'},
        {name: 'Satuan', value: 'price.array.0.type'},
        {name: 'Jumlah', value: 'qty'},
        {name: 'Harga', value: 'price.array.0.price_sell'},
    ]
    // let data = [
    //     {code: "abc-123", name: "handphone", unit: "pcs", qty: 100, price: 200000,}
    // ]
    let loading = $state(true)
    let searchDb = $state(null)
    let data = $state([])

    function search(e) {
        // e.preventDefault();
        if (e.key === "Enter" || e === "enter") {
            if (e === "enter") {
                fetchData()
            } else {
                searchDb = e.target.value
                fetchData()
            }
        }
    }

    async function fetchData() {
        loading = true
        try {
            const result = await invoke("items_get", {
                filter: {
                    search: searchDb,
                }
            })

            data = result.result

            console.log(data[0])
        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }

    }

    onMount(fetchData)

    let open = $state(false)
</script>
<div class="w-full">
	<div class="ml-4">
		<div class="p-3">
			<button type="button"
					onclick="{() => open = true}"
					class="text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium rounded-base text-sm px-4 py-2.5 text-center leading-5">
				Tambah Data
			</button>
		</div>
	</div>
	{#if (loading)}
		<div class="flex w-full h-[80vh] mt-3 rounded-2xl bg-white justify-center items-center">
			<img src="/icon/gift/loading.gif" class="h-50 w-50 p-1 justify-center" alt="Tauri Logo"/>
		</div>
	{:else}
		<form class="mt-3 ml-3 w-100" onsubmit={(e)=>(search("enter"))}>
			<label for="search" class="block mb-2.5 text-sm font-medium text-heading sr-only ">Search</label>
			<div class="relative">
				<div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
					<svg class="w-4 h-4 text-body" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
						<path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"/>
					</svg>
				</div>
				<input
						type="search"
						id="search"
						bind:value={searchDb}
						class="block w-full p-3 ps-9 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
						placeholder="Search"
						onkeydown={(e)=>(search(e))}
						required/>
				<button
						type="submit"
						class="absolute end-1.5 bottom-1.5 text-white bg-brand hover:bg-blue-950 box-border border border-transparent focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded text-xs px-3 py-1.5 focus:outline-none"
				>
					Search
				</button>
			</div>
		</form>
		<Table headers={headers} data={data}/>
	{/if}
</div>

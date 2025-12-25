<script>
    import Sale from "./Sale.svelte"
    import Table from "$lib/component/table/Table.svelte";

    let open = $state(false)
    let header = [
        {name: 'Code Order', value: 'code'},
        {name: 'Tanggal', value: 'date'},
        {name: 'Pelanggan', value: 'consumer'},
    ];

    let dataTable = [
        {code: '0001', date: "02-01-2026", consumer: "totipal"},
    ];

</script>

<div class="w-[84.5vw]">

	<div class="p-3">
		<button type="button"
				onclick="{() => open = true}"
				class="text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium rounded-base text-sm px-4 py-2.5 text-center leading-5">
			Tambah Data
		</button>
	</div>

	<Sale {open} on:close={() => open = false}/>

	<Table headers={header} data={dataTable}/>


	<nav aria-label="Page navigation example" class="flex items-center space-x-4">
		<ul class="flex -space-x-px text-sm">
			<li>
				<a href="#"
				   class="flex items-center justify-center text-body bg-neutral-secondary-medium border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading shadow-xs font-medium leading-5 rounded-s-base text-sm px-3 h-9 focus:outline-none">Previous</a>
			</li>
			<li>
				<a href="#"
				   class="flex items-center justify-center text-body bg-neutral-secondary-medium border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading shadow-xs font-medium leading-5 text-sm w-9 h-9 focus:outline-none">1</a>
			</li>
			<li>
				<a href="#"
				   class="flex items-center justify-center text-body bg-neutral-secondary-medium border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading shadow-xs font-medium leading-5 text-sm w-9 h-9 focus:outline-none">2</a>
			</li>
			<li>
				<a href="#" aria-current="page"
				   class="flex items-center justify-center text-fg-brand bg-neutral-tertiary-medium box-border border border-default-medium hover:text-fg-brand font-medium text-sm w-9 h-9 focus:outline-none">3</a>
			</li>
			<li>
				<a href="#"
				   class="flex items-center justify-center text-body bg-neutral-secondary-medium border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading shadow-xs font-medium leading-5 text-sm w-9 h-9 focus:outline-none">4</a>
			</li>
			<li>
				<a href="#"
				   class="flex items-center justify-center text-body bg-neutral-secondary-medium border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading shadow-xs font-medium leading-5 text-sm w-9 h-9 focus:outline-none">5</a>
			</li>
			<li>
				<a href="#"
				   class="flex items-center justify-center text-body bg-neutral-secondary-medium border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading shadow-xs font-medium leading-5 rounded-e-base text-sm px-3 h-9 focus:outline-none">Next</a>
			</li>
		</ul>
		<form class="mx-auto">
			<div class="flex items-center space-x-2">
				<label for="visitors" class="text-sm font-medium text-body shrink-0">Go to</label>
				<input type="text" id="visitors"
					   class="bg-neutral-secondary-medium w-10 border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block px-2.5 py-2 shadow-xs placeholder:text-body"
					   placeholder="99" required/>
				<span class="text-sm font-medium text-body">page</span>
			</div>
		</form>
	</nav>

</div>





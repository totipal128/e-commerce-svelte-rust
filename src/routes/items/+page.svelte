<script>
    import Table from '$lib/component/table/Table.svelte'

    const headers = [
        {name: 'Code', value: 'code'},
        {name: 'Nama Barang', value: 'name'},
        {name: 'Satuan', value: 'unit'},
        {name: 'Jumlah', value: 'qty'},
        {name: 'Harga', value: 'price'},
    ]
    let data = [
        {code: "abc-123", name: "handphone", unit: "pcs", qty: 100, price: 200000,}
    ]

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
    <Table headers={headers} data={data}/>
</div>

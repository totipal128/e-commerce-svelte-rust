<script>
    import {createEventDispatcher} from 'svelte';
    import items from '$lib/data/items.json';


    const dispatch = createEventDispatcher();
    let {
        codeSale = "CODE-01",
        open = false,
        data = [
            {code: "", name: "", unit: "", price: 0, qty: 1, total: 0},
        ]


    } = $props()

    let headers = [
        {name: 'Code Barang', value: 'code', type: 'input-code'},
        {name: 'Nama Barang', value: 'name', type: 'text'},
        {name: 'Satuan', value: 'unit', type: 'select'},
        {name: 'Harga', value: 'price', type: 'number'},
        {name: 'QTY', value: 'qty', type: 'input-number'},
        {name: 'Total', value: 'total', type: 'number'},
    ];
    let data_c = $state(structuredClone(data));
    let date = new Date();
    let total = $state(0)
    let index_row = $state(0);

    function close() {
        dispatch('close');
    }

    function refresh() {
        data_c = structuredClone(data);
    }

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/\./g, ""));
    };

    $effect(() => {
        if (data_c.length - 1 === index_row) {
            data_c.push(
                {code: "", name: "", unit: "", price: 0, qty: 1, total: 0}
            )
        }

        if (index_row !== null) {
            let found = items.find(item => item.code === data_c[index_row].code);

            if (found === undefined) return;

            console.log(found.name)
            data_c[index_row].name = found.name;
            data_c[index_row].unit = found.unit;
            data_c[index_row].price = found.price;
            data_c[index_row].total = parseNumber(found.price) * parseNumber(data_c[index_row].qty);

            total = parseNumber(data_c.reduce((acc, item) => acc + item.total, 0)).toLocaleString('id-ID', {
                style: 'currency',
                currency: 'IDR'
            });
        }
    })
</script>

{#if open}
    <div class="absolute h-full w-full bg-gray-300 z-100 top-0 left-0 p-3 ">
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
                                    value="{date.toLocaleDateString()}"
                                    type="date"
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

        <button onclick="{close}" type="button"
                class=" p-3 m-5 text-body bg-neutral-secondary-medium box-border border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading focus:ring-4 focus:ring-neutral-tertiary shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
            <div class="flex">
                <img src="/icon/angle-double-small-left.svg" class="w-5 h-5" alt="back"/> Kembali
            </div>
        </button>

        <button onclick="{refresh}" type="button"
                class=" p-3 m-5 text-body bg-neutral-secondary-medium box-border border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading focus:ring-4 focus:ring-neutral-tertiary shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
            <div class="flex">
                <img src="/icon/refresh.svg" class="w-5 h-5 pr-2" alt="back"/> Refresh
            </div>
        </button>


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
                                {#each headers as header}
                                    <th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
                                        {#if header['type'] === 'input-number'}
                                            <input
                                                    bind:value={item[header.value]}
                                                    oninput={() =>(index_row = i) }
                                                    type="number"
                                                    id="visitors"
                                                    size="10"
                                                    class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
                                                    placeholder=""
                                            />
                                        {:else if header['type'] === 'input-code'}
                                            <input
                                                    type="text"
                                                    id="visitors"
                                                    class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
                                                    bind:value={item[header.value]}
                                                    oninput={() =>(index_row = i) }
                                                    placeholder=""
                                            />
                                        {:else if header['type'] === 'select'}
                                            <select
                                                    id="small"
                                                    class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body mb-4">
                                                <option selected>Choose a country</option>
                                                <option value="US">United States</option>
                                                <option value="CA">Canada</option>
                                                <option value="FR">France</option>
                                                <option value="DE">Germany</option>
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
                                        {:else }
                                            <input
                                                    bind:value={item[header.value]}
                                                    type="text"
                                                    id="visitors"
                                                    size="10"
                                                    class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-2.5 py-2 shadow-xs placeholder:text-body"
                                                    placeholder=""
                                                    readonly
                                            />
                                        {/if}
                                    </th>
                                {/each}

                                <td class="px-6 py-4">
                                    <button
                                            type="button"
                                            onclick="{() => data_c.splice(i, 1)}"
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

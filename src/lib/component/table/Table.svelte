<script>
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    let {

        data = [
            {name: 'John', age: 25},
            {name: 'John dasdsadsadsads', age: 26},
            {name: 'John', age: 27},
            {name: 'John', age: 28},
            {name: 'John', age: 29},
            {name: 'John', age: 30},
        ],

        headers = [
            {name: 'Name', value: 'name'},
            {name: 'Age', value: 'age'}
        ]

    } = $props()

    function getValue(item, path) {
        let splitText = path.split(".");

        splitText = splitText.map(key => (key === "array" ? null : key))
            .filter(key => key !== null);

        return splitText.reduce((obj, key) => {
            // jika key adalah number → akses array
            if (obj === undefined || obj === null) return undefined;
            let index = Number(key);
            return !isNaN(index) ? obj[index] : obj[key];
        }, item)
    }

    function handleRemove() {
        dispatch('remove', {id: 1})
    }
</script>
<div class="m-3">
	<div class="table-auto md:table-fixed  bg-gray-200 h-max p-3 rounded-lg">


		<div class="relative overflow-x-auto bg-neutral-primary-soft shadow-xs rounded-base border border-default">
			<table class="w-full text-sm text-left rtl:text-right text-body">
				<thead class="bg-neutral-secondary-soft border-b border-default">
				<tr>
					<th scope="col" class="px-6 py-3 font-medium">
						#
					</th>
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
				{#if data.length > 0}
					{#each data as item, index}
						<tr class="odd:bg-neutral-primary  border-b border-default">
							<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
								{index + 1}
							</th>

							{#each headers as header}
								<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
									{#if header['value'].split(".").length > 1}
										{getValue(item, header.value)}
									{:else}
										{item[header['value']]}
									{/if}
								</th>
							{/each}

							<td class="px-6 py-4">
								<button type="button"
										class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
									<img src="/icon/search-alt.png" class="h-6 w-6 p-1 justify-center" alt="Tauri Logo"/>
								</button>

								<button
										type="button"
										class=" w-10text-white bg-warning box-border border border-transparent hover:bg-warning-strong focus:ring-4 focus:ring-warning-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
									<img src="/icon/pen-field.svg" class="h-6 w-6 p-1 justify-center" alt="Tauri Logo"/>
								</button>

								<button onclick="{handleRemove}" type="button"
										class="text-white bg-danger box-border border border-transparent hover:bg-danger-strong focus:ring-4 focus:ring-danger-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
									<img src="/icon/trash.svg" class="h-6 w-6 p-1 justify-center" alt="Tauri Logo"/>
								</button>

							</td>
						</tr>
					{/each}
				{:else }
					<tr>
						<td colspan="{headers.length + 2}" class="text-center">
							<p class="p-5">Data Tidak Di temukan</p>
						</td>
					</tr>
				{/if}

				</tbody>
			</table>
		</div>

	</div>
</div>






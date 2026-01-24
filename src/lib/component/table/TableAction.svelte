<script>
    import {formatCurrencyIDR} from "$lib/helpers/parse_number_format_currency.ts";
    import {FormatDate} from "$lib/helpers/formated_date.ts";

    let {
        data = [
            {name: '', age: 0},
        ],

        headers = [
            {name: 'Name', value: 'name'},
            {name: 'Age', value: 'age'}
        ]

    } = $props()

    let lengthHeader = headers.length

</script>
<div class="m-3 w-full">
	<div class="table-auto md:table-fixed  bg-white h-max p-3 rounded-lg">

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
				{#if data.length < 1 || data === null}
					<tr class="odd:bg-neutral-primary  border-b border-default text-center items-center">
						<td colspan={lengthHeader+2} class="px-6 py-4">
							Data Tidak Ada
						</td>
					</tr>
				{/if}

				{#each data as item, index}
					<tr class="odd:bg-neutral-primary  border-b border-default">
						<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
							{index + 1}
						</th>

						{#each headers as header}
							<th scope="row" class="px-6 py-4 font-medium text-heading whitespace-nowrap">
								{#if header['type_data'] == "parseIDR"}
									{formatCurrencyIDR(item[header.value])}
									<!--											hello-->
								{:else if header['type_data'] == "date"}
									{FormatDate(item[header.value])}
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
									class="text-white bg-warning box-border border border-transparent hover:bg-warning-strong focus:ring-4 focus:ring-warning-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
								<img src="/icon/pen-field.svg" class="h-6 w-6 p-1 justify-center" alt="Tauri Logo"/>
							</button>

							<button type="button"
									class="text-white bg-danger box-border border border-transparent hover:bg-danger-strong focus:ring-4 focus:ring-danger-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
								<img src="/icon/trash.svg" class="h-6 w-6 p-1 justify-center" alt="Tauri Logo"/>
							</button>

						</td>
					</tr>
				{/each}

				</tbody>
			</table>
		</div>

	</div>
</div>






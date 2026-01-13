<script>
    import {createEventDispatcher} from 'svelte';
    import {FormatDate} from '$lib/helpers/formated_date.ts';
    import {FormatCurrency} from '$lib/helpers/parse_number_format_currency.ts';

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

    function handleDetail(id) {
        dispatch('detail', {id: id})
    }

    function handleRemove(id) {
        dispatch('remove', {id: id})
    }

    function handleUpdate(id) {
        dispatch('update', {id: id, action: true})
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
										{#if header['type_data'] == "parse-number"}
											{FormatCurrency(item[header.value])}
										{:else if header['type_data'] == "date"}
											{FormatDate(item[header.value])}
										{:else}
											{item[header['value']]}
										{/if}
									{/if}
								</th>
							{/each}

							<td class="px-6 py-4">
								<button
										onclick="{(e) => handleDetail(item.id)}"
										type="button"
										class="text-white  hover:bg-gray-500 px-3 py-2.5 rounded-base shadow-xl">
									<svg
											class="h-6 w-6 text-black hover:text-white"
											fill="currentColor"
											xmlns="http://www.w3.org/2000/svg" id="Outline" viewBox="0 0 24 24" width="512" height="512">
										<path d="M23.271,9.419C21.72,6.893,18.192,2.655,12,2.655S2.28,6.893.729,9.419a4.908,4.908,0,0,0,0,5.162C2.28,17.107,5.808,21.345,12,21.345s9.72-4.238,11.271-6.764A4.908,4.908,0,0,0,23.271,9.419Zm-1.705,4.115C20.234,15.7,17.219,19.345,12,19.345S3.766,15.7,2.434,13.534a2.918,2.918,0,0,1,0-3.068C3.766,8.3,6.781,4.655,12,4.655s8.234,3.641,9.566,5.811A2.918,2.918,0,0,1,21.566,13.534Z"/>
										<path d="M12,7a5,5,0,1,0,5,5A5.006,5.006,0,0,0,12,7Zm0,8a3,3,0,1,1,3-3A3,3,0,0,1,12,15Z"/>
									</svg>
								</button>

								<button
										onclick="{(e) => handleUpdate(item.id)}"
										type="button"
										class="text-white  hover:bg-gray-500 px-3 py-2.5 rounded-base shadow-xl">

									<svg
											xmlns="http://www.w3.org/2000/svg"
											id="Layer_1"
											data-name="Layer 1"
											class="h-6 w-6 text-black hover:text-white"
											fill="currentColor"
											viewBox="0 0 24 24">
										<path d="m24,11v12H0v-12c0-1.654,1.346-3,3-3h7v2H3c-.552,0-1,.449-1,1v10h20v-10c0-.53-.418-.956-.941-.988l1.541-1.541c.839.532,1.401,1.464,1.401,2.529ZM5,15.5c0,.828.672,1.5,1.5,1.5s1.5-.672,1.5-1.5-.672-1.5-1.5-1.5-1.5.672-1.5,1.5Zm6.5,1.5c.828,0,1.5-.672,1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5,1.5.672,1.5,1.5,1.5Zm4.742-5h-4.242v-4.243L18.879.879c1.17-1.17,3.072-1.17,4.242,0,.566.566.879,1.32.879,2.121s-.313,1.555-.879,2.122l-6.879,6.878Zm-.828-2l6.293-6.293c.189-.189.293-.44.293-.707s-.104-.518-.293-.707c-.391-.391-1.023-.39-1.414,0l-6.293,6.292v1.415h1.414Z"/>
									</svg>
								</button>

								<button
										onclick="{(e) => handleRemove(item.id)}" type="button"
										class="text-white  hover:bg-gray-500 px-3 py-2.5 rounded-base shadow-xl">

									<svg
											class="h-6 w-6 text-black hover:text-white"
											fill="currentColor"
											xmlns="http://www.w3.org/2000/svg"
											viewBox="0 0 24 24" width="512" height="512">
										<g id="_01_align_center" data-name="01 align center">
											<path d="M22,4H17V2a2,2,0,0,0-2-2H9A2,2,0,0,0,7,2V4H2V6H4V21a3,3,0,0,0,3,3H17a3,3,0,0,0,3-3V6h2ZM9,2h6V4H9Zm9,19a1,1,0,0,1-1,1H7a1,1,0,0,1-1-1V6H18Z"/>
											<rect x="9" y="10" width="2" height="8"/>
											<rect x="13" y="10" width="2" height="8"/>
										</g>
									</svg>
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






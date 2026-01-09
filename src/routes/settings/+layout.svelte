<script>
    import {page} from '$app/stores';

    let subMenu = [
        // {label: 'Users', url: "/settings/users"},
        // {label: 'Pelanggan', url: $page.url.pathname + ""},
        {label: 'Users', url: "/settings/users"},
        {label: 'Pelanggan', url: "/settings/consumer"},
        {label: 'Supplier', url: "/settings/supplier"}
    ]
</script>

<div class="pt-2 mt-4 w-full mr-6">
	<div class="flex ml-3 flex-row">
		{#each subMenu as item}
			<a href="{item['url']}" class="p-2 mr-2 hover:bg-gray-200 rounded-lg min-w-30 text-center basis-auto"
			   class:bg-gray-200={$page.url.pathname === item['url']}
			   class:bg-white={$page.url.pathname != item['url']}>
				{item['label']}
			</a>
		{/each}
	</div>
	<slot/>
</div>
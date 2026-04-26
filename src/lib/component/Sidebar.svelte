<script>
    import { page } from "$app/stores";

    let hiddenDropDownSetting = $state(true);

    function toggleDropDownSetting() {
        hiddenDropDownSetting = !hiddenDropDownSetting;
    }

    let {
        data = [
            {
                name: "Dashboard",
                icon:
                    '<svg class="w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">\n' +
                    '            <path d="M128 128C128 110.3 113.7 96 96 96C78.3 96 64 110.3 64 128L64 464C64 508.2 99.8 544 144 544L544 544C561.7 544 576 529.7 576 512C576 494.3 561.7 480 544 480L144 480C135.2 480 128 472.8 128 464L128 128zM534.6 214.6C547.1 202.1 547.1 181.8 534.6 169.3C522.1 156.8 501.8 156.8 489.3 169.3L384 274.7L326.6 217.4C314.1 204.9 293.8 204.9 281.3 217.4L185.3 313.4C172.8 325.9 172.8 346.2 185.3 358.7C197.8 371.2 218.1 371.2 230.6 358.7L304 285.3L361.4 342.7C373.9 355.2 394.2 355.2 406.7 342.7L534.7 214.7z"/>\n' +
                    "        </svg>",
                link: "/",
            },
            {
                name: "Pembelian",
                icon: '<img src="/icon/shopping-cart-add.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>',
                link: "/purchase",
            },
            {
                name: "Penjualan",
                icon: '<img src="/icon/scanner-touchscreen.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>',
                link: "/sale",
            },
            {
                name: "Barang",
                icon: '<img src="/icon/list-items.png" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>',
                link: "/items",
            },
            {
                name: "Pengguna",
                icon: '<svg class="w-7 h-7 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z" clip-rule="evenodd"/></svg>',
                link: "/users",
            },
            {
                name: "Pelanggan",
                icon: '<svg class="w-7 h-7 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z" clip-rule="evenodd"/></svg>',
                link: "/pelanggan",
            },
            {
                name: "Supplier",
                icon: '<svg class="w-7 h-7 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Zm0-15a3 3 0 1 0 0 6 3 3 0 0 0 0-6Zm0 8.5c-2.3 0-4.3 1-5.7 2.6A7 7 0 0 0 12 19a7 7 0 0 0 5.7-1.9 8.1 8.1 0 0 0-5.7-2.6Z" clip-rule="evenodd"/></svg>',
                link: "/supplier",
            },
        ],
    } = $props();
</script>

<aside
    class="bg-white w-65 m-2 rounded-sm p-5 inset-shadow-sm h-[90vh] overflow-y-auto"
>
    {#each data as menu}
        <a
            href={menu["link"]}
            class="flex items-center hover:bg-gray-200 rounded-xl p-2 mb-2 transition-colors"
            class:bg-gray-200={menu["link"] === $page.url.pathname}
        >
            <div class="flex items-center justify-center w-10 h-10 shrink-0">
                {@html menu["icon"]}
            </div>
            <p class="ml-3 font-medium text-gray-700">{menu["name"]}</p>
        </a>
    {/each}
</aside>

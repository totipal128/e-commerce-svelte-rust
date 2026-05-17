<script>
    import { page } from "$app/stores";
    import { authStore } from "$lib/store/auth.js";
    import { roleMenusStore } from "$lib/store/role_menus.js";
    import { onMount } from "svelte";

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
                name: "Satuan Barang",
                icon: '<svg class="w-7 h-7 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3"/></svg>',
                link: "/unit-barang",
            },
            {
                name: "Kategori Barang",
                icon: '<svg class="w-7 h-7 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/></svg>',
                link: "/jenis-barang",
            },
            {
                name: "Pengguna",
                icon: '<svg class="w-7 h-7 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z" clip-rule="evenodd"/></svg>',
                link: "/users",
            },
            {
                name: "Peran & Menu",
                icon: '<svg class="w-7 h-7 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/></svg>',
                link: "/roles",
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

    // Derived reactive access permissions
    let hasPenggunaAccess = $derived(
        ($roleMenusStore[$authStore.role || "kasir"] || []).includes("Pengguna")
    );
    let hasRolesAccess = $derived(
        ($roleMenusStore[$authStore.role || "kasir"] || []).includes("Peran & Menu")
    );
    let hasPengaturanAccess = $derived(hasPenggunaAccess || hasRolesAccess || $authStore.isLoggedIn);

    let isSettingActive = $derived($page.url.pathname === "/users" || $page.url.pathname === "/roles" || $page.url.pathname === "/profile");

    onMount(() => {
        if (isSettingActive) {
            hiddenDropDownSetting = false;
        }
    });

    // Filter out standard menus so that setting submenus only render in the dropdown
    let filteredMenu = $derived(
        data.filter(menu => {
            if (menu.name === "Pengguna" || menu.name === "Peran & Menu") {
                return false;
            }
            const role = $authStore.role || "kasir";
            const allowedMenus = $roleMenusStore[role] || [];
            return allowedMenus.includes(menu.name);
        })
    );
</script>

<aside
    class="bg-white w-65 m-2 rounded-sm p-5 inset-shadow-sm h-[90vh] overflow-y-auto"
>
    {#each filteredMenu as menu}
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

    {#if hasPengaturanAccess}
        <!-- Parent Collapsible Settings Item -->
        <button
            on:click={toggleDropDownSetting}
            class="flex items-center justify-between w-full hover:bg-gray-200 rounded-xl p-2 mb-2 transition-colors focus:outline-none"
            class:bg-gray-200={isSettingActive}
        >
            <div class="flex items-center">
                <div class="flex items-center justify-center w-10 h-10 shrink-0">
                    <svg class="w-6 h-6 text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.43l-1.003.828c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .252c-.008.379.137.751.43.992l1.004.827a1.125 1.125 0 0 1 .26 1.43l-1.297 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.43l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 0 1 0-.252c.007-.379-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128c.332-.183.582-.495.645-.869L9.594 3.94ZM12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
                    </svg>
                </div>
                <span class="ml-3 font-medium text-gray-700">Pengaturan</span>
            </div>
            <!-- Arrow icon with reactiveness -->
            <svg
                class="w-4 h-4 text-gray-500 transition-transform duration-200 shrink-0"
                class:rotate-180={!hiddenDropDownSetting}
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="3"
            >
                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
            </svg>
        </button>

        <!-- Collapsible Content Panel -->
        <div
            class="pl-4 overflow-hidden transition-all duration-300 ease-in-out"
            style="max-height: {hiddenDropDownSetting ? '0px' : '300px'}"
        >
            <a
                href="/profile"
                class="flex items-center hover:bg-gray-100 rounded-xl p-2 mb-1 transition-colors"
                class:bg-gray-100={$page.url.pathname === "/profile"}
            >
                <div class="flex items-center justify-center w-8 h-8 shrink-0">
                    <svg class="w-5 h-5 text-gray-500" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                    </svg>
                </div>
                <span class="ml-3 text-sm font-medium text-gray-600">Profil Saya</span>
            </a>

            {#if hasPenggunaAccess}
                <a
                    href="/users"
                    class="flex items-center hover:bg-gray-100 rounded-xl p-2 mb-1 transition-colors"
                    class:bg-gray-100={$page.url.pathname === "/users"}
                >
                    <div class="flex items-center justify-center w-8 h-8 shrink-0">
                        <svg class="w-5 h-5 text-gray-500" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24">
                            <path fill-rule="evenodd" d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z" clip-rule="evenodd"/>
                        </svg>
                    </div>
                    <span class="ml-3 text-sm font-medium text-gray-600">Pengguna</span>
                </a>
            {/if}

            {#if hasRolesAccess}
                <a
                    href="/roles"
                    class="flex items-center hover:bg-gray-100 rounded-xl p-2 mb-1 transition-colors"
                    class:bg-gray-100={$page.url.pathname === "/roles"}
                >
                    <div class="flex items-center justify-center w-8 h-8 shrink-0">
                        <svg class="w-5 h-5 text-gray-500" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                        </svg>
                    </div>
                    <span class="ml-3 text-sm font-medium text-gray-600">Peran & Menu</span>
                </a>
            {/if}
        </div>
    {/if}
</aside>

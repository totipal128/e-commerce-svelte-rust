<script>
    import {page} from "$app/stores";

    let hiddenDropDownSetting = $state(true);

    function toggleDropDownSetting() {
        hiddenDropDownSetting = !hiddenDropDownSetting;
    }

    let {
        data = [
            {
                name: 'Dashboard',
                icon: '<svg class="w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">\n' +
                    '            <path d="M128 128C128 110.3 113.7 96 96 96C78.3 96 64 110.3 64 128L64 464C64 508.2 99.8 544 144 544L544 544C561.7 544 576 529.7 576 512C576 494.3 561.7 480 544 480L144 480C135.2 480 128 472.8 128 464L128 128zM534.6 214.6C547.1 202.1 547.1 181.8 534.6 169.3C522.1 156.8 501.8 156.8 489.3 169.3L384 274.7L326.6 217.4C314.1 204.9 293.8 204.9 281.3 217.4L185.3 313.4C172.8 325.9 172.8 346.2 185.3 358.7C197.8 371.2 218.1 371.2 230.6 358.7L304 285.3L361.4 342.7C373.9 355.2 394.2 355.2 406.7 342.7L534.7 214.7z"/>\n' +
                    '        </svg>',
                link: '/'
            }, {
                name: 'Pembelian',
                icon: '<img src="/icon/shopping-cart-add.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>',
                link: '/purchase'
            }, {
                name: 'Penjualan',
                icon: '<img src="/icon/scanner-touchscreen.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>',
                link: '/sale'
            }, {
                name: 'Pengaturan',
                icon: '<img src="/icon/gears.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>',
                link: '/settings'
            }
        ]
    } = $props()

</script>
<aside class="bg-white w-65 m-2 rounded-2xl p-5 inset-shadow-sm h-[89vh]">
    {#each data as menu}
        <a href="{menu['link']}" class="flex hover:bg-gray-200 rounded-xl p-1 m-1"
           class:bg-gray-200={menu['link'] === $page.url.pathname}>
            {@html menu['icon']}
            <p class="pt-2 pl-3"> {menu['name']} </p>
        </a>
    {/each}

    <!--    <button onclick="{toggleDropDownSetting}" class="flex hover:bg-blue-700 rounded-xl p-1 m-1">-->
    <!--        <img src="/icon/gears.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>-->
    <!--        <p class="pt-2 pl-3"> Setting </p>-->

    <!--        <img src="/icon/chevron-double-down.svg"-->
    <!--             class="h-7 w-7 ml-2 pt-3 justify-center transition duration-300"-->
    <!--             alt="Tauri Logo"-->
    <!--             class:rotate-180={hiddenDropDownSetting} class:mt-3={hiddenDropDownSetting}/>-->
    <!--&lt;!&ndash;    </button>&ndash;&gt;-->
    <!--    &lt;!&ndash;    <div class="duration-300 ease-in" class:hidden="{hiddenDropDownSetting}">&ndash;&gt;-->
    <!--        <hr/>-->
    <!--        <a href="/settings/users" class="flex hover:bg-blue-700 rounded-xl pl-2.5 p-1 m-1">-->
    <!--            <img src="/icon/users-gear.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>-->
    <!--            <p class="pt-2 pl-3"> users </p>-->
    <!--        </a>-->

    <!--    </div>-->

</aside>
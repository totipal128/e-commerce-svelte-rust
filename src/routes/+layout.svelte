<script>
    import "../app.css";
    import "../app.js"
    import Sidebar from "$lib/component/Sidebar.svelte"

    let hiddenDropDownSetting = $state(true);

    function toggleDropDownSetting() {
        hiddenDropDownSetting = !hiddenDropDownSetting;
    }
</script>


<div class="text-gray-800 h-[90vh] w-full ">
    <nav class="bg-white h-15 shadow-md">
        <div class="flex flex-row">
            <div class="basis-1/3">
                icon
            </div>
            <div class="basis-2/3">
                <div class="flex flex-row-reverse">
                    <p class="pr-5 pt-5 p-3">Totipal etc</p>
                    <div class="bg-white h-12 w-12 mt-1 rounded-full">
                        <img src="/tauri.svg" class="h-10 w-10 p-1 justify-center" alt="Tauri Logo"/>
                    </div>
                </div>
            </div>
        </div>
    </nav>
    <div class="flex bg-gray-300">
        <Sidebar/>

        <slot/>

    </div>
    <footer class="bg-gray-500 h-10">
        footer
    </footer>
</div>

<script>
    import "../app.css";
    import "../app.js"
    import Sidebar from "$lib/component/Sidebar.svelte"

    let hiddenDropDownSetting = $state(true);

    function toggleDropDownSetting() {
        hiddenDropDownSetting = !hiddenDropDownSetting;
    }
</script>


<div class="text-gray-800 h-full w-full overflow-y-auto">
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
	<div class="flex bg-gray-300 overflow-y-auto" style="height: 93.4vh">
		<Sidebar/>

		<slot/>

	</div>
	<footer class="absolute  bottom-0 bg-gray-500 h-10 w-full">
		footer
	</footer>
</div>
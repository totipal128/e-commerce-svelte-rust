<script>
    import {createEventDispatcher, onMount, tick} from 'svelte';
    import {invoke} from "@tauri-apps/api/core";

    const dispatch = createEventDispatcher();
    let relationItems = [];
    let loading = $state(false);
    let {
        id = 0,
        title = "",
    } = $props()

    async function remove_data() {
        loading = true
        try {
            const result = await invoke("sale_delete", {
                id: id
            })

        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }

    }

    // onMount(detail_data)

    function parseNumber(v) {
        if (!v) return 0;
        return Number(v.toString().replace(/\./g, ""));
    };

    function handlerRemove() {
        remove_data()
        dispatch("close", "remove");
    }

    function handlerClose() {
        dispatch("close");
    }
</script>

<div class="absolute bg-black w-screen h-screen z-11 top-0" style="opacity: 0.95">
</div>
<div class="absolute inset-0 flex items-center justify-center z-[11] " on:click={handlerClose}>
	<div class=" bg-white min-w-150  rounded-2xl" on:click|stopPropagation>
		<div class="relative flex items-center justify-center py-6 ">
			<div class="relative flex flex-col items-center justify-center text-center ">
				<svg
						class="text-red-800"
						fill="currentColor"
						xmlns="http://www.w3.org/2000/svg"
						id="Layer_1"
						data-name="Layer 1"
						viewBox="0 0 24 24"
						width="100"
						height="100"
				>
					<path d="M20.1,20.182a1.5,1.5,0,0,1-1.156-2.455,9.05,9.05,0,0,0,0-11.454,1.5,1.5,0,0,1,2.313-1.91,12.065,12.065,0,0,1,0,15.274A1.5,1.5,0,0,1,20.1,20.182ZM4.855,19.838a1.5,1.5,0,0,0,.2-2.111,9.049,9.049,0,0,1,0-11.454,1.5,1.5,0,0,0-2.313-1.91,12.065,12.065,0,0,0,0,15.274,1.5,1.5,0,0,0,2.111.2ZM19,12c-.3,9.251-13.707,9.249-14,0C5.294,2.749,18.707,2.751,19,12Zm-5.5,3.5a1.5,1.5,0,0,0-3,0A1.5,1.5,0,0,0,13.5,15.5Zm0-7a1.5,1.5,0,0,0-3,0v3a1.5,1.5,0,0,0,3,0Z"/>
				</svg>
				<span class="text-3xl font-semibold content-center">Hapus Data</span>
			</div>


			<button
					on:click={handlerClose}
					class="absolute right-4 top-4 hover:bg-gray-200 p-3 rounded-full"
			>
				X
			</button>
		</div>

		<!-- CONTENT -->
		<div class="p-3 text-2xl">
			<p class="p-5 text-gray-500">
				Apakah Anda Yakin Ingin Menghapus Transaksi {title} ?
			</p>
		</div>

		<!-- FOOTER -->
		<div class=" p-4 flex justify-center gap-3">
			<button
					on:click={handlerRemove}
					class="px-4 py-2 rounded-lg bg-red-500 hover:bg-red-900 hover:text-white"
			>
				Hapus
			</button>
			<button
					on:click={handlerClose}
					class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300"
			>
				Batal
			</button>
		</div>
	</div>
</div>
<script>
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    let {
        open = false,
        id = 0
    } = $props();

    let loading = $state(true)

    let dataUsers = $state({
        username: null,
        email: null,
        password: null,
        name: null,
        address: null,
        no_handphone: null,

    })
    let data = $state([])

    async function saveData(data) {
        loading = true
        try {
            const result = await invoke("create_users", {
                data: data
            })

            data = result.results

        } catch (err) {
            console.log("err", err)
        } finally {
            closeModal()
            loading = false;
        }

    }

    //
    // onMount(fetchData)

    function closeModal(confirm) {
        let c = false
        if (typeof (confirm) !== "boolean") {
            c = true
        }

        dispatch('close', {confirm: c});
    }

    function save(e) {
        saveData(dataUsers)
    }
</script>

{#if open}
	<!-- Overlay -->
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black" style="opacity: 0.9" on:click={closeModal}>
		<!-- Modal -->
		<div class="relative  bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6 w-300" on:click|stopPropagation>
			<!-- Close button -->
			<button on:click|stopPropagation={closeModal} class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<!-- Content -->
			<div class="relative h-full">
				<!--				<svg class="mx-auto mb-4 text-fg-disabled w-12 h-12" viewBox="0 0 24 24" fill="none">-->
				<!--					<path stroke="currentColor" stroke-width="2"-->
				<!--						  d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0Z"/>-->

				<!--				</svg>-->

				<div class="mx-auto mb-4 text-fg text-2xl text-center w-50 h-12">
					Tambah Data

					<div class="h-[2px] bg-gray-400 my-2"></div>

				</div>


				<form class="relative overflow-y-auto" on:submit|preventDefault={save}>
					<div class="max-w-sm mx-auto space-y-4">
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Username</label>
							<input bind:value={dataUsers.username} type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""
								   required/>
						</div>
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Email</label>
							<input bind:value={dataUsers.email} type="email" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""
								   required/>
						</div>
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Nama</label>
							<input bind:value={dataUsers.name} type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""
								   required/>
						</div>
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Password</label>
							<input bind:value={dataUsers.password} type="password" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
								   placeholder=""
								   required/>
						</div>
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Alamat</label>
							<input bind:value={dataUsers.address} type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""
								   required/>
						</div>
						<div>
							<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">No. Telp</label>
							<input bind:value={dataUsers.no_handphone} type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
								   placeholder=""
								   required/>
						</div>

					</div>


					<div class="flex gap-4 justify-center mt-5">

						<button
								class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
							Simpan
						</button>

						<button
								on:click|stopPropagation={closeModal}
								class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base">
							Batal
						</button>
					</div>


				</form>


			</div>
		</div>
	</div>
{/if}

<script>
    import {createEventDispatcher} from 'svelte';
    import {invoke} from "@tauri-apps/api/core";
    import {showToast} from "$lib/store/toast.js";

    const dispatch = createEventDispatcher();

    let {
        open = false,
        id = 0
    } = $props();

    let loading = $state(false)
    
    let supplierData = $state({
        name: "",
        email: "",
        address: "",
        no_hp: "",
    })

    async function saveData() {
        loading = true
        try {
            const result = await invoke("supplier_create", {
                data: supplierData
            })
            showToast("Berhasil menyimpan data pemasok", "success");
            closeModal()
        } catch (err) {
            console.log("err", err)
            showToast("Gagal menyimpan data: " + err, "error");
        } finally {
            loading = false;
        }
    }

    function closeModal(confirm) {
        let c = false
        if (typeof (confirm) !== "boolean") {
            c = true
        }
        dispatch('close', {confirm: c});
    }

    function save(e) {
        saveData()
    }
</script>

{#if open}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black" style="opacity: 0.9" on:click={closeModal}>
		<div class="relative bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6 w-96 md:w-[500px]" on:click|stopPropagation>
			<button on:click={closeModal} class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<div class="relative h-full">
				<div class="mx-auto mb-4 text-fg text-2xl text-center w-full h-12">
					Tambah Pemasok (Supplier)
					<div class="h-[2px] bg-gray-400 my-2"></div>
				</div>

				<form class="relative overflow-y-auto" on:submit|preventDefault={save}>
					<div class="space-y-4">
						<div>
							<label class="block mb-2.5 text-sm font-medium text-heading">Nama Lengkap</label>
							<input bind:value={supplierData.name} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="Masukkan nama..." required/>
						</div>
						<div>
							<label class="block mb-2.5 text-sm font-medium text-heading">Ponsel / No HP</label>
							<input bind:value={supplierData.no_hp} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="0812xxxx" required/>
						</div>
                        <div>
							<label class="block mb-2.5 text-sm font-medium text-heading">Email</label>
							<input bind:value={supplierData.email} type="email" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="nama@email.com"/>
						</div>
                        <div>
							<label class="block mb-2.5 text-sm font-medium text-heading">Alamat</label>
							<textarea bind:value={supplierData.address} rows="3" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="Alamat pemasok..."></textarea>
						</div>
					</div>

					<div class="flex gap-4 justify-center mt-8">
						<button disabled={loading} class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
							Simpan
						</button>
						<button on:click={closeModal} type="button" class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base">
							Batal
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}

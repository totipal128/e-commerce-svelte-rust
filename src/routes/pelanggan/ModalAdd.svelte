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
    
    let consumerData = $state({
        name: "",
        email: "",
        address: "",
        no_hp: "",
        username: "",
        password: ""
    })

    async function saveData() {
        loading = true
        try {
            const result = await invoke("consumer_create", {
                data: consumerData
            })
            showToast("Berhasil menyimpan data pelanggan", "success");
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
		<div class="relative bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6 w-96 md:w-[600px] max-h-[90vh] overflow-y-auto" on:click|stopPropagation>
			<button on:click={closeModal} class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<div class="relative h-full">
				<div class="mx-auto mb-4 text-fg text-lg md:text-2xl text-center w-full min-h-12">
					Tambah Pelanggan Baru
					<div class="h-[2px] bg-gray-400 my-2"></div>
				</div>

				<form class="relative mt-5" on:submit|preventDefault={save}>
                    <p class="text-sm text-gray-500 mb-3 italic">Data Kredensial Akun (Opsional, untuk login pelanggan)</p>
					<div class="grid grid-cols-2 gap-4 place-items-stretch mb-4">
						<div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Username</label>
							<input bind:value={consumerData.username} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="Opsional..."/>
						</div>
                        <div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Password</label>
							<input bind:value={consumerData.password} type="password" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"/>
						</div>
                    </div>

                    <p class="text-sm text-gray-500 mb-3 italic">Data Informasi Asli Pelanggan</p>
					<div class="grid grid-cols-2 gap-4 place-items-stretch">
						<div class="col-span-2">
							<label class="block mb-2 text-sm font-medium text-heading">Nama Lengkap</label>
							<input bind:value={consumerData.name} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="Masukkan nama..." required/>
						</div>
						<div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Ponsel / No HP</label>
							<input bind:value={consumerData.no_hp} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="0812xxxx" required/>
						</div>
                        <div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Email</label>
							<input bind:value={consumerData.email} type="email" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="nama@email.com"/>
						</div>
                        <div class="col-span-2">
							<label class="block mb-2 text-sm font-medium text-heading">Alamat</label>
							<textarea bind:value={consumerData.address} rows="2" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder="Alamat pelanggan..."></textarea>
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

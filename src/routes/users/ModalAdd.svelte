<script>
    import {createEventDispatcher} from 'svelte';
    import {invoke} from "@tauri-apps/api/core";
    import {showToast} from "$lib/store/toast.js";

    import { rolesStore } from "$lib/store/role_menus.js";

    const dispatch = createEventDispatcher();

    let {
        open = false
    } = $props();

    let loading = $state(false)
    
    let usersData = $state({
        username: "",
        password: "",
        name: "",
        email: "",
        address: "",
        no_handphone: "",
        barcode: "",
        role: "kasir",
    })

    async function saveData() {
        loading = true
        try {
            const result = await invoke("create_users", {
                data: usersData
            })
            showToast("Berhasil meregistrasi pengguna baru", "success");
            closeModal(true)
        } catch (err) {
            console.log("err", err)
            showToast("Gagal menyimpan pengguna: " + err, "error");
        } finally {
            loading = false;
        }
    }

    /**
     * @param {any} confirm
     */
    function closeModal(confirm) {
        let c = false
        if (typeof (confirm) !== "boolean") {
            c = true
        }
        dispatch('close', {confirm: c});
    }

    /**
     * @param {any} e
     */
    function save(e) {
        saveData()
    }
</script>

{#if open}
	<div class="fixed inset-0 z-[100] flex items-center justify-center bg-black" style="opacity: 0.9" on:click={closeModal}>
		<div class="relative bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6 w-96 md:w-[600px] overflow-y-auto max-h-[90vh]" on:click|stopPropagation>
			<button on:click={closeModal} class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<div class="relative w-full">
				<div class="mx-auto mb-4 text-fg text-2xl text-center w-full h-12">
					Tambah Pengguna Baru
					<div class="h-[2px] bg-gray-400 my-2"></div>
				</div>

				<form class="relative overflow-y-auto mt-5" on:submit|preventDefault={save}>
					<div class="grid grid-cols-2 gap-4 place-items-stretch">
						<div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Username</label>
							<input bind:value={usersData.username} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" placeholder="username_login" required/>
						</div>
                        <div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Password</label>
							<input bind:value={usersData.password} type="password" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" required/>
						</div>
						<div class="col-span-2">
							<label class="block mb-2 text-sm font-medium text-heading">Nama Lengkap</label>
							<input bind:value={usersData.name} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" placeholder="Nama asli" required/>
						</div>
                        <div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Email</label>
							<input bind:value={usersData.email} type="email" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" placeholder="pengguna@email.com"/>
						</div>
                        <div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">No Handphone</label>
							<input bind:value={usersData.no_handphone} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" placeholder="0812xxxx"/>
						</div>
                         <div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Barcode (Bila ada)</label>
							<input bind:value={usersData.barcode} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" placeholder="Brcode karyawan..."/>
						</div>
                         <div class="col-span-2 md:col-span-1">
							<label class="block mb-2 text-sm font-medium text-heading">Peran Kerja (Role)</label>
							<select bind:value={usersData.role} class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3.5 py-3 shadow-xs font-semibold focus:outline-none">
								{#each $rolesStore as role}
									<option value={role.code}>{role.name} ({role.code})</option>
								{/each}
							</select>
						</div>
                        <div class="col-span-2">
							<label class="block mb-2 text-sm font-medium text-heading">Alamat</label>
							<textarea bind:value={usersData.address} rows="2" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" placeholder="Alamat..."></textarea>
						</div>
					</div>

					<div class="flex gap-4 justify-center mt-8">
						<button disabled={loading} class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none w-24">
							Simpan
						</button>
						<button on:click={closeModal} type="button" class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base w-24">
							Batal
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}

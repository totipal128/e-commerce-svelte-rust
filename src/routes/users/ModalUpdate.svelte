<script>
    import {createEventDispatcher} from 'svelte';
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import {showToast} from "$lib/store/toast.js";

    const dispatch = createEventDispatcher();

    let {
        open = false,
        id = 0
    } = $props();

    let loading = $state(true)
    let usersData = $state({
        id: 0,
        username: "",
        password: "", // User can reset password, but normally backend skips empty.
        name: "",
        email: "",
        address: "",
        no_handphone: "",
        barcode: ""
    })

    async function fetchData() {
        loading = true
        try {
            const result = await invoke("get_detail_users_by_id", {
                id: id
            })
            if (result.data) {
                usersData = { ...result.data, password: "" } 
                // Set explicitly empty password so it won't overwrite unless typed
            }
        } catch (err) {
            console.log("err", err)
            showToast("Gagal mengambil data pengguna: " + err, "error");
        } finally {
            loading = false;
        }
    }

    async function updateData() {
        loading = true
        try {
            const dataToUpdate = { ...usersData };
            // Optional: logic to remove password field if empty depending on how rust handles struct decoding
            await invoke("update_users", {
                data: dataToUpdate
            })
            showToast("Berhasil memperbarui data pengguna", "success");
            closeModal()
        } catch (err) {
            console.log("err", err)
            showToast("Gagal memperbarui pengguna: " + err, "error");
        } finally {
            loading = false;
        }
    }

    onMount(fetchData)

    function closeModal(confirm) {
        let c = false
        if (typeof (confirm) !== "boolean") {
            c = true
        }
        dispatch('close', {confirm: c});
    }

    function save(e) {
        updateData()
    }
</script>

{#if open}
	<div class="fixed inset-0 z-[100] flex items-center justify-center bg-black" style="opacity: 0.9" on:click={closeModal}>
		<div class="relative bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6 w-96 md:w-[600px] overflow-y-auto max-h-[90vh]" on:click|stopPropagation>
			<button on:click={closeModal} class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<div class="relative h-full w-full">
				<div class="mx-auto mb-4 text-fg text-2xl text-center w-full h-12">
					Ubah Data Pengguna
					<div class="h-[2px] bg-gray-400 my-2"></div>
				</div>

                {#if loading && !usersData.username}
                    <div class="flex justify-center p-5">
                       <img src="/icon/gift/loading.gif" class="h-10 w-10" alt="Loading"/>
                    </div>
                {:else}
                    <form class="relative overflow-y-auto mt-5" on:submit|preventDefault={save}>
                        <div class="grid grid-cols-2 gap-4 place-items-stretch">
                            <div class="col-span-2 md:col-span-1">
                                <label class="block mb-2 text-sm font-medium text-heading">Username</label>
                                <input bind:value={usersData.username} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" required/>
                            </div>
                            <div class="col-span-2 md:col-span-1">
                                <label class="block mb-2 text-sm font-medium text-heading">Password (Baru)</label>
                                <input bind:value={usersData.password} type="password" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" placeholder="Kosongkan jika tidak diubah"/>
                            </div>
                            <div class="col-span-2">
                                <label class="block mb-2 text-sm font-medium text-heading">Nama Lengkap</label>
                                <input bind:value={usersData.name} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs" required/>
                            </div>
                            <div class="col-span-2 md:col-span-1">
                                <label class="block mb-2 text-sm font-medium text-heading">Email</label>
                                <input bind:value={usersData.email} type="email" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs"/>
                            </div>
                            <div class="col-span-2 md:col-span-1">
                                <label class="block mb-2 text-sm font-medium text-heading">No Handphone</label>
                                <input bind:value={usersData.no_handphone} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs"/>
                            </div>
                            <div class="col-span-2 md:col-span-1">
                                <label class="block mb-2 text-sm font-medium text-heading">Barcode</label>
                                <input bind:value={usersData.barcode} type="text" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs"/>
                            </div>
                            <div class="col-span-2">
                                <label class="block mb-2 text-sm font-medium text-heading">Alamat</label>
                                <textarea bind:value={usersData.address} rows="2" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs"></textarea>
                            </div>
                        </div>

                        <div class="flex gap-4 justify-center mt-8">
                            <button disabled={loading} class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none w-24">
                                Ubah Data
                            </button>
                            <button on:click={closeModal} type="button" class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base w-24">
                                Batal
                            </button>
                        </div>
                    </form>
                {/if}
			</div>
		</div>
	</div>
{/if}

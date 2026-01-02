<script>
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    let {
        open = false,
        id = 0
    } = $props();

    let detailData = $state(null)

    let loading = $state(true)
    let dataUsers = $state({
        username: null,
        email: null,
        password: null,
        name: null,
        address: null,
        no_handphone: null,

    })


    async function dataDetail() {
        loading = true
        try {
            const result = await invoke("get_detail_users_by_id", {
                id: id
            })

            dataUsers = result.data

        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }

    }

    //
    onMount(dataDetail)

    function closeModal() {
        dispatch('close');
    }

</script>

{#if open}
	<!-- Overlay -->
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black" style="opacity: 0.9" on:click={closeModal}>
		<!-- Modal -->
		<div class="relative  bg-neutral-primary-soft border border-default rounded-base shadow-sm p-4 md:p-6">
			<!-- Close button -->
			<button on:click={closeModal} class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center">
				✕
			</button>

			<!-- Content -->
			<div class="relative h-full">
				<!--				<svg class="mx-auto mb-4 text-fg-disabled w-12 h-12" viewBox="0 0 24 24" fill="none">-->
				<!--					<path stroke="currentColor" stroke-width="2"-->
				<!--						  d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0Z"/>-->

				<!--				</svg>-->

				<div class="mx-auto mb-4 text-fg text-2xl text-center w-50 h-12">
					Detail Data

					<div class="h-[2px] bg-gray-400 my-2"></div>

				</div>


			</div>

			<table class="p-3">
				<tbody>
				<tr class="p-2">
					<td class="p-3">Nama</td>
					<td>:</td>
					<td class="p-3">{dataUsers.name}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Username</td>
					<td>:</td>
					<td class="p-3">{dataUsers.username}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Email</td>
					<td>:</td>
					<td class="p-3">{dataUsers.email}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">Alamat</td>
					<td>:</td>
					<td class="p-3">{dataUsers.address}</td>
				</tr>
				<tr class="p-2">
					<td class="p-3">No. Telp</td>
					<td>:</td>
					<td class="p-3">{dataUsers.no_handphone}</td>
				</tr>
				</tbody>
			</table>
		</div>
	</div>
{/if}

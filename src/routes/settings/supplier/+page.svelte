<script>
    import Table from "$lib/component/table/Table.svelte"
    import ModalCreate from "$lib/component/Modal/ModalCreate.svelte"
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    let loading = false

    let data = $state([])
    let headerTable = [
        {name: 'Nama', value: 'name'},
        {name: 'Email', value: 'email'},
        {name: 'Alamat', value: 'address'},
        {name: 'No Telpon', value: 'no_hp'},
        {name: 'Tanggal Buat', value: 'created_at', type_data: 'date'},
    ]

    async function fetchDataList() {
        loading = true
        try {
            let result = await invoke("supplier_list",
                {
                    filter: {
                        search: "",
                    }
                }
            )

            data = result.results
        } catch (e) {
            console.error("", e)
        } finally {
            loading = false
        }
    }


    onMount(fetchDataList)


    // 	ModalCreate Action
    let showModalCreate = $state(false)

    // Create
    let formatDataSave = null
    let formatFormSave = [
        {title: "Nama", value: "name", type: "text", is_required: true, placeholder: ""},
        {title: "Email", value: "email", type: "text", is_required: true, placeholder: ""},
        {title: "Alamat", value: "address", type: "textarea", is_required: true, placeholder: ""},
        {title: "No Telpon", value: "no_hp", type: "text", is_required: true, placeholder: ""},
    ]

    function handlerShowModalCreate() {
        showModalCreate = !showModalCreate;

        if (showModalCreate) {
            formatDataSave = {
                name: "",
                email: "",
                address: "",
                no_hp: "",
            }
        }
    }

    async function handlerSave(e) {
        let payload = e.detail;
        loading = true
        try {
            let result = await invoke("supplier_create",
                {
                    data: payload
                }
            )

            data = result.results
        } catch (e) {
            console.error("error", e)
        } finally {
            fetchDataList()
            loading = false
            showModalCreate = false;
        }
    }

</script>

{#if showModalCreate}
	<ModalCreate formatData={formatDataSave} loading formatFormData={formatFormSave} on:create={handlerSave} on:close={handlerShowModalCreate}></ModalCreate>
{/if}


<div class="w-full">
	{#if (loading)}
		<div class="flex w-full h-[80vh] mt-3 rounded-2xl bg-white justify-center items-center">
			<img src="/icon/gift/loading.gif" class="h-50 w-50 p-1 justify-center" alt="Tauri Logo"/>
		</div>
	{:else}
		<div class="flex justify-between">
			<div class="ml-5 mt-3 flex">
				<button onclick={handlerShowModalCreate} type="button" class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">Tambah Data</button>
			</div>

			<form class="mt-3 mr-5 w-100">
				<label for="search" class="block mb-2.5 text-sm font-medium text-heading sr-only ">Search</label>
				<div class="relative">
					<div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
						<svg class="w-4 h-4 text-body" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
							<path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"/>
						</svg>
					</div>
					<input
							type="search"
							id="search"
							class="block w-full p-3 ps-9 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
							placeholder="Search"
							required/>
					<button
							type="submit"
							class="absolute end-1.5 bottom-1.5 text-white bg-brand hover:bg-blue-950 box-border border border-transparent focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded text-xs px-3 py-1.5 focus:outline-none"
					>
						Search
					</button>
				</div>
			</form>
		</div>


		<Table headers={headerTable} data={data}/>
	{/if}
</div>
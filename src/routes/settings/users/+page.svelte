<script>
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import Table from "$lib/component/table/Table.svelte";
    import ModalAdd from './ModalAdd.svelte';
    import ModalUpdate from './ModalUpdate.svelte';
    import ModalRemove from './ModalRemove.svelte';
    import ModalDetail from './ModalDetail.svelte';
    import Modal from "./Modal.svelte"

    let getUsers = $state([]);
    let loading = $state(true);
    let searchDb = $state("")
    let searchDb_1 = $state("")

    let headerTable = [
        {name: 'Name', value: 'name'},
        {name: 'Email', value: 'email'},
        {name: 'No Telpon', value: 'no_handphone'},
        {name: 'Alamat', value: 'address'},
    ]

    function search(e) {
        // e.preventDefault();
        // console.log(e.preventDefault())
        if (e.key === "Enter" || e === "enter") {
            if (e === "enter") {
                searchDb = searchDb_1

                fetchUsersAll()
            } else {
                searchDb = e.target.value
                fetchUsersAll()
            }
        }
    }

    async function fetchUsersAll() {
        loading = true
        try {
            const result = await invoke("get_all_users", {
                filter: {
                    search: searchDb,
                }
            })

            getUsers = result.results
        } catch (err) {
            console.log(err)
        } finally {
            loading = false;
        }

    }

    onMount(fetchUsersAll)

    let idData = $state(0)
    let openModalAdd = $state(false)
    let openModalDetail = $state(false)
    let openModalUpdate = $state(false)
    let openModalRemove = $state(false)

    function handlerAdd() {
        openModalAdd = !openModalAdd
        fetchUsersAll()
    }

    function handlerDetail(e) {
        openModalDetail = !openModalDetail
        if (e.detail != null) {
            idData = e.detail.id
        }
        fetchUsersAll()
    }

    function handlerUpdate(e) {
        openModalUpdate = !openModalUpdate
        if (e.detail != null) {
            idData = e.detail.id
        }
        fetchUsersAll()
    }

    function handlerRemove(e) {
        openModalRemove = !openModalRemove
        if (e.detail != null) {
            idData = e.detail.id
        }
        fetchUsersAll()
    }

</script>


{#if openModalAdd}
	<ModalAdd open={openModalAdd} on:close={handlerAdd}></ModalAdd>
{/if}
{#if openModalDetail}
	<ModalDetail open={openModalDetail} id={idData} on:close={handlerDetail}></ModalDetail>
{/if}
{#if openModalUpdate}
	<ModalUpdate open={openModalUpdate} id={idData} on:close={handlerUpdate}></ModalUpdate>
{/if}
{#if openModalRemove}
	<ModalRemove openRemove={openModalRemove} id={idData} on:close={handlerRemove}></ModalRemove>
{/if}
<div class="w-full">
	{#if (loading)}
		<div class="flex w-full h-[80vh] mt-3 rounded-2xl bg-white justify-center items-center">
			<img src="/icon/gift/loading.gif" class="h-50 w-50 p-1 justify-center" alt="Tauri Logo"/>
		</div>
	{:else}
		<div class="flex justify-between">
			<div class="ml-5 mt-3 flex">
				<button onclick={handlerAdd} type="button" class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">Tambah Data</button>
			</div>

			<form class="mt-3 mr-5 w-100" onsubmit={(e)=>(search("enter"))}>
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
							bind:value={searchDb}
							class="block w-full p-3 ps-9 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
							placeholder="Search"
							onkeydown={(e)=>(search(e))}
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


		<Table headers={headerTable} data={getUsers} on:detail={handlerDetail} on:update={handlerUpdate} on:remove={handlerRemove}/>
	{/if}
</div>
<script>
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import {showToast} from "$lib/store/toast.js";
    import ModalRemove from "./ModalRemove.svelte"
    import ModalAdd from "./ModalAdd.svelte"
    import ModalUpdate from "./ModalUpdate.svelte"

    import Table from '$lib/component/table/Table.svelte'

    const headers = [
        {name: 'Username', value: 'username'},
        {name: 'Nama Lengkap', value: 'name'},
        {name: 'Email', value: 'email'},
        {name: 'HP', value: 'no_handphone'},
        {name: 'Kode Unix / Barcode', value: 'barcode'},
        {name: 'Peran / Role', value: 'role'}
    ]
    
    let loading = $state(true)
    /** @type {any[]} */
    let data = $state([])
    let searchDb = $state("")

    // Pagination state
    let currentPage = $state(1);
    let totalPages = $state(1);
    let totalItems = $state(0);
    let pageSize = $state(10);

    /**
     * @param {any} e
     */
    function search(e) {
        if (e.key === "Enter" || e === "enter") {
            if (e === "enter") {
                fetchData()
            } else {
                searchDb = e.target.value
                fetchData()
            }
        }
    }

    async function fetchData() {
        loading = true
        try {
            const result = await invoke("get_all_users", {
                filter: {
                    search: searchDb,
                    page: currentPage,
                    page_size: pageSize
                }
            })
            // Since get_all_users returns Pagination<UserNoPass>, the records are in result.results
            data = result.results || []
            totalPages = result.total_page;
            totalItems = result.count;
            currentPage = result.page;
        } catch (err) {
            console.log("err", err)
        } finally {
            loading = false;
        }
    }

    /**
     * @param {number} newPage
     */
    function handlePageChange(newPage) {
        currentPage = newPage;
        fetchData();
    }

    onMount(fetchData)

    let idData = $state(0)
    let openModalAdd = $state(false)
    let openModalUpdate = $state(false)
    let removeIsOpen = $state(false)

    /**
     * @param {any} e
     */
    function handlerRemove(e) {
        removeIsOpen = !removeIsOpen
        if (e.detail !== null) {
            idData = e.detail.id
            fetchData()
        }
    }

    /**
     * @param {any} e
     */
    function handlerAdd(e) {
        openModalAdd = !openModalAdd
        if (e.detail !== null) {
            idData = e.detail.id
            fetchData()
        }
    }

    /**
     * @param {any} e
     */
    function handlerUpdate(e) {
        openModalUpdate = !openModalUpdate
        if (e.detail !== null) {
            idData = e.detail.id
        } else {
            fetchData()
        }
    }

    async function handlerReseed() {
        if (!confirm("Apakah Anda yakin ingin memuat ulang database default users? Seluruh data password default ('admin123', 'kasir123', 'gudang123') dan relasi perannya akan disetel kembali secara default.")) {
            return;
        }
        loading = true;
        try {
            const res = await invoke("reseed_database_users");
            if (res.success) {
                showToast(res.message || "Berhasil memuat ulang default users", "success");
            } else {
                showToast(res.message || "Gagal memuat ulang data", "error");
            }
            await fetchData();
        } catch (err) {
            console.error(err);
            showToast("Terjadi kesalahan sistem saat memuat ulang data.", "error");
        } finally {
            loading = false;
        }
    }

</script>

{#if openModalAdd}
	<ModalAdd open={openModalAdd} on:close={handlerAdd}/>
{/if}
{#if openModalUpdate}
	<ModalUpdate id={idData} open={openModalUpdate} on:close={handlerUpdate}/>
{/if}
{#if removeIsOpen}
	<ModalRemove openRemove={removeIsOpen} id={idData} on:close={handlerRemove}/>
{/if}

<div class="w-full h-full">
	{#if (loading)}
		<div class="flex w-full h-[80vh] mt-3 rounded-2xl bg-white justify-center items-center">
			<img src="/icon/gift/loading.gif" class="h-50 w-50 p-1 justify-center" alt="Loading"/>
		</div>
	{:else}
		<div class="flex justify-between">
			<div class="ml-5 mt-3 flex gap-2">
				<button onclick={handlerAdd} type="button" class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">Tambah Pengguna</button>
				<button onclick={handlerReseed} type="button" class="text-emerald-700 bg-emerald-50 hover:bg-emerald-100 hover:text-emerald-800 border border-emerald-200 shadow-xs font-semibold leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none flex items-center gap-1.5 transition-all">
					<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 4v5h.582m15.356 2A8.001 8.001 0 1121.21 7.89L21 9m-9 5a3 3 0 110-6 3 3 0 010 6z" />
					</svg>
					Muat Ulang Default Users
				</button>
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
							placeholder="Pencarian nama/username..."
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

		<Table 
            headers={headers} 
            data={data} 
            page={currentPage}
            totalPages={totalPages}
            totalItems={totalItems}
            onPageChange={handlePageChange}
            on:remove={handlerRemove} 
            on:update={handlerUpdate} 
            on:detail={handlerUpdate} 
        />
	{/if}
</div>

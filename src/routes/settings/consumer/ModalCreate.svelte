<script>
    import ModalCreate from "$lib/component/Modal/ModalCreate.svelte"
</script>

<ModalCreate/>

<!--<script>-->
<!--    import {createEventDispatcher, onMount, tick} from 'svelte';-->
<!--    import {invoke} from "@tauri-apps/api/core";-->

<!--    const dispatch = createEventDispatcher();-->
<!--    let relationItems = [];-->
<!--    let loading = $state(false);-->
<!--    let get_detail_data = $state(-->
<!--        {-->
<!--            code: null,-->
<!--            customer_id: null,-->
<!--            PPN: 0,-->
<!--            discount: 0,-->
<!--            total_item: 0,-->
<!--            total: 0,-->
<!--            change: 0,-->
<!--            payment: null,-->
<!--            items: [-->
<!--                {-->
<!--                    code: null,-->
<!--                    items_id: null,-->
<!--                    items_name: null,-->
<!--                    items_unit: 0,-->
<!--                    items_price: 0,-->
<!--                    total: 0,-->
<!--                    qty: 0-->
<!--                }-->
<!--            ],-->
<!--        },-->
<!--    );-->
<!--    let {-->
<!--        id = 0,-->
<!--    } = $props()-->

<!--    async function detail_data() {-->
<!--        loading = true-->
<!--        try {-->
<!--            const result = await invoke("sale_by_id", {-->
<!--                id: id-->
<!--            })-->

<!--            get_detail_data = result.data;-->
<!--        } catch (err) {-->
<!--            console.log("err", err)-->
<!--        } finally {-->
<!--            loading = false;-->
<!--        }-->

<!--    }-->

<!--    onMount(detail_data)-->

<!--    function handlerClose() {-->
<!--        dispatch("close", false);-->
<!--    }-->
<!--</script>-->

<!--<div class="absolute bg-black w-screen h-screen z-11 left-0 top-0" style="opacity: 0.95">-->
<!--</div>-->
<!--<div class="absolute inset-0 flex items-center justify-center z-[11] " on:click={handlerClose}>-->
<!--	<div class=" bg-white min-w-200  rounded-2xl" on:click|stopPropagation>-->
<!--		<div class="relative flex items-center justify-center py-6 ">-->
<!--			<span class="text-3xl font-semibold">ModalCreate Data</span>-->

<!--			<button-->
<!--					on:click={handlerClose}-->
<!--					class="absolute right-4 top-4 hover:bg-gray-200 p-3 rounded-full"-->
<!--			>-->
<!--				X-->
<!--			</button>-->
<!--		</div>-->

<!--		&lt;!&ndash; CONTENT &ndash;&gt;-->
<!--		<div class="p-3 text-2xl">-->
<!--			<form class="relative overflow-y-auto">-->
<!--				<div class="max-w-sm mx-auto space-y-4">-->
<!--					<div>-->
<!--						<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Username</label>-->
<!--						<input type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""-->
<!--							   required/>-->
<!--					</div>-->
<!--					<div>-->
<!--						<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Email</label>-->
<!--						<input type="email" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""-->
<!--							   required/>-->
<!--					</div>-->
<!--					<div>-->
<!--						<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Nama</label>-->
<!--						<input type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""-->
<!--							   required/>-->
<!--					</div>-->
<!--					<div>-->
<!--						<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Password</label>-->
<!--						<input type="password" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"-->
<!--							   placeholder=""-->
<!--							   required/>-->
<!--					</div>-->
<!--					<div>-->
<!--						<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">Alamat</label>-->
<!--						<input type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body" placeholder=""-->
<!--							   required/>-->
<!--					</div>-->
<!--					<div>-->
<!--						<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">No. Telp</label>-->
<!--						<input type="text" id="visitors" class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"-->
<!--							   placeholder=""-->
<!--							   required/>-->
<!--					</div>-->

<!--				</div>-->


<!--				<div class="flex gap-4 justify-center mt-5">-->

<!--					<button-->
<!--							class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">-->
<!--						Simpan-->
<!--					</button>-->

<!--					<button-->
<!--							class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base">-->
<!--						Batal-->
<!--					</button>-->
<!--				</div>-->


<!--			</form>-->
<!--		</div>-->

<!--		&lt;!&ndash; FOOTER &ndash;&gt;-->
<!--		<div class=" p-4 flex justify-center gap-3">-->
<!--			<button-->
<!--					on:click={handlerClose}-->
<!--					class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300"-->
<!--			>-->
<!--				Close-->
<!--			</button>-->
<!--		</div>-->
<!--	</div>-->
<!--</div>-->
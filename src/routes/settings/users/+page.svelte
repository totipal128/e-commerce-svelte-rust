<script>
    import Table from "$lib/component/table/Table.svelte";
    import Modal from "./Modal.svelte"
    import ModalRemove from "./ModalRemove.svelte"

    let open = $state(false);
    let openRemove = $state(false);

    function openModal() {
        open = true;
    }

    function closeModal() {
        open = false;
        openRemove = false;
    }

    function openModalRemove(id) {
        console.log(id["detail"])
        openRemove = true;
    }

</script>


<Table on:remove={openModalRemove}/>
<ModalRemove {openRemove} on:close={closeModal}/>

{openRemove}
<!-- Button -->
<button
        on:click={openModal}
        class="text-white bg-brand hover:bg-brand-strong
         focus:ring-4 focus:ring-brand-medium
         shadow-xs font-medium rounded-base
         text-sm px-4 py-2.5">
    Toggle modal
</button>

<Modal {open} on:close={closeModal}/>
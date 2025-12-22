<script>
    import Table from "$lib/component/table/Table.svelte";
    import {page} from "$app/stores";
    import {goto} from "$app/navigation";

    goto($page.url.pathname + "/users")
</script>


<!--<Table/>
-->
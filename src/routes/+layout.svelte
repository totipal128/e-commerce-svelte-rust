<script>
    import "../app.css";
    import "../app.js"
    import Sidebar from "$lib/component/Sidebar.svelte"
    import Navbar from "$lib/component/Navbar.svelte"
	import ErrConnection from "$lib/component/err/ErrConnection.svelte";
	import Toast from "$lib/component/Toast.svelte";
    import Login from "$lib/component/Login.svelte";
    import { authStore } from "$lib/store/auth.js";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { afterNavigate } from "$app/navigation";

    let isDbConnected = $state(true);

    // Compute userDisplay name reactively and cast user session safely
    let userDisplay = $derived(
        $authStore.user 
            ? (($authStore.user).name || ($authStore.user).username || "Staf POS") 
            : "Staf POS"
    );

    async function checkDb() {
        try {
            isDbConnected = await invoke("check_db_connection");
        } catch (error) {
            console.error("DB Check error:", error);
            isDbConnected = false;
        }
    }

    onMount(() => {
        checkDb();
    });

    afterNavigate(() => {
        checkDb();
    });
</script>

<Toast />
{#if !isDbConnected}
    <ErrConnection on:check={checkDb} />
{:else if !$authStore.isLoggedIn}
    <Login />
{:else}
    <div class="text-gray-800 h-screen w-full flex flex-col overflow-hidden animate-fade-in">
        <Navbar user={userDisplay} />
        <div class="flex flex-1 overflow-hidden">
            <Sidebar/>
            <main class="flex-1 overflow-y-auto bg-gray-100">
                <slot/>
            </main>
        </div>
    </div>
{/if}
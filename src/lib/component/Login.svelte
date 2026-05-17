<script>
    import { invoke } from "@tauri-apps/api/core";
    import { loginUser } from "$lib/store/auth.js";
    import { showToast } from "$lib/store/toast.js";

    let username = $state("");
    let password = $state("");
    let isLoading = $state(false);

    async function handleLogin(e) {
        e.preventDefault();
        if (!username || !password) {
            showToast("Harap isi username dan password", "error");
            return;
        }

        isLoading = true;
        try {
            const response = await invoke("login_user", {
                data: { username, password }
            });

            if (response.success && response.data) {
                loginUser(response.data.user, response.data.role, response.data.access_token);
                showToast("Selamat datang kembali!", "success");
            } else {
                showToast(response.message || "Gagal masuk", "error");
            }
        } catch (error) {
            console.error("Login error:", error);
            showToast(error.message || "Username atau password salah", "error");
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-radial from-slate-900 via-slate-950 to-black overflow-hidden font-sans">
    <!-- Abstract gradient glow background blobs -->
    <div class="absolute -top-40 -left-40 w-96 h-96 bg-emerald-500/20 rounded-full blur-3xl animate-pulse"></div>
    <div class="absolute -bottom-40 -right-40 w-96 h-96 bg-teal-500/20 rounded-full blur-3xl animate-pulse delay-700"></div>

    <!-- Login Card Container -->
    <div class="relative w-full max-w-md p-8 mx-4 rounded-3xl bg-white/5 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 transition-all duration-300">
        <!-- Logo Header -->
        <div class="flex flex-col items-center mb-8">
            <div class="flex items-center justify-center w-16 h-16 mb-4 rounded-2xl bg-gradient-to-tr from-emerald-500 to-teal-400 shadow-lg shadow-emerald-500/30">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 text-slate-950" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                </svg>
            </div>
            <h2 class="text-3xl font-extrabold tracking-tight text-white">POS <span class="bg-gradient-to-r from-emerald-400 to-teal-300 bg-clip-text text-transparent">PRO</span></h2>
            <p class="mt-2 text-sm text-gray-400">Masuk untuk mengoperasikan mesin kasir</p>
        </div>

        <!-- Form -->
        <form onsubmit={handleLogin} class="space-y-6">
            <!-- Username Input -->
            <div class="relative">
                <label for="username" class="block text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2">Username</label>
                <div class="relative flex items-center">
                    <span class="absolute left-4 text-gray-500">
                        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                        </svg>
                    </span>
                    <input 
                        type="text" 
                        id="username"
                        bind:value={username}
                        placeholder="Masukkan username Anda"
                        disabled={isLoading}
                        class="w-full py-3.5 pl-12 pr-4 text-white bg-white/5 border border-white/10 rounded-2xl outline-none focus:border-emerald-500 focus:bg-white/10 transition duration-200 text-sm placeholder-gray-600 disabled:opacity-50"
                    />
                </div>
            </div>

            <!-- Password Input -->
            <div class="relative">
                <label for="password" class="block text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2">Password</label>
                <div class="relative flex items-center">
                    <span class="absolute left-4 text-gray-500">
                        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                        </svg>
                    </span>
                    <input 
                        type="password" 
                        id="password"
                        bind:value={password}
                        placeholder="••••••••"
                        disabled={isLoading}
                        class="w-full py-3.5 pl-12 pr-4 text-white bg-white/5 border border-white/10 rounded-2xl outline-none focus:border-emerald-500 focus:bg-white/10 transition duration-200 text-sm placeholder-gray-600 disabled:opacity-50"
                    />
                </div>
            </div>

            <!-- Submit Button -->
            <button 
                type="submit" 
                disabled={isLoading}
                class="w-full py-3.5 flex items-center justify-center font-bold text-slate-950 bg-gradient-to-r from-emerald-400 to-teal-300 hover:from-emerald-350 hover:to-teal-250 rounded-2xl shadow-lg shadow-emerald-500/20 active:scale-98 transition duration-150 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
            >
                {#if isLoading}
                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-slate-950" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Memverifikasi...
                {:else}
                    Masuk ke Sistem
                {/if}
            </button>
        </form>

        <!-- Footer / Seed Info -->
        <div class="mt-8 text-center border-t border-white/5 pt-4">
            <p class="text-xs text-gray-500">Gunakan akun default <b>admin</b> / <b>admin123</b> untuk masuk pertama kali</p>
        </div>
    </div>
</div>

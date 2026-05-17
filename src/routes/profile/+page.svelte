<script>
    import { invoke } from "@tauri-apps/api/core";
    import { authStore } from "$lib/store/auth.js";
    import { showToast } from "$lib/store/toast.js";
    import { onMount } from "svelte";

    let loading = $state(false);
    let saving = $state(false);

    // Dynamic reactive state for form inputs
    let profileData = $state({
        id: null,
        username: "",
        name: "",
        email: "",
        address: "",
        no_handphone: "",
        barcode: null,
        role: "",
        password: "" // Optional for updating
    });

    onMount(() => {
        // Initialize form with logged in user session details
        if ($authStore.user) {
            const u = $authStore.user;
            profileData.id = u.id || null;
            profileData.username = u.username || "";
            profileData.name = u.name || "";
            profileData.email = u.email || "";
            profileData.address = u.address || "";
            profileData.no_handphone = u.no_handphone || "";
            profileData.barcode = u.barcode || null;
            profileData.role = $authStore.role || "kasir";
        }
    });

    /**
     * @param {SubmitEvent} event
     */
    async function handleUpdateProfile(event) {
        event.preventDefault();
        saving = true;

        try {
            // Re-map fields into Tauri User structure
            const payload = {
                id: profileData.id,
                username: profileData.username,
                email: profileData.email ? profileData.email : null,
                name: profileData.name ? profileData.name : null,
                address: profileData.address ? profileData.address : null,
                no_handphone: profileData.no_handphone ? profileData.no_handphone : null,
                barcode: profileData.barcode,
                role: profileData.role,
                // Only pass password if it is not empty, otherwise keep it None in JSON
                password: profileData.password.trim() !== "" ? profileData.password : null
            };

            const updatedUser = await invoke("update_users", {
                data: payload
            });

            // Reactively update local auth session store
            authStore.update(current => {
                return {
                    ...current,
                    user: {
                        ...current.user,
                        name: updatedUser.name,
                        email: updatedUser.email,
                        address: updatedUser.address,
                        no_handphone: updatedUser.no_handphone
                    }
                };
            });

            // Clear password field after success
            profileData.password = "";

            showToast("Profil Anda berhasil diperbarui!", "success");
        } catch (err) {
            console.error("Error updating profile:", err);
            showToast("Gagal memperbarui profil: " + err, "error");
        } finally {
            saving = false;
        }
    }

    /**
     * @param {string} role
     * @returns {string}
     */
    function getRoleLabel(role) {
        switch (role?.toLowerCase()) {
            case "admin": return "Administrator";
            case "kasir": return "Kasir (Cashier)";
            case "gudang": return "Staf Gudang (Warehouse)";
            default: return role || "-";
        }
    }
</script>

<div class="p-6 max-w-4xl mx-auto">
    <!-- Header Page Info -->
    <div class="mb-8">
        <h1 class="text-3xl font-extrabold text-gray-900 tracking-tight">Pengaturan Profil</h1>
        <p class="mt-2 text-sm text-gray-500">Kelola preferensi akun pribadi dan perbarui rincian informasi kontak Anda.</p>
    </div>

    {#if !$authStore.isLoggedIn}
        <div class="bg-red-50 border-l-4 border-red-400 p-4 rounded-r-xl shadow-xs">
            <div class="flex">
                <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z" clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="ml-3">
                    <p class="text-sm font-semibold text-red-800">Sesi Habis</p>
                    <p class="text-xs text-red-700 mt-1">Silakan masuk log kembali untuk melihat dan memperbarui pengaturan preferensi profil Anda.</p>
                </div>
            </div>
        </div>
    {:else}
        <form onsubmit={handleUpdateProfile} class="space-y-6">
            <!-- Glassmorphic Card Container -->
            <div class="bg-white border border-gray-100 rounded-2xl shadow-sm p-6 md:p-8 space-y-8">
                
                <!-- Section 1: Account Info (Read-only) -->
                <div>
                    <h2 class="text-lg font-bold text-gray-800 border-b border-gray-100 pb-3 mb-6">Informasi Akun</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <!-- Username (Disabled) -->
                        <div>
                            <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Username Pengguna (Terkunci)</label>
                            <div class="relative">
                                <span class="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400">
                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                    </svg>
                                </span>
                                <input
                                    type="text"
                                    value={profileData.username}
                                    disabled
                                    class="bg-gray-50 border border-gray-200 text-gray-400 text-sm rounded-xl block w-full pl-10 pr-3.5 py-3 font-semibold select-none cursor-not-allowed outline-none"
                                />
                            </div>
                            <span class="text-[11px] text-gray-400 mt-1.5 block">Nama pengguna akun Anda terikat permanen dan tidak dapat diubah.</span>
                        </div>

                        <!-- Assigned Role (Read-only Badge) -->
                        <div>
                            <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Hak Akses / Peran Kerja</label>
                            <div class="flex items-center h-12">
                                <span class="inline-flex items-center px-4 py-2 rounded-full text-xs font-bold bg-brand/10 text-brand border border-brand/20 uppercase tracking-wider">
                                    <svg class="w-3.5 h-3.5 mr-1.5" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M2.166 4.9L10 1.154l7.834 3.746A1 1 0 0118.5 5.79v7.42a1 1 0 01-.508.874L10 18.846l-7.992-4.762A1 1 0 011.5 13.21V5.79a1 1 0 01.666-.89zM10 3.3l-6 2.87v5.525l6 3.575 6-3.575V6.17l-6-2.87z" clip-rule="evenodd" />
                                    </svg>
                                    {getRoleLabel(profileData.role)}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Section 2: Personal Profile Info (Editable) -->
                <div>
                    <h2 class="text-lg font-bold text-gray-800 border-b border-gray-100 pb-3 mb-6">Detail Profil & Kontak</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <!-- Full Name -->
                        <div class="col-span-2 md:col-span-1">
                            <label class="block text-xs font-semibold text-gray-600 uppercase tracking-wider mb-2">Nama Lengkap</label>
                            <input
                                type="text"
                                bind:value={profileData.name}
                                placeholder="Masukkan nama lengkap Anda"
                                required
                                class="bg-white border border-gray-200 text-gray-800 text-sm rounded-xl focus:ring-brand focus:border-brand block w-full px-3.5 py-3 shadow-xs font-semibold focus:outline-none transition-all"
                            />
                        </div>

                        <!-- Email -->
                        <div class="col-span-2 md:col-span-1">
                            <label class="block text-xs font-semibold text-gray-600 uppercase tracking-wider mb-2">Alamat Email</label>
                            <input
                                type="email"
                                bind:value={profileData.email}
                                placeholder="nama@pospro.com"
                                required
                                class="bg-white border border-gray-200 text-gray-800 text-sm rounded-xl focus:ring-brand focus:border-brand block w-full px-3.5 py-3 shadow-xs font-semibold focus:outline-none transition-all"
                            />
                        </div>

                        <!-- Handphone Number -->
                        <div class="col-span-2 md:col-span-1">
                            <label class="block text-xs font-semibold text-gray-600 uppercase tracking-wider mb-2">Nomor Handphone / WhatsApp</label>
                            <input
                                type="text"
                                bind:value={profileData.no_handphone}
                                placeholder="08xxxxxxxxxx"
                                class="bg-white border border-gray-200 text-gray-800 text-sm rounded-xl focus:ring-brand focus:border-brand block w-full px-3.5 py-3 shadow-xs font-semibold focus:outline-none transition-all"
                            />
                        </div>

                        <!-- Barcode (Read-only display for scanning, if exists) -->
                        <div class="col-span-2 md:col-span-1">
                            <label class="block text-xs font-semibold text-gray-600 uppercase tracking-wider mb-2">Barcode Pengguna</label>
                            <input
                                type="text"
                                value={profileData.barcode || "Belum Dibuat"}
                                disabled
                                class="bg-gray-50 border border-gray-200 text-gray-400 text-sm rounded-xl block w-full px-3.5 py-3 font-semibold select-none cursor-not-allowed outline-none"
                            />
                        </div>

                        <!-- Address -->
                        <div class="col-span-2">
                            <label class="block text-xs font-semibold text-gray-600 uppercase tracking-wider mb-2">Alamat Tempat Tinggal</label>
                            <textarea
                                bind:value={profileData.address}
                                rows="3"
                                placeholder="Tulis alamat rumah lengkap..."
                                class="bg-white border border-gray-200 text-gray-800 text-sm rounded-xl focus:ring-brand focus:border-brand block w-full px-3.5 py-3 shadow-xs font-semibold focus:outline-none transition-all resize-none"
                            ></textarea>
                        </div>
                    </div>
                </div>

                <!-- Section 3: Password Update -->
                <div>
                    <h2 class="text-lg font-bold text-gray-800 border-b border-gray-100 pb-3 mb-6">Ubah Kata Sandi</h2>
                    <div class="max-w-md">
                        <label class="block text-xs font-semibold text-gray-600 uppercase tracking-wider mb-2">Kata Sandi Baru</label>
                        <input
                            type="password"
                            bind:value={profileData.password}
                            placeholder="••••••••"
                            class="bg-white border border-gray-200 text-gray-800 text-sm rounded-xl focus:ring-brand focus:border-brand block w-full px-3.5 py-3 shadow-xs font-semibold focus:outline-none transition-all"
                        />
                        <span class="text-[11px] text-gray-400 mt-2 block">Kosongkan kolom kata sandi di atas jika Anda tidak ingin memperbarui kata sandi saat ini.</span>
                    </div>
                </div>

            </div>

            <!-- Submit Action Bar -->
            <div class="flex items-center justify-end space-x-3 pt-2">
                <button
                    type="submit"
                    disabled={saving}
                    class="inline-flex items-center justify-center px-6 py-3 border border-transparent rounded-xl shadow-sm text-sm font-semibold text-white bg-brand hover:bg-brand-medium focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brand disabled:opacity-50 transition-all cursor-pointer"
                >
                    {#if saving}
                        <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        Menyimpan Perubahan...
                    {:else}
                        Simpan Perubahan
                    {/if}
                </button>
            </div>
        </form>
    {/if}
</div>

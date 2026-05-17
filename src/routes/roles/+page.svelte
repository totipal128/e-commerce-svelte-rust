<script>
    import { rolesStore, roleMenusStore, updateRoleMenus, addNewRole, deleteRole } from "$lib/store/role_menus.js";
    import { showToast } from "$lib/store/toast.js";
    import { authStore } from "$lib/store/auth.js";

    let selectedRole = $state("kasir");

    // Form state for new role
    let showAddModal = $state(false);
    let newRoleName = $state("");
    let newRoleCode = $state("");
    let newRoleDesc = $state("");

    // Complete list of available menu names
    const allMenus = [
        { name: "Dashboard", desc: "Melihat visualisasi laporan, chart penjualan, dan status stok" },
        { name: "Pembelian", desc: "Melakukan transaksi pembelian barang baru dari Supplier" },
        { name: "Penjualan", desc: "Layar kasir utama (touchscreen & scanner barcode)" },
        { name: "Barang", desc: "Manajemen katalog produk, input item, dan kelola harga" },
        { name: "Satuan Barang", desc: "Manajemen data satuan barang (PCS, BOX, LUSIN)" },
        { name: "Kategori Barang", desc: "Manajemen kategori produk untuk klasifikasi barang" },
        { name: "Pengguna", desc: "Kelola akun pengguna, kasir, admin, dan hak masuk" },
        { name: "Peran & Menu", desc: "Mengatur peran pengguna dan perizinan modul menu" },
        { name: "Pelanggan", desc: "Manajemen data pelanggan/konsumen loyal" },
        { name: "Supplier", desc: "Manajemen data supplier & vendor pengirim barang" }
    ];

    // Selected role details
    let currentRoleDetails = $derived(
        $rolesStore.find(r => r.code === selectedRole) || { name: "Kasir", code: "kasir", description: "" }
    );

    // List of active menus for selected role
    let activeMenus = $derived(
        $roleMenusStore[selectedRole] || []
    );

    /**
     * @param {string} menuName
     */
    function toggleMenuPermission(menuName) {
        // Warn if trying to remove Dashboard for cashier/warehouse or Peran & Menu for admin
        if (selectedRole === 'admin' && menuName === 'Peran & Menu') {
            showToast("Admin wajib memiliki akses Peran & Menu!", "error");
            return;
        }

        let updated = [...activeMenus];
        if (updated.includes(menuName)) {
            updated = updated.filter(m => m !== menuName);
        } else {
            updated = [...updated, menuName];
        }

        updateRoleMenus(selectedRole, updated);
        showToast(`Akses menu '${menuName}' diperbarui`, "success");
    }

    /**
     * @param {SubmitEvent} e
     */
    function handleAddRole(e) {
        e.preventDefault();
        if (!newRoleName.trim() || !newRoleCode.trim()) {
            showToast("Nama dan Kode peran wajib diisi!", "error");
            return;
        }

        const formattedCode = newRoleCode.toLowerCase().trim().replace(/\s+/g, '_');
        if ($rolesStore.some(r => r.code === formattedCode)) {
            showToast("Kode peran sudah terdaftar!", "error");
            return;
        }

        addNewRole(newRoleName, formattedCode, newRoleDesc);
        showToast(`Peran '${newRoleName}' berhasil dibuat`, "success");

        // Reset & Close
        newRoleName = "";
        newRoleCode = "";
        newRoleDesc = "";
        showAddModal = false;
        selectedRole = formattedCode;
    }

    /**
     * @param {string} code
     * @param {string} name
     */
    function handleDeleteRole(code, name) {
        if (code === 'admin' || code === 'kasir' || code === 'gudang') {
            showToast("Peran bawaan sistem tidak boleh dihapus!", "error");
            return;
        }

        if (confirm(`Apakah Anda yakin ingin menghapus peran '${name}'? Tindakan ini tidak dapat dibatalkan.`)) {
            deleteRole(code);
            showToast(`Peran '${name}' berhasil dihapus`, "success");
            selectedRole = "kasir";
        }
    }
</script>

<svelte:head>
    <title>Manajemen Peran & Menu | POS PRO</title>
</svelte:head>

<div class="p-6 space-y-6 min-h-full">

    <!-- Header -->
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-2xl font-black text-gray-800">Manajemen Peran & Menu</h1>
            <p class="text-sm text-gray-400 mt-1">Kelola perizinan modul menu dan kustomisasi peran kerja karyawan</p>
        </div>
        <button onclick={() => showAddModal = true} class="flex items-center gap-2 px-5 py-3 bg-brand text-white text-sm font-bold rounded-xl hover:bg-brand-strong transition-all shadow-md shadow-brand/20">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
            </svg>
            Tambah Peran Kustom
        </button>
    </div>

    <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">

        <!-- Left: Roles Directory -->
        <div class="xl:col-span-1 flex flex-col gap-4">
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-100">
                <h2 class="font-black text-gray-800 mb-4 flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                    Daftar Peran Kerja
                </h2>

                <div class="space-y-3">
                    {#each $rolesStore as role}
                        <div
                            onclick={() => selectedRole = role.code}
                            onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { selectedRole = role.code; } }}
                            role="button"
                            tabindex="0"
                            class="w-full text-left p-4 rounded-2xl border transition-all relative group flex flex-col gap-1 cursor-pointer select-none focus:outline-none focus:ring-2 focus:ring-brand/20 {selectedRole === role.code ? 'border-brand bg-brand/5 shadow-sm shadow-brand/5' : 'border-gray-100 hover:border-gray-200 bg-white'}"
                        >
                            <div class="flex items-center justify-between">
                                <span class="font-bold text-sm {selectedRole === role.code ? 'text-brand' : 'text-gray-800'}">{role.name}</span>
                                <span class="text-[10px] font-black tracking-wider uppercase px-2 py-0.5 rounded-full {role.code === 'admin' ? 'bg-red-50 text-red-600' : role.code === 'kasir' ? 'bg-blue-50 text-blue-600' : role.code === 'gudang' ? 'bg-orange-50 text-orange-600' : 'bg-gray-100 text-gray-600'}">
                                    {role.code}
                                </span>
                            </div>
                            <p class="text-xs text-gray-400 mt-1 pr-6 leading-relaxed">{role.description}</p>

                            <!-- Delete button for custom roles -->
                            {#if role.code !== 'admin' && role.code !== 'kasir' && role.code !== 'gudang'}
                                <button
                                    onclick={(e) => { e.stopPropagation(); handleDeleteRole(role.code, role.name); }}
                                    class="absolute right-3 bottom-3 p-1.5 rounded-lg text-gray-300 hover:text-red-600 hover:bg-red-50 opacity-0 group-hover:opacity-100 transition-all"
                                    title="Hapus peran"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                    </svg>
                                </button>
                            {/if}
                        </div>
                    {/each}
                </div>
            </div>

            <!-- Role Summary Card -->
            <div class="bg-gradient-to-br from-slate-900 to-slate-800 rounded-2xl p-6 text-white shadow-lg relative overflow-hidden">
                <div class="absolute -right-4 -top-4 w-20 h-20 bg-white/5 rounded-full"></div>
                <div class="relative">
                    <h3 class="font-bold text-sm text-slate-300 mb-2">💡 Tips Keamanan</h3>
                    <p class="text-xs text-slate-400 leading-relaxed">
                        Membatasi akses modul menu (RBAC) sangat penting untuk menjaga integritas data transaksi kasir dan logistik gudang. Selalu berikan akses minimum yang diperlukan oleh masing-masing staff.
                    </p>
                </div>
            </div>
        </div>

        <!-- Right: Menu Permissions -->
        <div class="xl:col-span-2 bg-white rounded-2xl p-6 shadow-sm border border-gray-100 flex flex-col gap-6">
            
            <!-- Selected Role Header Info -->
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-4 border-b border-gray-100">
                <div>
                    <span class="text-xs font-semibold text-brand tracking-wider uppercase">Pengaturan Akses</span>
                    <h2 class="text-xl font-black text-gray-800 mt-1">{currentRoleDetails.name}</h2>
                    <p class="text-xs text-gray-400 mt-1 font-medium">{currentRoleDetails.description || "Peran kustom dalam sistem POS PRO."}</p>
                </div>
                <div class="px-4 py-2.5 bg-emerald-50 text-emerald-700 rounded-xl text-xs font-bold shrink-0 self-start md:self-center">
                    {activeMenus.length} dari {allMenus.length} Modul Aktif
                </div>
            </div>

            <!-- Module Permissions Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                {#each allMenus as menu}
                    {@const isPermitted = activeMenus.includes(menu.name)}
                    <button
                        onclick={() => toggleMenuPermission(menu.name)}
                        class="w-full text-left p-4 rounded-2xl border transition-all flex items-start gap-4 hover:bg-gray-50/50 {isPermitted ? 'border-emerald-200 bg-emerald-50/5' : 'border-gray-100 bg-white'}"
                    >
                        <!-- custom checkbox toggle -->
                        <div class="h-6 w-6 rounded-lg border-2 flex items-center justify-center shrink-0 mt-0.5 transition-all {isPermitted ? 'border-emerald-500 bg-emerald-500 text-white' : 'border-gray-200 bg-white'}" >
                            {#if isPermitted}
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                </svg>
                            {/if}
                        </div>
                        
                        <div>
                            <p class="text-sm font-bold {isPermitted ? 'text-gray-800' : 'text-gray-600'}">{menu.name}</p>
                            <p class="text-xs text-gray-400 mt-1 leading-relaxed">{menu.desc}</p>
                        </div>
                    </button>
                {/each}
            </div>

            <!-- Visual Preview Card -->
            <div class="bg-gray-50 rounded-2xl p-5 border border-gray-100 mt-2">
                <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-3">Live Sidebar Preview</p>
                <div class="flex flex-wrap gap-2">
                    {#each allMenus as menu}
                        {@const isPermitted = activeMenus.includes(menu.name)}
                        {#if isPermitted}
                            <span class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-white border border-gray-200 rounded-full text-xs font-semibold text-gray-700 shadow-sm animate-fade-in">
                                <span class="h-1.5 w-1.5 bg-emerald-500 rounded-full"></span>
                                {menu.name}
                            </span>
                        {:else}
                            <span class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-gray-100/50 border border-gray-100 rounded-full text-xs font-medium text-gray-300 select-none">
                                {menu.name}
                            </span>
                        {/if}
                    {/each}
                </div>
            </div>

        </div>
    </div>
</div>

<!-- Add Custom Role Modal -->
{#if showAddModal}
    <div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-sm" onclick={() => showAddModal = false}>
        <div class="relative bg-white border border-gray-100 rounded-2xl shadow-2xl p-6 w-96 md:w-[500px] overflow-y-auto max-h-[90vh]" onclick={(e) => e.stopPropagation()}>
            
            <button onclick={() => showAddModal = false} class="absolute top-4 right-4 text-gray-400 hover:text-gray-600 hover:bg-gray-50 rounded-xl w-9 h-9 flex items-center justify-center font-bold text-sm">
                ✕
            </button>

            <h2 class="text-xl font-black text-gray-800 mb-1 flex items-center gap-2">
                Tambah Peran Kustom
            </h2>
            <p class="text-xs text-gray-400 mb-6">Tambahkan divisi atau jabatan karyawan baru ke sistem POS PRO.</p>

            <form onsubmit={handleAddRole} class="space-y-4">
                <div>
                    <label class="block mb-2 text-xs font-bold text-gray-500 uppercase tracking-wider" for="role_name">Nama Peran Kerja</label>
                    <input bind:value={newRoleName} id="role_name" type="text" class="block w-full p-3 bg-gray-50 border border-gray-200 text-sm rounded-xl focus:outline-none focus:border-brand font-medium placeholder:text-gray-300" placeholder="Contoh: Supervisor Toko" required/>
                </div>

                <div>
                    <label class="block mb-2 text-xs font-bold text-gray-500 uppercase tracking-wider" for="role_code">Kode Peran (Unique Key)</label>
                    <input bind:value={newRoleCode} id="role_code" type="text" class="block w-full p-3 bg-gray-50 border border-gray-200 text-sm rounded-xl focus:outline-none focus:border-brand font-medium placeholder:text-gray-300" placeholder="Contoh: supervisor" required/>
                    <p class="text-[10px] text-gray-400 mt-1 leading-normal">Kode ini akan secara otomatis diformat menjadi huruf kecil dengan garis bawah (snake_case).</p>
                </div>

                <div>
                    <label class="block mb-2 text-xs font-bold text-gray-500 uppercase tracking-wider" for="role_desc">Deskripsi Peran</label>
                    <textarea bind:value={newRoleDesc} id="role_desc" rows="3" class="block w-full p-3 bg-gray-50 border border-gray-200 text-sm rounded-xl focus:outline-none focus:border-brand font-medium placeholder:text-gray-300" placeholder="Tuliskan wewenang atau tanggung jawab peran ini..."></textarea>
                </div>

                <div class="flex gap-3 justify-end pt-4">
                    <button type="button" onclick={() => showAddModal = false} class="px-5 py-3 border border-gray-200 rounded-xl text-sm font-bold text-gray-500 hover:bg-gray-50 transition-colors w-24">
                        Batal
                    </button>
                    <button type="submit" class="px-5 py-3 bg-brand text-white text-sm font-bold rounded-xl hover:bg-brand-strong transition-colors shadow-md shadow-brand/20 w-32">
                        Buat Peran
                    </button>
                </div>
            </form>
        </div>
    </div>
{/if}

<style>
    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(4px); }
        to { opacity: 1; transform: translateY(0); }
    }
    .animate-fade-in {
        animation: fadeIn 0.2s ease-out forwards;
    }
</style>

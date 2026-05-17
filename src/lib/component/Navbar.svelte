<script>
    import { authStore, logoutUser } from "$lib/store/auth.js";
    import { showToast } from "$lib/store/toast.js";

    let { user = "Administrator" } = $props();

    let showNotifications = $state(false);
    let showChat = $state(false);
    let showProfileDropdown = $state(false);

    let unreadNotif = $state(2);
    let unreadChat = $state(3);

    function toggleProfile() {
        showNotifications = false;
        showChat = false;
        showProfileDropdown = !showProfileDropdown;
    }

    function handleLogout() {
        logoutUser();
        showToast("Anda telah keluar", "success");
    }

    let notifications = $state([
        { id: 1, icon: "⚠️", title: "Stok Menipis", desc: "Barang 'Susu Kaleng' tersisa 5 pcs", time: "2 mnt lalu", read: false },
        { id: 2, icon: "✅", title: "Penjualan Berhasil", desc: "Transaksi TRX-20240426-1 tersimpan", time: "10 mnt lalu", read: false },
        { id: 3, icon: "📦", title: "Pembelian Baru", desc: "PUR00012 telah masuk ke sistem", time: "1 jam lalu", read: true },
        { id: 4, icon: "👤", title: "Pelanggan Baru", desc: "Budi Santoso telah terdaftar", time: "3 jam lalu", read: true },
    ]);

    let messages = $state([
        { id: 1, name: "Budi S.", avatar: "B", text: "Harga barang sudah diupdate?", time: "5 mnt lalu", unread: true },
        { id: 2, name: "Siti R.", avatar: "S", text: "Tolong cek stok karung beras", time: "12 mnt lalu", unread: true },
        { id: 3, name: "Admin", avatar: "A", text: "Laporan bulan ini sudah siap", time: "1 jam lalu", unread: true },
        { id: 4, name: "Supplier X", avatar: "X", text: "Pengiriman akan tiba besok pagi", time: "2 jam lalu", unread: false },
    ]);

    let newMessage = $state("");

    function markAllRead() {
        notifications = notifications.map(n => ({ ...n, read: true }));
        unreadNotif = 0;
    }

    function markChatRead() {
        messages = messages.map(m => ({ ...m, unread: false }));
        unreadChat = 0;
    }

    function closeAll() {
        showNotifications = false;
        showChat = false;
        showProfileDropdown = false;
    }

    function toggleNotif() {
        showChat = false;
        showProfileDropdown = false;
        showNotifications = !showNotifications;
        if (showNotifications) markAllRead();
    }

    function toggleChat() {
        showNotifications = false;
        showProfileDropdown = false;
        showChat = !showChat;
        if (showChat) markChatRead();
    }
</script>

<!-- Overlay to close modals -->
{#if showNotifications || showChat || showProfileDropdown}
    <div class="fixed inset-0 z-[49]" onclick={closeAll}></div>
{/if}

<nav class="bg-white/90 backdrop-blur-md h-16 shadow-sm border-b border-gray-100 sticky top-0 z-[50] flex items-center justify-between px-6">

    <!-- Left: Brand -->
    <div class="flex items-center gap-3">
        <div class="bg-brand p-2 rounded-xl shadow-lg shadow-brand/20">
            <img src="/tauri.svg" class="h-5 w-5 invert" alt="Logo"/>
        </div>
        <span class="font-black text-xl tracking-tight text-gray-800">POS <span class="text-brand">PRO</span></span>
    </div>

    <!-- Right: Actions -->
    <div class="flex items-center gap-2">

        <!-- Chat Button -->
        <div class="relative">
            <button
                onclick={toggleChat}
                class="relative p-2.5 text-gray-500 hover:text-brand hover:bg-brand/5 rounded-xl transition-all duration-200 group"
                title="Pesan"
            >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 group-hover:scale-110 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                </svg>
                {#if unreadChat > 0}
                    <span class="absolute -top-0.5 -right-0.5 h-5 w-5 bg-red-500 text-white text-[10px] font-black rounded-full flex items-center justify-center border-2 border-white shadow-lg animate-pulse">{unreadChat}</span>
                {/if}
            </button>

            <!-- Chat Modal -->
            {#if showChat}
                <div class="absolute top-12 right-0 w-80 bg-white rounded-2xl shadow-2xl border border-gray-100 overflow-hidden z-[51]" style="animation: slideDown 0.2s ease-out;">
                    <div class="p-4 bg-gradient-to-r from-brand to-brand-strong text-white flex justify-between items-center">
                        <div>
                            <p class="font-bold text-sm">Pesan Masuk</p>
                            <p class="text-xs text-white/70">{messages.length} percakapan</p>
                        </div>
                        <button class="text-xs bg-white/20 hover:bg-white/30 px-3 py-1 rounded-full font-bold transition-all">+ Baru</button>
                    </div>
                    <div class="max-h-80 overflow-y-auto divide-y divide-gray-50">
                        {#each messages as msg}
                            <button class="w-full flex items-center gap-3 p-3 hover:bg-gray-50 transition-colors text-left">
                                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-purple-400 to-pink-500 flex items-center justify-center text-white font-bold text-sm shrink-0">
                                    {msg.avatar}
                                </div>
                                <div class="flex-1 min-w-0">
                                    <div class="flex justify-between items-baseline">
                                        <p class="text-sm font-bold text-gray-800 truncate">{msg.name}</p>
                                        <p class="text-[10px] text-gray-400 shrink-0 ml-2">{msg.time}</p>
                                    </div>
                                    <p class="text-xs text-gray-500 truncate">{msg.text}</p>
                                </div>
                                {#if msg.unread}
                                    <span class="h-2.5 w-2.5 bg-brand rounded-full shrink-0"></span>
                                {/if}
                            </button>
                        {/each}
                    </div>
                    <div class="p-3 border-t bg-gray-50">
                        <div class="flex gap-2">
                            <input
                                bind:value={newMessage}
                                placeholder="Ketik pesan..."
                                class="flex-1 text-xs px-3 py-2 bg-white border border-gray-200 rounded-xl focus:outline-none focus:border-brand"
                            />
                            <button class="bg-brand text-white px-3 py-2 rounded-xl text-xs font-bold hover:bg-brand-strong transition-colors">Kirim</button>
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        <!-- Notification Button -->
        <div class="relative">
            <button
                onclick={toggleNotif}
                class="relative p-2.5 text-gray-500 hover:text-brand hover:bg-brand/5 rounded-xl transition-all duration-200 group"
                title="Notifikasi"
            >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 group-hover:scale-110 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                </svg>
                {#if unreadNotif > 0}
                    <span class="absolute -top-0.5 -right-0.5 h-5 w-5 bg-red-500 text-white text-[10px] font-black rounded-full flex items-center justify-center border-2 border-white shadow-lg">{unreadNotif}</span>
                {/if}
            </button>

            <!-- Notification Modal -->
            {#if showNotifications}
                <div class="absolute top-12 right-0 w-80 bg-white rounded-2xl shadow-2xl border border-gray-100 overflow-hidden z-[51]" style="animation: slideDown 0.2s ease-out;">
                    <div class="p-4 bg-gradient-to-r from-orange-500 to-red-500 text-white flex justify-between items-center">
                        <div>
                            <p class="font-bold text-sm">Notifikasi</p>
                            <p class="text-xs text-white/70">Semua aktivitas terbaru</p>
                        </div>
                        <button onclick={markAllRead} class="text-xs bg-white/20 hover:bg-white/30 px-3 py-1 rounded-full font-bold transition-all">Baca Semua</button>
                    </div>
                    <div class="max-h-80 overflow-y-auto divide-y divide-gray-50">
                        {#each notifications as notif}
                            <button class="w-full flex items-start gap-3 p-3 hover:bg-gray-50 transition-colors text-left {notif.read ? 'opacity-60' : ''}">
                                <span class="text-xl mt-0.5">{notif.icon}</span>
                                <div class="flex-1 min-w-0">
                                    <div class="flex justify-between items-baseline">
                                        <p class="text-sm font-bold text-gray-800">{notif.title}</p>
                                        <p class="text-[10px] text-gray-400 shrink-0 ml-2">{notif.time}</p>
                                    </div>
                                    <p class="text-xs text-gray-500 mt-0.5">{notif.desc}</p>
                                </div>
                                {#if !notif.read}
                                    <span class="h-2.5 w-2.5 bg-orange-500 rounded-full shrink-0 mt-1"></span>
                                {/if}
                            </button>
                        {/each}
                    </div>
                    <div class="p-3 border-t bg-gray-50 text-center">
                        <button class="text-xs text-brand font-bold hover:underline">Lihat Semua Notifikasi</button>
                    </div>
                </div>
            {/if}
        </div>

        <!-- Divider -->
        <div class="h-8 w-[1px] bg-gray-100 mx-2"></div>

        <!-- Profile -->
        <div class="relative">
            <div class="flex items-center gap-3 cursor-pointer select-none" onclick={toggleProfile}>
                <div class="text-right hidden md:block">
                    <p class="text-[11px] font-medium text-gray-400 leading-none mb-1">Masuk sebagai</p>
                    <p class="text-sm font-bold text-gray-800 leading-none">{user}</p>
                </div>
                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-emerald-500 to-teal-400 border-2 border-white shadow-lg flex items-center justify-center text-slate-950 font-black text-base hover:scale-105 transition-transform">
                    {user.charAt(0).toUpperCase()}
                </div>
            </div>

            {#if showProfileDropdown}
                <div class="absolute top-12 right-0 w-48 bg-white rounded-2xl shadow-2xl border border-gray-100 py-2 z-[51]" style="animation: slideDown 0.2s ease-out;">
                    <div class="px-4 py-2 border-b border-gray-50">
                        <p class="text-xs text-gray-400 font-medium">Peran Anda</p>
                        <p class="text-sm font-bold text-emerald-600 capitalize">{$authStore.role || "Kasir"}</p>
                    </div>
                    
                    <a href="/profile" onclick={closeAll} class="w-full flex items-center gap-2 px-4 py-3 text-gray-700 hover:bg-gray-50 hover:text-gray-900 font-semibold text-xs transition-colors border-none bg-transparent">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-gray-500 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                        Pengaturan Profil
                    </a>

                    <button onclick={handleLogout} class="w-full flex items-center gap-2 px-4 py-3 text-red-600 hover:bg-red-50 hover:text-red-700 font-semibold text-xs text-left transition-colors cursor-pointer border-none bg-transparent">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                        </svg>
                        Keluar dari Sistem
                    </button>
                </div>
            {/if}
        </div>
    </div>
</nav>

<style>
    @keyframes slideDown {
        from { opacity: 0; transform: translateY(-8px) scale(0.97); }
        to   { opacity: 1; transform: translateY(0) scale(1); }
    }
</style>

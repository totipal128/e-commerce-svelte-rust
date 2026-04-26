import { writable } from 'svelte/store';

export const toastStore = writable({ message: "", type: "error", show: false });

export function showToast(message, type = "error") {
    toastStore.set({ message, type, show: true });
    setTimeout(() => {
        toastStore.update(t => {
            t.show = false;
            return t;
        });
    }, 3000);
}

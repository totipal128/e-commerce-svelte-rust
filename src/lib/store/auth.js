import { writable } from 'svelte/store';

/**
 * @typedef {Object} UserSession
 * @property {boolean} isLoggedIn
 * @property {any} user
 * @property {string|null} role
 * @property {string|null} accessToken
 */

/** @type {UserSession} */
const initialAuth = {
    isLoggedIn: false,
    user: null,
    role: null,
    accessToken: null
};

// Load initial state from localStorage if available
let savedAuth = initialAuth;
if (typeof window !== 'undefined') {
    try {
        const stored = localStorage.getItem('pos_pro_auth');
        if (stored) {
            savedAuth = JSON.parse(stored);
        }
    } catch (e) {
        console.error("Error reading auth from localStorage:", e);
    }
}

export const authStore = writable(savedAuth);

// Subscribe to sync with localStorage
authStore.subscribe(auth => {
    if (typeof window !== 'undefined') {
        try {
            localStorage.setItem('pos_pro_auth', JSON.stringify(auth));
        } catch (e) {
            console.error("Error saving auth to localStorage:", e);
        }
    }
});

/**
 * @param {any} user
 * @param {string} role
 * @param {string} accessToken
 */
export function loginUser(user, role, accessToken) {
    authStore.set({
        isLoggedIn: true,
        user,
        role,
        accessToken
    });
}

export function logoutUser() {
    authStore.set(initialAuth);
}
